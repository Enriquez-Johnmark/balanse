//! Bounces (F4): delivery failures and spam complaints the provider reports
//! for reminders Balanse sent. Only an event matching one of our messages
//! counts.
//!
//! A permanent bounce or a complaint marks every Debtor of the Business at
//! that address `bounced`, which stops their reminders until the Owner
//! changes the Contact's address; their waiting reminders are cancelled. A temporary bounce is only noted, for the
//! dashboard. A Debtor who asked to stop stays `opted_out`.
//!
//! Recording is a system-wide worker: the provider's message ID names the
//! Business, so that one lookup isn't scoped. Everything after it is.

use std::sync::Arc;

use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use super::businesses::BusinessId;
use super::debtors::DebtorId;
use super::reminders::cancel_waiting;
use crate::engine::audit::{self, AuditEvent, SYSTEM};
use crate::engine::clock::Clock;
use crate::engine::outbox::{Outbox, OutboxError};
use crate::engine::webhooks::DeliveryEvent;

/// Bounce errors.
#[derive(Debug, thiserror::Error)]
pub enum BounceError {
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    /// Cancelling the Debtor's waiting reminders failed.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
}

/// What became of a delivery event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recorded {
    /// It matched no reminder Balanse sent, or wasn't a failure: ignored.
    Ignored,
    /// A temporary bounce, or one for an address since replaced: noted.
    Noted,
    /// A permanent bounce or complaint: the Debtor's reminders stopped.
    Stopped,
}

/// Records delivery failures.
#[derive(Clone)]
pub struct Bounces {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    outbox: Outbox,
}

impl Bounces {
    /// Bounces of `pool`'s Businesses.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self {
            outbox: Outbox::new(pool.clone(), clock.clone()),
            pool,
            clock,
        }
    }

    /// Takes in one verified delivery event.
    pub async fn record(&self, event: &DeliveryEvent) -> Result<Recorded, BounceError> {
        let (provider_message_id, kind, reason) = match event {
            DeliveryEvent::Bounced {
                provider_message_id,
                permanent,
                reason,
            } => (
                provider_message_id,
                if *permanent { "bounce" } else { "soft_bounce" },
                Some(reason.as_str()),
            ),
            DeliveryEvent::Complained { provider_message_id } => (provider_message_id, "complaint", None),
            _ => return Ok(Recorded::Ignored),
        };
        let Some(sent) = self.outbox.find_sent(provider_message_id).await? else {
            tracing::info!(provider_message_id, "a delivery event for no message we sent; ignored");
            return Ok(Recorded::Ignored);
        };
        let business: BusinessId = sent.tenant;
        let debtor = sqlx::query_scalar!(
            "select debtor_id from reminders where tenant_id = $1 and message_id = $2",
            business.as_uuid(),
            sent.id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await?;
        let Some(debtor) = debtor.map(DebtorId::from) else {
            // A login email or a notice to the Owner: not a reminder.
            tracing::warn!(%business, kind, "a message to the business itself bounced");
            return Ok(Recorded::Ignored);
        };
        let address = sent.to_address.trim().to_lowercase();
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            "insert into bounces (id, tenant_id, debtor_id, message_id, address, kind, reason, at)
             values ($1, $2, $3, $4, $5, $6, $7, $8)
             on conflict (message_id, kind) do nothing",
            Uuid::now_v7(),
            business.as_uuid(),
            debtor.as_uuid(),
            sent.id.as_uuid(),
            address,
            kind,
            reason,
            now,
        )
        .execute(&mut *tx)
        .await?;
        if kind == "soft_bounce" {
            tx.commit().await?;
            return Ok(Recorded::Noted);
        }
        // Every Debtor of this Business at the failed address stops (F4); a
        // late bounce for an address since replaced stops nothing.
        let stopped: Vec<DebtorId> = sqlx::query_scalar!(
            "update debtors set state = 'bounced', promised_until = null, updated_at = $3
             where tenant_id = $1 and lower(trim(contact_email)) = $2 and state <> 'opted_out'
             returning id",
            business.as_uuid(),
            address,
            now,
        )
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .map(DebtorId::from)
        .collect();
        for stopped in &stopped {
            audit::record(
                &mut *tx,
                &AuditEvent {
                    tenant: business,
                    at: now,
                    actor: SYSTEM,
                    action: "debtor.bounced",
                    subject_type: "debtor",
                    subject_id: stopped.as_uuid(),
                    details: json!({ "kind": kind, "message": sent.id.as_uuid(), "debtor": debtor.as_uuid() }),
                },
            )
            .await?;
        }
        tx.commit().await?;
        if stopped.is_empty() {
            return Ok(Recorded::Noted);
        }
        for stopped in stopped {
            cancel_waiting(&self.outbox, &self.pool, business, stopped, SYSTEM).await?;
        }
        Ok(Recorded::Stopped)
    }
}
