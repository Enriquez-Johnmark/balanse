//! Debts: invoices a Business already issued and hasn't been paid for.
//! Each carries the Business's own invoice number and date (R11). Amounts
//! are kept exactly as typed (B3, R10); Balanse never computes interest,
//! penalties or totals across Debtors. Every query is scoped by the
//! Business (A2).

use std::sync::Arc;

use serde_json::json;
use sqlx::PgPool;
use time::Date;
use time::macros::format_description;

use super::businesses::BusinessId;
use super::debtors::DebtorId;
use super::reminders::redraft_waiting;
use crate::engine::audit::{self, AuditEvent};
use crate::engine::clock::Clock;
use crate::engine::ids::id_type;
use crate::engine::outbox::{Outbox, OutboxError};

id_type!(
    /// One Debt of a Business.
    DebtId
);

/// The largest amount accepted, in centavos: ₱999,999,999.99.
const MAX_CENTAVOS: i64 = 99_999_999_999;

/// A Debt as typed into a form or read from a spreadsheet row.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DebtInput {
    /// The Business's own invoice number.
    pub invoice_number: String,
    /// The invoice's date, `YYYY-MM-DD`.
    pub invoice_date: String,
    /// The amount owed, as the Business writes it, e.g. `₱12,500.00`.
    pub amount: String,
    /// When it was due, `YYYY-MM-DD`.
    pub due_date: String,
    /// A private note; may be blank. Never sent to the Debtor or the model.
    pub note: String,
}

/// Where a Debt stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebtStatus {
    /// Owed, and reminded about.
    Open,
    /// The Debtor says it paid; waiting for the Owner to confirm.
    PaymentClaimed,
    /// Paid, as the Owner confirmed.
    Paid,
    /// The Debtor says it isn't owed.
    Disputed,
    /// The Owner gave up on it.
    WrittenOff,
    /// Entered by mistake.
    Cancelled,
}

impl DebtStatus {
    /// The stored name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::PaymentClaimed => "payment_claimed",
            Self::Paid => "paid",
            Self::Disputed => "disputed",
            Self::WrittenOff => "written_off",
            Self::Cancelled => "cancelled",
        }
    }

    fn parse(text: &str) -> Self {
        match text {
            "payment_claimed" => Self::PaymentClaimed,
            "paid" => Self::Paid,
            "disputed" => Self::Disputed,
            "written_off" => Self::WrittenOff,
            "cancelled" => Self::Cancelled,
            _ => Self::Open,
        }
    }
}

/// A Debt as stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Debt {
    /// Its ID.
    pub id: DebtId,
    /// Who owes it.
    pub debtor: DebtorId,
    /// The Business's invoice number.
    pub invoice_number: String,
    /// The invoice's date.
    pub invoice_date: Date,
    /// The amount exactly as typed.
    pub amount: String,
    /// When it was due.
    pub due_date: Date,
    /// The private note, if any.
    pub note: Option<String>,
    /// Where it stands.
    pub status: DebtStatus,
}

/// Debt errors.
#[derive(Debug, thiserror::Error)]
pub enum DebtError {
    /// Every problem with the input, for the form to show.
    #[error("{}", .0.join("; "))]
    Invalid(Vec<String>),
    /// No such Debt (or Debtor) in this Business.
    #[error("debt not found")]
    NotFound,
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    /// Cancelling a reminder that no longer matched failed.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
}

/// The Debts of each Business.
#[derive(Clone)]
pub struct Debts {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    outbox: Outbox,
}

