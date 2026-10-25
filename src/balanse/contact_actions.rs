//! What a Contact does on the Debtor's page (G2–G4): says they've paid,
//! disputes what is owed, or asks for no more email. Each stops the
//! reminders at once, is kept in the evidence log, and tells the Owner.
//! A Contact's words never reach the Owner's email: the notice says only
//! what happened.

use std::sync::Arc;

use serde_json::json;
use sqlx::PgPool;

use super::businesses::BusinessId;
use super::debtor_links::DebtorLinks;
use super::debtors::DebtorId;
use super::notices::{Notice, about, approve_notice, draft_notice};
use super::presend::BalanseChecks;
use super::reminders::cancel_waiting;
use crate::engine::audit::{self, AuditEvent};
use crate::engine::clock::Clock;
use crate::engine::ids::id_type;
use crate::engine::outbox::{Outbox, OutboxError};

id_type!(
    /// One claim of payment.
    ClaimId
);

/// Why a Contact's action couldn't be recorded.
#[derive(Debug, thiserror::Error)]
pub enum ActionError {
    /// No such Debtor in this Business.
    #[error("debtor not found")]
    NotFound,
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    /// The outbox refused the Owner's notice or a cancellation.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
}

/// The Contacts' actions, for every Business.
#[derive(Clone)]
pub struct ContactActions {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    outbox: Outbox,
    links: DebtorLinks,
}

impl ContactActions {
    /// Actions in `pool`; notices link to the app where `links` point.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, links: DebtorLinks) -> Self {
        Self {
            outbox: Outbox::new(pool.clone(), clock.clone()),
            pool,
            clock,
            links,
        }
    }

    /// The Contact says they've paid (G2): the Debtor is "payment claimed"
    /// and reminders pause until the Owner confirms or rejects the claim.
    pub async fn claim_paid(&self, business: BusinessId, debtor: DebtorId) -> Result<ClaimId, ActionError> {
        let about = about(&self.pool, business, debtor)
            .await?
            .ok_or(ActionError::NotFound)?;
        let now = self.clock.now();
        let id = ClaimId::new();
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            "insert into claims (id, tenant_id, debtor_id, claimed_at) values ($1, $2, $3, $4)",
            id.as_uuid(),
            business.as_uuid(),
            debtor.as_uuid(),
            now,
        )
        .execute(&mut *tx)
        .await?;
        // A stop or a bounce stays as it is; anything else is now a claim.
        sqlx::query!(
            "update debtors set state = 'payment_claimed', promised_until = null, updated_at = $3
             where tenant_id = $1 and id = $2 and state not in ('opted_out', 'bounced')",
            business.as_uuid(),
            debtor.as_uuid(),
            now,
        )
        .execute(&mut *tx)
        .await?;
        let event = AuditEvent {
            tenant: business,
            at: now,
            actor: "contact",
            action: "debtor.payment_claimed",
            subject_type: "debtor",
            subject_id: debtor.as_uuid(),
            details: json!({ "claim": id.as_uuid() }),
        };
        audit::record(&mut *tx, &event).await?;
        let notice = draft_notice(
            &self.outbox,
            &mut tx,
            (business, debtor),
            &about,
            self.links.base_url(),
            &Notice {
                subject: &format!("{} ({}) says they've paid", about.contact_name, about.debtor_name),
                what: &format!("{} of {} says they've paid.", about.contact_name, about.debtor_name),
                then: "Reminders are paused until you confirm or reject the claim.",
            },
        )
        .await?;
        tx.commit().await?;
        cancel_waiting(&self.outbox, &self.pool, business, debtor, "contact").await?;
        let checks = BalanseChecks::for_approval(self.pool.clone(), self.links.clone());
        approve_notice(&self.outbox, &checks, business, notice).await?;
        Ok(id)
    }
}
