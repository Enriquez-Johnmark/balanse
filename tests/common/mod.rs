//! Shared test helpers.

#![allow(dead_code, reason = "each test binary uses a different subset")]

use std::sync::Arc;

use balanse::balanse::businesses::{BusinessId, create_business};
use balanse::engine::clock::FixedClock;
use sqlx::PgPool;
use time::macros::datetime;

/// A clock fixed at Monday 2026-11-02 09:00 Manila time (01:00 UTC); tests
/// move it forward by hand.
pub fn clock() -> Arc<FixedClock> {
    Arc::new(FixedClock::new(datetime!(2026-11-02 01:00 UTC)))
}

/// A new Business, "Reyes Creative", owned by ana@reyes.test.
pub async fn business(pool: &PgPool, clock: &Arc<FixedClock>) -> BusinessId {
    create_business(pool, clock.as_ref(), "Reyes Creative", "ana@reyes.test")
        .await
        .unwrap()
}
