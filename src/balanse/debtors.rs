//! Debtors: the clients of a Business that owe it money, each with the one
//! Contact who receives reminders (R2). Every query is scoped by the
//! Business (A2).

use std::sync::Arc;

use sqlx::PgPool;

use serde_json::json;

use super::businesses::BusinessId;
use super::reminders::redraft_waiting;
use crate::engine::accounts::is_email_address;
use crate::engine::audit::{self, AuditEvent};
use crate::engine::clock::Clock;
use crate::engine::ids::id_type;
use crate::engine::outbox::{Outbox, OutboxError};

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
    /// How many of its reminders wait for the Owner's approval.
    pub drafts_waiting: i64,
    /// The last message approved or sent, if any; never a discarded,
    /// expired or blocked one.
    pub last_message: Option<LastMessage>,
}

/// A Debtor's last message, sent by Balanse or logged as sent by hand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LastMessage {
    /// The outbox status (`approved`, `sent`, `failed`…) or `by_hand`.
    pub status: String,
    /// When it was sent, approved or logged.
    pub at: time::OffsetDateTime,
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
    /// Redrafting the Debtor's waiting reminders failed.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
}

/// The Debtors of each Business.
#[derive(Clone)]
pub struct Debtors {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    outbox: Outbox,
}

impl Debtors {
    /// Debtors in `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self {
            outbox: Outbox::new(pool.clone(), clock.clone()),
            pool,
            clock,
        }
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

    /// Changes one of `business`'s Debtors and its Contact. A new address
    /// ends a stop after a bounce (F4), and waiting reminders are drafted
    /// again for it. `by` names who did it in the audit log.
    pub async fn update(
        &self,
        business: BusinessId,
        id: DebtorId,
        input: &DebtorInput,
        by: &str,
    ) -> Result<(), DebtorError> {
        let input = check(input)?;
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        let before = sqlx::query_scalar!(
            "select contact_email from debtors where tenant_id = $1 and id = $2 for update",
            business.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(DebtorError::NotFound)?;
        let new_address = !before.trim().eq_ignore_ascii_case(&input.contact_email);
        sqlx::query!(
            "update debtors
             set business_name = $3, contact_name = $4, contact_email = $5, contact_mobile = $6, language = $7,
                 state = case when $8 and state = 'bounced' then 'active' else state end,
                 updated_at = $9
             where tenant_id = $1 and id = $2",
            business.as_uuid(),
            id.as_uuid(),
            input.business_name,
            input.contact_name,
            input.contact_email,
            input.contact_mobile,
            input.language,
            new_address,
            now,
        )
        .execute(&mut *tx)
        .await?;
        audit::record(
            &mut *tx,
            &AuditEvent {
                tenant: business,
                at: now,
                actor: by,
                action: "debtor.changed",
                subject_type: "debtor",
                subject_id: id.as_uuid(),
                details: json!({ "new_address": new_address }),
            },
        )
        .await?;
        tx.commit().await?;
        if new_address {
            redraft_waiting(&self.outbox, &self.pool, business, id, by).await?;
        }
        Ok(())
    }

    /// Every Debtor of `business`, in the dashboard's order (H1): those
    /// needing action first, then active ones, then the rest, each group
    /// oldest overdue first.
    pub async fn list(&self, business: BusinessId) -> Result<Vec<DebtorSummary>, DebtorError> {
        let rows = sqlx::query!(
            r#"with summary as (
                   select d.id, d.business_name, d.contact_name, d.state, d.promised_until,
                          (select count(*) from debts t
                           where t.tenant_id = d.tenant_id and t.debtor_id = d.id
                             and t.status in ('open', 'disputed')) as open_debts,
                          (select min(t.due_date) from debts t
                           where t.tenant_id = d.tenant_id and t.debtor_id = d.id
                             and t.status in ('open', 'disputed')) as oldest_due,
                          (select count(*) from reminders r
                           join outbox_messages m on m.tenant_id = r.tenant_id and m.id = r.message_id
                           where r.tenant_id = d.tenant_id and r.debtor_id = d.id
                             and m.status = 'draft') as drafts_waiting,
                          last.status as last_status, last.at as last_at
                   from debtors d
                   left join lateral (
                       select m.status, coalesce(m.sent_at, m.approved_at, m.created_at) as at
                       from reminders r
                       join outbox_messages m on m.tenant_id = r.tenant_id and m.id = r.message_id
                       where r.tenant_id = d.tenant_id and r.debtor_id = d.id
                         and m.status in ('approved', 'sending', 'sent', 'failed', 'unconfirmed')
                       union all
                       select 'by_hand', s.sent_at from manual_sends s
                       where s.tenant_id = d.tenant_id and s.debtor_id = d.id
                       order by at desc
                       limit 1
                   ) last on true
                   where d.tenant_id = $1
               )
               select id as "id!", business_name as "business_name!", contact_name as "contact_name!",
                      state as "state!", promised_until, open_debts as "open_debts!", oldest_due,
                      drafts_waiting as "drafts_waiting!", last_status, last_at
               from summary
               order by case
                            when state in ('paused', 'payment_claimed', 'needs_call', 'disputed', 'bounced')
                              or drafts_waiting > 0 then 0
                            when state = 'active' and open_debts > 0 then 1
                            else 2
                        end,
                        oldest_due nulls last, lower(business_name), id"#,
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
                drafts_waiting: row.drafts_waiting,
                last_message: row
                    .last_status
                    .zip(row.last_at)
                    .map(|(status, at)| LastMessage { status, at }),
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

/// Every problem with `input`, for a form or an import row to show (B1).
pub fn problems(input: &DebtorInput) -> Vec<String> {
    match check(input) {
        Err(DebtorError::Invalid(problems)) => problems,
        _ => Vec::new(),
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
