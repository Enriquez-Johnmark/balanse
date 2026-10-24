//! Replies to reminders (F1, F2). A reply counts only when it comes to a
//! reminder's signed reply address; anything else is ignored.
//!
//! - A human reply is kept, pauses the Debtor's reminders, cancels any that
//!   wait, and tells the Owner. The notice never copies the reply's text,
//!   so Balanse's domain never relays content that could be phishing.
//! - An automatic reply (out of office and the like) is kept and changes
//!   nothing.
//! - A message delivered twice is kept once.
//!
//! This handler finds the Business from the signed reply address, before
//! any Business is known (AGENTS.md, Business isolation).

use std::sync::Arc;

use serde_json::json;
use sqlx::PgPool;

use super::presend::BalanseChecks;
use super::reminders::{ReminderId, ReplyAddresses, cancel_waiting};
use crate::engine::audit::{self, AuditEvent};
use crate::engine::clock::Clock;
use crate::engine::ids::{TenantId, id_type};
use crate::engine::inbound::InboundEmail;
use crate::engine::mail::Email;
use crate::engine::outbox::{Outbox, OutboxError};

id_type!(
    /// One reply kept.
    ReplyId
);

/// The approver recorded for notices Balanse sends the Owner itself.
pub const NOTICE_APPROVER: &str = "system:notice";

/// What a received message did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplyOutcome {
    /// A human reply: the Debtor is paused and the Owner told.
    Paused,
    /// An automatic reply: kept, nothing else.
    Automatic,
    /// Not to any reminder's signed reply address: ignored.
    Unmatched,
    /// Already received.
    Duplicate,
}

/// Reply errors: only infrastructure failures.
#[derive(Debug, thiserror::Error)]
pub enum ReplyError {
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    /// The outbox refused the Owner's notice or a cancellation.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
}

/// Takes in replies.
#[derive(Clone)]
pub struct Replies {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    outbox: Outbox,
    addresses: ReplyAddresses,
    base_url: String,
}

impl Replies {
    /// Replies matched with `addresses`; notices link to the app at
    /// `base_url`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, addresses: ReplyAddresses, base_url: &str) -> Self {
        Self {
            outbox: Outbox::new(pool.clone(), clock.clone()),
            pool,
            clock,
            addresses,
            base_url: base_url.trim_end_matches('/').to_owned(),
        }
    }

    /// Takes in one inbound message.
    pub async fn receive(&self, email: &InboundEmail) -> Result<ReplyOutcome, ReplyError> {
        let Some(reminder) = email.recipients.iter().find_map(|to| self.addresses.reminder(to)) else {
            return Ok(ReplyOutcome::Unmatched);
        };
        let Some(found) = self.find(reminder).await? else {
            return Ok(ReplyOutcome::Unmatched);
        };
        let tenant = TenantId::from(found.tenant_id);
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        let id = ReplyId::new();
        let kept = sqlx::query!(
            "insert into replies
                (id, tenant_id, debtor_id, reminder_id, from_address, subject, body_text, automatic, received_at,
                 message_key)
             values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             on conflict (tenant_id, message_key) do nothing",
            id.as_uuid(),
            found.tenant_id,
            found.debtor_id,
            reminder.as_uuid(),
            email.from,
            email.subject,
            email.text,
            email.automatic,
            now,
            email.key,
        )
        .execute(&mut *tx)
        .await?;
        if kept.rows_affected() == 0 {
            return Ok(ReplyOutcome::Duplicate);
        }
        if email.automatic {
            tx.commit().await?;
            return Ok(ReplyOutcome::Automatic);
        }
        sqlx::query!(
            "update debtors set state = 'paused', promised_until = null, updated_at = $3
             where tenant_id = $1 and id = $2 and state in ('active', 'promised', 'needs_call')",
            found.tenant_id,
            found.debtor_id,
            now,
        )
        .execute(&mut *tx)
        .await?;
        let event = AuditEvent {
            tenant,
            at: now,
            actor: "system",
            action: "debtor.replied",
            subject_type: "debtor",
            subject_id: found.debtor_id,
            details: json!({ "reply": id.as_uuid() }),
        };
        audit::record(&mut *tx, &event).await?;
        let notice = Email {
            to_address: found.owner_email,
            from_name: "Balanse".to_owned(),
            reply_to: None,
            subject: format!("{} ({}) replied", found.contact_name, found.debtor_name),
            body_text: format!(
                "Hi,\n\n{contact} of {debtor} replied to your reminder. Read the reply in Balanse:\n\n\
                 {base}/debtors/{id}\n\nReminders to {debtor} are paused until you decide what happens next.\n",
                contact = found.contact_name,
                debtor = found.debtor_name,
                base = self.base_url,
                id = found.debtor_id,
            ),
        };
        let notice = self.outbox.create_draft_in(&mut tx, tenant, &notice, None).await?;
        tx.commit().await?;
        cancel_waiting(&self.outbox, &self.pool, tenant, found.debtor_id.into(), "system").await?;
        let hash = self.outbox.get(tenant, notice).await?.content_hash();
        self.outbox
            .approve(
                tenant,
                notice,
                NOTICE_APPROVER,
                &hash,
                &BalanseChecks::for_approval(self.pool.clone()),
            )
            .await?;
        Ok(ReplyOutcome::Paused)
    }

    async fn find(&self, reminder: ReminderId) -> Result<Option<Found>, sqlx::Error> {
        sqlx::query_as!(
            Found,
            "select r.tenant_id, r.debtor_id, d.contact_name, d.business_name as debtor_name, m.email as owner_email
             from reminders r
             join debtors d on d.tenant_id = r.tenant_id and d.id = r.debtor_id
             join accounts a on a.tenant_id = r.tenant_id and a.closed_at is null
             join account_members m on m.tenant_id = r.tenant_id and m.role = 'owner'
             where r.id = $1",
            reminder.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await
    }
}

/// The reminder a reply answered, and who to tell.
struct Found {
    tenant_id: uuid::Uuid,
    debtor_id: uuid::Uuid,
    contact_name: String,
    debtor_name: String,
    owner_email: String,
}
