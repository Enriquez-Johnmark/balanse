//! What the Owner decides after a reply or once a Debtor needs their call
//! (F3). Each decision is kept in the evidence log.
//!
//! - **Paid:** the Debtor's open Debts are settled; its next Debt starts a
//!   fresh cycle.
//! - **Promised** to pay by a date: reminders resume the day after it.
//! - **Disputed:** reminders stop.
//! - **Resume:** a new cycle starts today, so the first reminder is due
//!   tomorrow.
//!
//! A Debtor who asked to stop, or whose address bounced, stays stopped: the
//! Owner can't resume reminders or take a promise for them (G4, F4). Paid
//! and disputed are still recorded.

use std::sync::Arc;

use serde_json::json;
use sqlx::PgPool;
use time::Date;

use super::businesses::BusinessId;
use super::debtors::DebtorId;
use super::reminders::{MANILA, cancel_waiting};
use crate::engine::audit::{self, AuditEvent};
use crate::engine::clock::Clock;
use crate::engine::outbox::{Outbox, OutboxError};

/// Outcome errors.
#[derive(Debug, thiserror::Error)]
pub enum OutcomeError {
    /// No such Debtor in this Business.
    #[error("debtor not found")]
    NotFound,
    /// A promise date that has already passed.
    #[error("the promised date must be today or later")]
    PastDate,
    /// The Debtor asked to stop, or their address bounced; reminders stay off.
    #[error("reminders to this debtor are stopped")]
    Stopped,
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    /// Cancelling a waiting reminder failed.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
}

/// The Owner's decisions about Debtors.
#[derive(Clone)]
pub struct Outcomes {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    outbox: Outbox,
}

impl Outcomes {
    /// Outcomes in `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self {
            outbox: Outbox::new(pool.clone(), clock.clone()),
            pool,
            clock,
        }
    }

    /// The Debtor paid: its open Debts (and any it claimed to have paid)
    /// are settled.
    pub async fn paid(&self, business: BusinessId, debtor: DebtorId, by: &str) -> Result<(), OutcomeError> {
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        let settled = sqlx::query!(
            "update debts set status = 'paid', updated_at = $3
             where tenant_id = $1 and debtor_id = $2 and status in ('open', 'payment_claimed', 'disputed')",
            business.as_uuid(),
            debtor.as_uuid(),
            now,
        )
        .execute(&mut *tx)
        .await?
        .rows_affected();
        self.set_state(&mut tx, business, debtor, Decision::Paid).await?;
        record(
            &mut tx,
            business,
            debtor,
            now,
            by,
            "debtor.paid",
            json!({ "debts": settled }),
        )
        .await?;
        tx.commit().await?;
        cancel_waiting(&self.outbox, &self.pool, business, debtor, by).await?;
        Ok(())
    }

    /// The Debtor promised to pay by `until`; reminders resume the day after.
    pub async fn promised(
        &self,
        business: BusinessId,
        debtor: DebtorId,
        until: Date,
        by: &str,
    ) -> Result<(), OutcomeError> {
        let now = self.clock.now();
        if until < now.to_offset(MANILA).date() {
            return Err(OutcomeError::PastDate);
        }
        let mut tx = self.pool.begin().await?;
        self.set_state(&mut tx, business, debtor, Decision::Promised(until))
            .await?;
        record(
            &mut tx,
            business,
            debtor,
            now,
            by,
            "debtor.promised",
            json!({ "until": until.to_string() }),
        )
        .await?;
        tx.commit().await?;
        cancel_waiting(&self.outbox, &self.pool, business, debtor, by).await?;
        Ok(())
    }

    /// The Debtor disputes what it owes; reminders stop.
    pub async fn disputed(&self, business: BusinessId, debtor: DebtorId, by: &str) -> Result<(), OutcomeError> {
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        self.set_state(&mut tx, business, debtor, Decision::Disputed).await?;
        record(&mut tx, business, debtor, now, by, "debtor.disputed", json!({})).await?;
        tx.commit().await?;
        cancel_waiting(&self.outbox, &self.pool, business, debtor, by).await?;
        Ok(())
    }

    /// Reminders start again: a new cycle from today.
    pub async fn resume(&self, business: BusinessId, debtor: DebtorId, by: &str) -> Result<(), OutcomeError> {
        let now = self.clock.now();
        let today = now.to_offset(MANILA).date();
        let mut tx = self.pool.begin().await?;
        self.set_state(&mut tx, business, debtor, Decision::Resume).await?;
        sqlx::query!(
            "update debtors set cycle_start = $3 where tenant_id = $1 and id = $2",
            business.as_uuid(),
            debtor.as_uuid(),
            today,
        )
        .execute(&mut *tx)
        .await?;
        record(&mut tx, business, debtor, now, by, "debtor.resumed", json!({})).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Moves the Debtor to the state `decision` leads to, unless it is
    /// stopped: then a decision that would turn reminders on is refused, and
    /// any other leaves the stop in place.
    async fn set_state(
        &self,
        tx: &mut sqlx::PgConnection,
        business: BusinessId,
        debtor: DebtorId,
        decision: Decision,
    ) -> Result<(), OutcomeError> {
        let current = sqlx::query_scalar!(
            "select state from debtors where tenant_id = $1 and id = $2 for update",
            business.as_uuid(),
            debtor.as_uuid(),
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(OutcomeError::NotFound)?;
        if matches!(current.as_str(), "opted_out" | "bounced") {
            return match decision {
                Decision::Promised(_) | Decision::Resume => Err(OutcomeError::Stopped),
                Decision::Paid | Decision::Disputed => Ok(()),
            };
        }
        let (state, promised_until, new_cycle) = match decision {
            Decision::Paid => ("active", None, true),
            Decision::Promised(until) => ("promised", Some(until), false),
            Decision::Disputed => ("disputed", None, false),
            Decision::Resume => ("active", None, false),
        };
        sqlx::query!(
            "update debtors
             set state = $3, promised_until = $4,
                 cycle_start = case when $5 then null else cycle_start end,
                 updated_at = $6
             where tenant_id = $1 and id = $2",
            business.as_uuid(),
            debtor.as_uuid(),
            state,
            promised_until,
            new_cycle,
            self.clock.now(),
        )
        .execute(&mut *tx)
        .await?;
        Ok(())
    }
}

/// What the Owner decided.
#[derive(Debug, Clone, Copy)]
enum Decision {
    Paid,
    Promised(Date),
    Disputed,
    Resume,
}

async fn record(
    tx: &mut sqlx::PgConnection,
    business: BusinessId,
    debtor: DebtorId,
    at: time::OffsetDateTime,
    by: &str,
    action: &str,
    details: serde_json::Value,
) -> Result<(), sqlx::Error> {
    let event = AuditEvent {
        tenant: business,
        at,
        actor: by,
        action,
        subject_type: "debtor",
        subject_id: debtor.as_uuid(),
        details,
    };
    audit::record(&mut *tx, &event).await
}
