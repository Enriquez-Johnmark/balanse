//! The Owner's answer to a claim of payment (G2). Confirmed, the Debtor's
//! Debts are settled, as with "paid"; rejected, reminders resume where the
//! cycle left off. Either way the claim's proof files are deleted 30 days
//! later (G5), and the answer is kept in the evidence log.

use std::sync::Arc;

use serde_json::json;
use sqlx::PgPool;

use super::businesses::BusinessId;
use super::contact_actions::ClaimId;
use super::debtors::DebtorId;
use super::outcomes::{OutcomeError, Outcomes};
use super::proofs::{ProofError, Proofs};
use crate::engine::audit::{self, AuditEvent};
use crate::engine::clock::Clock;

/// Why a claim couldn't be answered.
#[derive(Debug, thiserror::Error)]
pub enum ClaimError {
    /// No such claim in this Business.
    #[error("claim not found")]
    NotFound,
    /// It was already confirmed or rejected.
    #[error("the claim was already answered")]
    AlreadySettled,
    /// Settling the Debts failed.
    #[error(transparent)]
    Outcome(#[from] OutcomeError),
    /// Bringing the proof files' deletion forward failed.
    #[error(transparent)]
    Proof(#[from] ProofError),
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// The Owner's answers to claims.
#[derive(Clone)]
pub struct Claims {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    outcomes: Outcomes,
    proofs: Option<Proofs>,
}

impl Claims {
    /// Claims in `pool`; `proofs`, if files are kept, has their deletion
    /// brought forward once a claim is answered.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, proofs: Option<Proofs>) -> Self {
        Self {
            outcomes: Outcomes::new(pool.clone(), clock.clone()),
            pool,
            clock,
            proofs,
        }
    }

    /// The Debtor did pay: its Debts are settled.
    pub async fn confirm(&self, business: BusinessId, claim: ClaimId, by: &str) -> Result<(), ClaimError> {
        let debtor = self.settle(business, claim, "confirmed", by).await?;
        self.outcomes.paid(business, debtor, by).await?;
        self.delete_files_soon(business, claim).await
    }

    /// The payment didn't arrive: reminders resume where they left off.
    pub async fn reject(&self, business: BusinessId, claim: ClaimId, by: &str) -> Result<(), ClaimError> {
        let debtor = self.settle(business, claim, "rejected", by).await?;
        sqlx::query!(
            "update debtors set state = 'active', updated_at = $3
             where tenant_id = $1 and id = $2 and state = 'payment_claimed'",
            business.as_uuid(),
            debtor.as_uuid(),
            self.clock.now(),
        )
        .execute(&self.pool)
        .await?;
        self.delete_files_soon(business, claim).await
    }

    /// Marks `claim` answered with `outcome` and returns its Debtor.
    async fn settle(
        &self,
        business: BusinessId,
        claim: ClaimId,
        outcome: &str,
        by: &str,
    ) -> Result<DebtorId, ClaimError> {
        let now = self.clock.now();
        let mut tx = self.pool.begin().await?;
        let row = sqlx::query!(
            "select debtor_id, settled_at from claims where tenant_id = $1 and id = $2 for update",
            business.as_uuid(),
            claim.as_uuid(),
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(ClaimError::NotFound)?;
        if row.settled_at.is_some() {
            return Err(ClaimError::AlreadySettled);
        }
        sqlx::query!(
            "update claims set settled_at = $3, outcome = $4 where tenant_id = $1 and id = $2",
            business.as_uuid(),
            claim.as_uuid(),
            now,
            outcome,
        )
        .execute(&mut *tx)
        .await?;
        let event = AuditEvent {
            tenant: business,
            at: now,
            actor: by,
            action: if outcome == "confirmed" {
                "claim.confirmed"
            } else {
                "claim.rejected"
            },
            subject_type: "claim",
            subject_id: claim.as_uuid(),
            details: json!({ "debtor": row.debtor_id }),
        };
        audit::record(&mut *tx, &event).await?;
        tx.commit().await?;
        Ok(row.debtor_id.into())
    }

    async fn delete_files_soon(&self, business: BusinessId, claim: ClaimId) -> Result<(), ClaimError> {
        if let Some(proofs) = &self.proofs {
            proofs.settle(business, claim).await?;
        }
        Ok(())
    }
}
