//! H3: the Owner can export one Debtor's evidence log; logs are deleted 1
//! year after the Debt is closed.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use balanse::balanse::outcomes::Outcomes;
use balanse::engine::clock::Clock;
use common::{clock, sent};
use sqlx::PgPool;

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h3_a_paid_debt_records_when_it_closed(pool: PgPool) {
    let clock = clock();
    let (business, debtor) = sent(&pool, &clock).await;

    Outcomes::new(pool.clone(), clock.clone())
        .paid(business, debtor, "ana@reyes.test")
        .await
        .unwrap();

    let closed_at: Option<time::OffsetDateTime> = sqlx::query_scalar("select closed_at from debts")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(closed_at, Some(clock.now()));
}