impl Debts {
    /// Debts in `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self {
            outbox: Outbox::new(pool.clone(), clock.clone()),
            pool,
            clock,
        }
    }

    /// Adds an open Debt owed by `debtor`, one of `business`'s Debtors.
    pub async fn create(&self, business: BusinessId, debtor: DebtorId, input: &DebtInput) -> Result<DebtId, DebtError> {
        let mut problems = Vec::new();
        let invoice_number = input.invoice_number.trim();
        if invoice_number.is_empty()
            || invoice_number.chars().count() > 40
            || invoice_number.chars().any(char::is_control)
        {
            problems.push("Invoice number: 1 to 40 characters".to_owned());
        }
        let invoice_date = parse_date(&input.invoice_date);
        if invoice_date.is_none() {
            problems.push("Invoice date: a date like 2026-09-15".to_owned());
        }
        let amount = input.amount.trim();
        let centavos = parse_amount(amount);
        if centavos.is_none() {
            problems.push("Amount: a peso amount like ₱12,500.00".to_owned());
        }
        let due_date = parse_date(&input.due_date);
        if due_date.is_none() {
            problems.push("Due date: a date like 2026-10-15".to_owned());
        }
        let note = input.note.trim();
        if note.chars().count() > 500 {
            problems.push("Note: at most 500 characters".to_owned());
        }
        let (Some(invoice_date), Some(centavos), Some(due_date), true) =
            (invoice_date, centavos, due_date, problems.is_empty())
        else {
            return Err(DebtError::Invalid(problems));
        };
        let id = DebtId::new();
        let now = self.clock.now();
        // The join keeps a Debt from being added to another Business's Debtor.
        let inserted = sqlx::query!(
            "insert into debts
                (id, tenant_id, debtor_id, invoice_number, invoice_date, amount, amount_centavos, due_date, note,
                 status, created_at, updated_at)
             select $1, d.tenant_id, d.id, $4, $5, $6, $7, $8, $9, 'open', $10, $10
             from debtors d where d.tenant_id = $2 and d.id = $3",
            id.as_uuid(),
            business.as_uuid(),
            debtor.as_uuid(),
            invoice_number,
            invoice_date,
            amount,
            centavos,
            due_date,
            (!note.is_empty()).then_some(note),
            now,
        )
        .execute(&self.pool)
        .await?;
        if inserted.rows_affected() == 0 {
            return Err(DebtError::NotFound);
        }
        redraft_waiting(&self.outbox, &self.pool, business, debtor, "system").await?;
        Ok(id)
    }

    /// `debtor`'s Debts, earliest due first.
    pub async fn for_debtor(&self, business: BusinessId, debtor: DebtorId) -> Result<Vec<Debt>, DebtError> {
        let rows = sqlx::query!(
            "select id, invoice_number, invoice_date, amount, due_date, note, status
             from debts where tenant_id = $1 and debtor_id = $2
             order by due_date, invoice_number, id",
            business.as_uuid(),
            debtor.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| Debt {
                id: row.id.into(),
                debtor,
                invoice_number: row.invoice_number,
                invoice_date: row.invoice_date,
                amount: row.amount,
                due_date: row.due_date,
                note: row.note,
                status: DebtStatus::parse(&row.status),
            })
            .collect())
    }

    /// One of `business`'s Debts.
    pub async fn get(&self, business: BusinessId, id: DebtId) -> Result<Debt, DebtError> {
        let row = sqlx::query!(
            "select debtor_id, invoice_number, invoice_date, amount, due_date, note, status
             from debts where tenant_id = $1 and id = $2",
            business.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DebtError::NotFound)?;
        Ok(Debt {
            id,
            debtor: row.debtor_id.into(),
            invoice_number: row.invoice_number,
            invoice_date: row.invoice_date,
            amount: row.amount,
            due_date: row.due_date,
            note: row.note,
            status: DebtStatus::parse(&row.status),
        })
    }

    /// Records a partial payment or a correction: the amount still owed,
    /// as typed. The change is kept in the evidence log; `by` names who
    /// made it.
    pub async fn change_amount(
        &self,
        business: BusinessId,
        id: DebtId,
        amount: &str,
        by: &str,
    ) -> Result<(), DebtError> {
        let amount = amount.trim();
        let centavos = parse_amount(amount)
            .ok_or_else(|| DebtError::Invalid(vec!["Amount: a peso amount like ₱12,500.00".to_owned()]))?;
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        let before = sqlx::query!(
            "select amount, debtor_id from debts where tenant_id = $1 and id = $2 for update",
            business.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(DebtError::NotFound)?;
        sqlx::query!(
            "update debts set amount = $3, amount_centavos = $4, updated_at = $5 where tenant_id = $1 and id = $2",
            business.as_uuid(),
            id.as_uuid(),
            amount,
            centavos,
            now,
        )
        .execute(&mut *tx)
        .await?;
        let event = AuditEvent {
            tenant: business,
            at: now,
            actor: by,
            action: "debt.amount_changed",
            subject_type: "debt",
            subject_id: id.as_uuid(),
            details: json!({ "from": before.amount, "to": amount }),
        };
        audit::record(&mut *tx, &event).await?;
        tx.commit().await?;
        redraft_waiting(&self.outbox, &self.pool, business, before.debtor_id.into(), by).await?;
        Ok(())
    }
}

fn parse_date(text: &str) -> Option<Date> {
    Date::parse(text.trim(), format_description!("[year]-[month]-[day]")).ok()
}

/// The value in centavos of a peso amount written as a person would: an
/// optional `₱` or `PHP`, digits with optional thousands commas, and up to
/// two decimals, e.g. `₱12,500.00`, `PHP 7,250.5` or `800`. `None` for
/// anything else, zero, or more than [`MAX_CENTAVOS`].
fn parse_amount(text: &str) -> Option<i64> {
    let text = text.trim();
    let text = text
        .strip_prefix('₱')
        .or_else(|| {
            text.get(..3)
                .filter(|p| p.eq_ignore_ascii_case("php"))
                .map(|_| &text[3..])
        })
        .unwrap_or(text)
        .trim();
    let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
    if fraction.len() > 2 || !fraction.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let groups: Vec<&str> = whole.split(',').collect();
    let well_grouped = groups
        .first()
        .is_some_and(|g| (1..=3).contains(&g.len()) || groups.len() == 1)
        && groups.iter().skip(1).all(|g| g.len() == 3);
    if whole.is_empty() || !well_grouped || !groups.iter().all(|g| g.chars().all(|c| c.is_ascii_digit())) {
        return None;
    }
    let pesos: i64 = groups.concat().parse().ok()?;
    let centavos: i64 = format!("{fraction:0<2}").parse().ok()?;
    let value = pesos.checked_mul(100)?.checked_add(centavos)?;
    (1..=MAX_CENTAVOS).contains(&value).then_some(value)
}

#[cfg(test)]
mod tests {
    use super::parse_amount;

    #[test]
    fn parse_amount_reads_common_ways_of_writing_pesos() {
        let parsed: Vec<Option<i64>> = ["₱12,500.00", "PHP 7,250.5", "php800", "800", "1,234,567.89"]
            .into_iter()
            .map(parse_amount)
            .collect();

        assert_eq!(
            parsed,
            [
                Some(1_250_000),
                Some(725_050),
                Some(80_000),
                Some(80_000),
                Some(123_456_789)
            ]
        );
    }

    #[test]
    fn parse_amount_refuses_everything_else() {
        let parsed: Vec<Option<i64>> = ["", "0", "12k", "1,23", "12.345", "-500", "₱", "1,2345"]
            .into_iter()
            .map(parse_amount)
            .collect();

        assert!(parsed.iter().all(Option::is_none), "got {parsed:?}");
    }
}
