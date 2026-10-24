//! Debtors: the clients of a Business that owe it money, each with the one
//! Contact who receives reminders (R2). Every query is scoped by the
//! Business (A2).

use std::sync::Arc;

use sqlx::PgPool;

use super::businesses::BusinessId;
use crate::engine::accounts::is_email_address;
use crate::engine::clock::Clock;
use crate::engine::ids::id_type;

id_type!(
    /// One Debtor of a Business.
    DebtorId
);

/// A Debtor as typed into a form or read from a spreadsheet row.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DebtorInput {
    /// The client company's name.
    pub business_name: String,
    /// The Contact's name, used in the greeting.
    pub contact_name: String,
    /// The Contact's email address, where reminders go.
    pub contact_email: String,
    /// The Contact's mobile number for "copy and open"; may be blank.
    pub contact_mobile: String,
    /// `taglish` or `english`.
    pub language: String,
}

/// A Debtor as stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Debtor {
    /// Its ID.
    pub id: DebtorId,
    /// The client company's name.
    pub business_name: String,
    /// The Contact's name.
    pub contact_name: String,
    /// The Contact's email address.
    pub contact_email: String,
    /// The Contact's mobile number, if given.
    pub contact_mobile: Option<String>,
    /// `taglish` or `english`.
    pub language: String,
    /// Where its reminders stand, e.g. `active` or `paused`.
    pub state: String,
    /// Until when a Promise holds, if one does.
    pub promised_until: Option<time::Date>,
}

/// A Debtor in the list, with what it owes counted, never summed (H2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebtorSummary {
    /// Its ID.
    pub id: DebtorId,
    /// The client company's name.
    pub business_name: String,
    /// The Contact's name.
    pub contact_name: String,
    /// Where its reminders stand, e.g. `active` or `paused`.
    pub state: String,
    /// Until when a Promise holds, if one does.
    pub promised_until: Option<time::Date>,
    /// How many of its Debts are open.
    pub open_debts: i64,
    /// The earliest due date among its open Debts.
    pub oldest_due: Option<time::Date>,
}

/// Debtor errors.
#[derive(Debug, thiserror::Error)]
pub enum DebtorError {
    /// Every problem with the input, for the form to show.
    #[error("{}", .0.join("; "))]
    Invalid(Vec<String>),
    /// No such Debtor in this Business.
    #[error("debtor not found")]
    NotFound,
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// The Debtors of each Business.
#[derive(Clone)]
pub struct Debtors {
    pool: PgPool,
    clock: Arc<dyn Clock>,
}

impl Debtors {
    /// Debtors in `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self { pool, clock }
    }

    /// Adds a Debtor to `business`.
    pub async fn create(&self, business: BusinessId, input: &DebtorInput) -> Result<DebtorId, DebtorError> {
        let input = check(input)?;
        let id = DebtorId::new();
        let now = self.clock.now();
        sqlx::query!(
            "insert into debtors
                (id, tenant_id, business_name, contact_name, contact_email, contact_mobile, language,
                 created_at, updated_at)
             values ($1, $2, $3, $4, $5, $6, $7, $8, $8)",
            id.as_uuid(),
            business.as_uuid(),
            input.business_name,
            input.contact_name,
            input.contact_email,
            input.contact_mobile,
            input.language,
            now,
        )
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    /// Every Debtor of `business`, by name.
    pub async fn list(&self, business: BusinessId) -> Result<Vec<DebtorSummary>, DebtorError> {
        let rows = sqlx::query!(
            r#"select d.id, d.business_name, d.contact_name, d.state, d.promised_until,
                      count(t.id) filter (where t.status = 'open') as "open_debts!",
                      min(t.due_date) filter (where t.status = 'open') as oldest_due
               from debtors d
               left join debts t on t.tenant_id = d.tenant_id and t.debtor_id = d.id
               where d.tenant_id = $1
               group by d.id
               order by lower(d.business_name), d.id"#,
            business.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| DebtorSummary {
                id: row.id.into(),
                business_name: row.business_name,
                contact_name: row.contact_name,
                state: row.state,
                promised_until: row.promised_until,
                open_debts: row.open_debts,
                oldest_due: row.oldest_due,
            })
            .collect())
    }

    /// One of `business`'s Debtors.
    pub async fn get(&self, business: BusinessId, id: DebtorId) -> Result<Debtor, DebtorError> {
        let row = sqlx::query!(
            "select business_name, contact_name, contact_email, contact_mobile, language, state, promised_until
             from debtors where tenant_id = $1 and id = $2",
            business.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DebtorError::NotFound)?;
        Ok(Debtor {
            id,
            business_name: row.business_name,
            contact_name: row.contact_name,
            contact_email: row.contact_email,
            contact_mobile: row.contact_mobile,
            language: row.language,
            state: row.state,
            promised_until: row.promised_until,
        })
    }
}

/// A Debtor's fields, trimmed and checked.
struct Checked {
    business_name: String,
    contact_name: String,
    contact_email: String,
    contact_mobile: Option<String>,
    language: String,
}

fn check(input: &DebtorInput) -> Result<Checked, DebtorError> {
    let mut problems = Vec::new();
    let business_name = input.business_name.trim();
    if !is_short_text(business_name, 100) {
        problems.push("Business name: 1 to 100 characters".to_owned());
    }
    let contact_name = input.contact_name.trim();
    if !is_short_text(contact_name, 60) {
        problems.push("Contact name: 1 to 60 characters".to_owned());
    }
    let contact_email = input.contact_email.trim();
    if !is_email_address(contact_email) {
        problems.push(format!("Email: {contact_email:?} is not an email address"));
    }
    let contact_mobile = input.contact_mobile.trim();
    if !contact_mobile.is_empty() && !is_mobile_number(contact_mobile) {
        problems.push(format!("Mobile: {contact_mobile:?} is not a mobile number"));
    }
    let language = input.language.trim().to_lowercase();
    if !matches!(language.as_str(), "taglish" | "english") {
        problems.push("Language: Taglish or English".to_owned());
    }
    if !problems.is_empty() {
        return Err(DebtorError::Invalid(problems));
    }
    Ok(Checked {
        business_name: business_name.to_owned(),
        contact_name: contact_name.to_owned(),
        contact_email: contact_email.to_owned(),
        contact_mobile: (!contact_mobile.is_empty()).then(|| contact_mobile.to_owned()),
        language,
    })
}

fn is_short_text(text: &str, max: usize) -> bool {
    !text.is_empty() && text.chars().count() <= max && !text.chars().any(char::is_control)
}

/// Digits with an optional leading `+`, and spaces or dashes between: 10 to
/// 15 digits, as in `0917 123 4567` or `+63 917 123 4567`.
fn is_mobile_number(text: &str) -> bool {
    let body = text.strip_prefix('+').unwrap_or(text);
    let digits = body.chars().filter(char::is_ascii_digit).count();
    body.chars().all(|c| c.is_ascii_digit() || c == ' ' || c == '-') && (10..=15).contains(&digits)
}
