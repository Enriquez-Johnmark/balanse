//! Businesses: the service businesses that use Balanse. Each is an engine
//! account (and tenant) with Balanse's own row beside it.

use sqlx::PgPool;

use crate::engine::accounts::{self, AccountError};
use crate::engine::clock::Clock;
use crate::engine::ids::TenantId;

/// A Business's ID; the same value is its engine `TenantId`.
pub type BusinessId = TenantId;

/// Creates a Business named `name`, owned by `owner_email`, as an engine
/// account and Balanse's row for it, together.
pub async fn create_business(
    pool: &PgPool,
    clock: &dyn Clock,
    name: &str,
    owner_email: &str,
) -> Result<BusinessId, AccountError> {
    let now = clock.now();
    let mut tx = pool.begin().await?;
    let business = accounts::create_in(&mut tx, now, name, owner_email).await?;
    sqlx::query!(
        "insert into businesses (tenant_id, created_at) values ($1, $2)",
        business.as_uuid(),
        now,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(business)
}
