//! Approving Drafts. Each approval binds the exact content the Owner was
//! shown, by its hash: a Draft changed since then isn't approved (D1, D2).

use super::businesses::BusinessId;
use crate::engine::ids::MessageId;
use crate::engine::outbox::{ContentHash, Outbox, OutboxError, PreSendCheck};

/// Approves each Draft in `seen` (its ID and the hash of the content the
/// Owner reviewed), returning each one's outcome. One failure doesn't stop
/// the rest.
pub async fn approve_batch(
    outbox: &Outbox,
    business: BusinessId,
    seen: &[(MessageId, ContentHash)],
    approver: &str,
    checks: &dyn PreSendCheck,
) -> Vec<(MessageId, Result<(), OutboxError>)> {
    let mut results = Vec::with_capacity(seen.len());
    for (id, hash) in seen {
        let result = outbox.approve(business, *id, approver, hash, checks).await;
        results.push((*id, result));
    }
    results
}
