//! Retention: what Balanse deletes, and when. A system-wide worker: it
//! sweeps every Business, naming each row's Business in every change.
//!
//! - Reply text (subject and body) 90 days after the reply arrived; that it
//!   came, and when, remains (F5).
//! - A closed Debt, a year after it closed (H3, R17). When every Debt of a
//!   Debtor closed over a year ago, the whole Debtor goes: its contact
//!   details, its evidence log and the messages sent to it.

use std::sync::Arc;

use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

use super::businesses::BusinessId;
use super::debtors::DebtorId;
use super::reminders::cancel_waiting;
use crate::engine::audit::SYSTEM;
use crate::engine::clock::Clock;
use crate::engine::ids::MessageId;
use crate::engine::outbox::{Outbox, OutboxError};

/// How long a reply's text is kept.
pub const REPLY_TEXT_KEPT: Duration = Duration::days(90);

/// How long a closed Debt, and its part of the evidence log, is kept.
pub const CLOSED_DEBT_KEPT: Duration = Duration::days(365);

/// Retention errors.
#[derive(Debug, thiserror::Error)]
pub enum RetentionError {
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    /// Cancelling or forgetting messages failed.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
}

/// What one sweep deleted.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Swept {
    /// Replies whose text was deleted.
    pub reply_texts: u64,
    /// Debtors deleted with everything of theirs.
    pub debtors: u64,
    /// Closed Debts deleted from Debtors that still owe.
    pub debts: u64,
}

/// Deletes what is past its time.
#[derive(Clone)]
pub struct Retention {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    outbox: Outbox,
}

impl Retention {
    /// Retention over `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self {
            outbox: Outbox::new(pool.clone(), clock.clone()),
            pool,
            clock,
        }
    }

    /// Deletes everything past its time, for every Business.
    pub async fn sweep(&self) -> Result<Swept, RetentionError> {
        let now = self.clock.now();
        let reply_texts = sqlx::query!(
            "update replies set subject = '', body_text = null
             where received_at <= $1 and (body_text is not null or subject <> '')",
            now - REPLY_TEXT_KEPT,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        let cutoff = now - CLOSED_DEBT_KEPT;
        let debtors = self.erase_settled_debtors(cutoff).await?;
        // Debtors that still owe keep their messages, which are evidence for
        // what is still open.
        let debts = sqlx::query!("delete from debts where closed_at <= $1", cutoff,)
            .execute(&self.pool)
            .await?
            .rows_affected();
        Ok(Swept {
            reply_texts,
            debtors,
            debts,
        })
    }

    /// Deletes every Debtor whose Debts all closed before `cutoff`, with its
    /// messages. One whose messages can't all be forgotten yet (one is still
    /// being sent) waits for the next sweep.
    async fn erase_settled_debtors(&self, cutoff: OffsetDateTime) -> Result<u64, RetentionError> {
        let settled = sqlx::query!(
            "select d.tenant_id, d.id from debtors d
             where exists (select 1 from debts t where t.tenant_id = d.tenant_id and t.debtor_id = d.id)
               and not exists (select 1 from debts t where t.tenant_id = d.tenant_id and t.debtor_id = d.id
                               and (t.closed_at is null or t.closed_at > $1))",
            cutoff,
        )
        .fetch_all(&self.pool)
        .await?;
        let mut erased = 0;
        for row in settled {
            let (business, debtor): (BusinessId, DebtorId) = (row.tenant_id.into(), row.id.into());
            cancel_waiting(&self.outbox, &self.pool, business, debtor, SYSTEM).await?;
            let messages: Vec<MessageId> = sqlx::query_scalar!(
                "select message_id from reminders where tenant_id = $1 and debtor_id = $2",
                business.as_uuid(),
                debtor.as_uuid(),
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(MessageId::from)
            .collect();
            let forgotten = self.outbox.forget(business, &messages).await?;
            if usize::try_from(forgotten).unwrap_or(usize::MAX) < messages.len() {
                tracing::info!(%business, %debtor, "a message is still going out; erasing the debtor later");
                continue;
            }
            erased += sqlx::query!(
                "delete from debtors where tenant_id = $1 and id = $2",
                business.as_uuid(),
                debtor.as_uuid(),
            )
            .execute(&self.pool)
            .await?
            .rows_affected();
        }
        Ok(erased)
    }
}
