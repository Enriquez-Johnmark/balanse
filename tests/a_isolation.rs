//! A2: one Business can never see or change another Business's Debtors or
//! Debts; another Business's IDs are "not found".

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use balanse::balanse::businesses::{BusinessId, create_business};
use balanse::balanse::debtors::{DebtorError, DebtorId, DebtorInput, Debtors};
use balanse::balanse::debts::{DebtError, DebtId, DebtInput, Debts};
use balanse::engine::clock::FixedClock;
use common::{business, clock};
use sqlx::PgPool;

/// Reyes Creative with one Debtor and one Debt, and a second Business.
async fn two_businesses(pool: &PgPool, clock: &Arc<FixedClock>) -> (BusinessId, DebtorId, DebtId, BusinessId) {
    let reyes = business(pool, clock).await;
    let other = create_business(pool, clock.as_ref(), "Other Studio", "cara@other.test")
        .await
        .unwrap();
    let debtor = Debtors::new(pool.clone(), clock.clone())
        .create(
            reyes,
            &DebtorInput {
                business_name: "Santos Trading".to_owned(),
                contact_name: "Lito".to_owned(),
                contact_email: "lito@santos.test".to_owned(),
                contact_mobile: String::new(),
                language: "english".to_owned(),
            },
        )
        .await
        .unwrap();
    let debt = Debts::new(pool.clone(), clock.clone())
        .create(
            reyes,
            debtor,
            &DebtInput {
                invoice_number: "INV-1".to_owned(),
                invoice_date: "2026-09-15".to_owned(),
                amount: "₱1,000.00".to_owned(),
                due_date: "2026-10-15".to_owned(),
                note: String::new(),
            },
        )
        .await
        .unwrap();
    (reyes, debtor, debt, other)
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a2_another_businesss_debtor_is_not_found(pool: PgPool) {
    let clock = clock();
    let (_, debtor, _, other) = two_businesses(&pool, &clock).await;

    let err = Debtors::new(pool, clock).get(other, debtor).await.unwrap_err();

    assert!(matches!(err, DebtorError::NotFound), "got {err:?}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a2_another_businesss_debt_is_not_found(pool: PgPool) {
    let clock = clock();
    let (_, _, debt, other) = two_businesses(&pool, &clock).await;

    let err = Debts::new(pool, clock).get(other, debt).await.unwrap_err();

    assert!(matches!(err, DebtError::NotFound), "got {err:?}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a2_another_business_cannot_change_a_debts_amount(pool: PgPool) {
    let clock = clock();
    let (_, _, debt, other) = two_businesses(&pool, &clock).await;

    let err = Debts::new(pool, clock)
        .change_amount(other, debt, "₱1.00", "cara@other.test")
        .await
        .unwrap_err();

    assert!(matches!(err, DebtError::NotFound), "got {err:?}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a2_another_business_cannot_add_a_debt_to_a_debtor(pool: PgPool) {
    let clock = clock();
    let (_, debtor, _, other) = two_businesses(&pool, &clock).await;

    let err = Debts::new(pool, clock)
        .create(
            other,
            debtor,
            &DebtInput {
                invoice_number: "X".to_owned(),
                invoice_date: "2026-09-15".to_owned(),
                amount: "1".to_owned(),
                due_date: "2026-10-15".to_owned(),
                note: String::new(),
            },
        )
        .await
        .unwrap_err();

    assert!(matches!(err, DebtError::NotFound), "got {err:?}");
}
