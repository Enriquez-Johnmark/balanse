//! Account requests (A3). The engine keeps them for the operator; Balanse
//! approves one by creating the Business (the engine account and Balanse's
//! own row together) and then removing the request.

use std::sync::Arc;

use sqlx::PgPool;

use super::businesses::{BusinessId, create_business};
use crate::engine::accounts::{AccountError, RequestId, SignupError, Signups};
use crate::engine::clock::Clock;

/// Approving a request failed.
#[derive(Debug, thiserror::Error)]
pub enum ApproveError {
    /// No such request waiting.
    #[error("request not found")]
    NotFound,
    /// Creating the Business failed.
    #[error(transparent)]
    Account(#[from] AccountError),
    /// Reading or removing the request failed.
    #[error(transparent)]
    Signup(#[from] SignupError),
}

/// Creates the Business a waiting request asked for, then removes the
/// request. If removing it fails, approving it again fails harmlessly: the
/// address already logs in.
pub async fn approve_signup(pool: &PgPool, clock: Arc<dyn Clock>, id: RequestId) -> Result<BusinessId, ApproveError> {
    let signups = Signups::new(pool.clone(), clock.clone());
    let request = signups
        .pending()
        .await?
        .into_iter()
        .find(|request| request.id == id)
        .ok_or(ApproveError::NotFound)?;
    let business = create_business(pool, clock.as_ref(), &request.name, &request.email).await?;
    signups.decline(id).await?;
    Ok(business)
}
