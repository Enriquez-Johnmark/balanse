//! H1: the dashboard lists Debtors needing action first, then active
//! Debtors, oldest overdue first.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use balanse::balanse::businesses::BusinessId;
use balanse::balanse::debtors::{DebtorId, DebtorInput, Debtors};
use balanse::balanse::debts::{DebtInput, Debts};
use balanse::engine::clock::FixedClock;
use common::{business, clock, scheduler};
use sqlx::PgPool;
use time::Duration;

async fn debtor_owing(pool: &PgPool, clock: &Arc<FixedClock>, business: BusinessId, name: &str, due: &str) -> DebtorId {
    let debtor = Debtors::new(pool.clone(), clock.clone())
        .create(
            business,
            &DebtorInput {
                business_name: name.to_owned(),
                contact_name: "Lito".to_owned(),
                contact_email: format!("{}@debtor.test", name.to_lowercase()),
                contact_mobile: String::new(),
                language: "english".to_owned(),
            },
        )
        .await
        .unwrap();
    Debts::new(pool.clone(), clock.clone())
        .create(
            business,
            debtor,
            &DebtInput {
                invoice_number: "INV-1".to_owned(),
                invoice_date: "2026-08-01".to_owned(),
                amount: "₱1,000".to_owned(),
                due_date: due.to_owned(),
                note: String::new(),
            },
        )
        .await
        .unwrap();
    debtor
}

async fn set_state(pool: &PgPool, debtor: DebtorId, state: &str, until: Option<&str>) {
    sqlx::query("update debtors set state = $2, promised_until = $3::date where id = $1")
        .bind(debtor.as_uuid())
        .bind(state)
        .bind(until)
        .execute(pool)
        .await
        .unwrap();
}

async fn order(pool: &PgPool, clock: &Arc<FixedClock>, business: BusinessId) -> Vec<String> {
    Debtors::new(pool.clone(), clock.clone())
        .list(business)
        .await
        .unwrap()
        .into_iter()
        .map(|row| row.business_name)
        .collect()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h1_debtors_needing_action_come_first_then_active_oldest_overdue_first(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    debtor_owing(&pool, &clock, business, "Newer", "2026-10-20").await;
    debtor_owing(&pool, &clock, business, "Older", "2026-09-01").await;
    let replied = debtor_owing(&pool, &clock, business, "Replied", "2026-10-25").await;
    set_state(&pool, replied, "paused", None).await;
    let disputed = debtor_owing(&pool, &clock, business, "Disputed", "2026-10-15").await;
    set_state(&pool, disputed, "disputed", None).await;
    let promised = debtor_owing(&pool, &clock, business, "Promised", "2026-07-01").await;
    set_state(&pool, promised, "promised", Some("2026-11-20")).await;

    let order = order(&pool, &clock, business).await;

    assert_eq!(order, ["Disputed", "Replied", "Older", "Newer", "Promised"]);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h1_a_debtor_with_a_draft_waiting_needs_action(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    debtor_owing(&pool, &clock, business, "Waiting", "2026-10-20").await;
    // Tuesday 07:00 in Manila: the Draft appears.
    clock.advance(Duration::hours(22));
    scheduler(&pool, &clock).run().await.unwrap();
    debtor_owing(&pool, &clock, business, "Alpha", "2026-09-01").await;

    let order = order(&pool, &clock, business).await;

    assert_eq!(order, ["Waiting", "Alpha"]);
}
