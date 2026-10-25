//! G2–G4: what a Contact does on the Debtor's page stops reminders at once
//! and tells the Owner, without the Contact's words.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use balanse::balanse::businesses::BusinessId;
use balanse::balanse::contact_actions::ContactActions;
use balanse::balanse::debtors::{DebtorId, DebtorInput, Debtors};
use balanse::balanse::debts::{DebtInput, Debts};
use balanse::engine::clock::FixedClock;
use common::{business, clock};
use sqlx::PgPool;

struct World {
    clock: Arc<FixedClock>,
    business: BusinessId,
    debtor: DebtorId,
}

/// Reyes Creative, Santos Trading owing it INV-1, and a first reminder
/// waiting for approval.
async fn world(pool: &PgPool) -> World {
    let clock = clock();
    let business = business(pool, &clock).await;
    let debtor = Debtors::new(pool.clone(), clock.clone())
        .create(
            business,
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
    Debts::new(pool.clone(), clock.clone())
        .create(
            business,
            debtor,
            &DebtInput {
                invoice_number: "INV-1".to_owned(),
                invoice_date: "2026-09-30".to_owned(),
                amount: "₱12,500.00".to_owned(),
                due_date: "2026-10-30".to_owned(),
                note: String::new(),
            },
        )
        .await
        .unwrap();
    common::scheduler(pool, &clock).run().await.unwrap();
    World {
        clock,
        business,
        debtor,
    }
}

impl World {
    fn actions(&self, pool: &PgPool) -> ContactActions {
        ContactActions::new(pool.clone(), self.clock.clone(), common::links(pool, &self.clock))
    }
}

async fn state(pool: &PgPool, debtor: DebtorId) -> String {
    sqlx::query_scalar("select state from debtors where id = $1")
        .bind(debtor.as_uuid())
        .fetch_one(pool)
        .await
        .unwrap()
}

/// Every outbox message's recipient and status.
async fn messages(pool: &PgPool) -> Vec<(String, String)> {
    sqlx::query_as("select to_address, status from outbox_messages order by to_address")
        .fetch_all(pool)
        .await
        .unwrap()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g2_i_have_paid_claims_payment_pauses_reminders_and_tells_the_owner(pool: PgPool) {
    let world = world(&pool).await;

    world
        .actions(&pool)
        .claim_paid(world.business, world.debtor)
        .await
        .unwrap();

    assert_eq!(state(&pool, world.debtor).await, "payment_claimed");
    assert_eq!(
        messages(&pool).await,
        [
            ("ana@reyes.test".to_owned(), "approved".to_owned()),
            ("lito@santos.test".to_owned(), "cancelled".to_owned())
        ]
    );
    let subject: String = sqlx::query_scalar("select subject from outbox_messages where to_address = 'ana@reyes.test'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(subject, "Lito (Santos Trading) says they've paid");
    let claims: i64 = sqlx::query_scalar("select count(*) from claims where settled_at is null")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(claims, 1);
}
