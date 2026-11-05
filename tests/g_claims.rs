//! G2: the Owner confirms or rejects a claim of payment; G5: either way the
//! proof files go 30 days later.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use balanse::balanse::businesses::BusinessId;
use balanse::balanse::claims::{ClaimError, Claims};
use balanse::balanse::contact_actions::{ClaimId, ContactActions};
use balanse::balanse::debtors::{DebtorId, DebtorInput, Debtors};
use balanse::balanse::debts::{DebtInput, Debts};
use balanse::balanse::proofs::{KEPT_AFTER_SETTLED_DAYS, Proofs};
use balanse::engine::clock::{Clock, FixedClock};
use common::{business, clock};
use sqlx::PgPool;
use time::Duration;

struct World {
    clock: Arc<FixedClock>,
    business: BusinessId,
    debtor: DebtorId,
    claim: ClaimId,
    proofs: Proofs,
    _dir: tempfile::TempDir,
}

/// Santos Trading owes INV-1, got its first reminder drafted, and says it
/// paid, attaching a photo.
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
    let claim = ContactActions::new(pool.clone(), clock.clone(), common::links(pool, &clock))
        .claim_paid(business, debtor)
        .await
        .unwrap();
    let dir = tempfile::tempdir().unwrap();
    let proofs = Proofs::new(pool.clone(), clock.clone(), common::files(pool, &clock, dir.path()));
    proofs.add(business, claim, b"\xFF\xD8\xFF\xE0 receipt").await.unwrap();
    World {
        clock,
        business,
        debtor,
        claim,
        proofs,
        _dir: dir,
    }
}

impl World {
    fn claims(&self, pool: &PgPool) -> Claims {
        Claims::new(pool.clone(), self.clock.clone(), Some(self.proofs.clone()))
    }
}

async fn row(pool: &PgPool, debtor: DebtorId) -> (String, String, Option<String>) {
    let state: String = sqlx::query_scalar("select state from debtors where id = $1")
        .bind(debtor.as_uuid())
        .fetch_one(pool)
        .await
        .unwrap();
    let debt: String = sqlx::query_scalar("select status from debts")
        .fetch_one(pool)
        .await
        .unwrap();
    let outcome: Option<String> = sqlx::query_scalar("select outcome from claims")
        .fetch_one(pool)
        .await
        .unwrap();
    (state, debt, outcome)
}

async fn file_deadline(pool: &PgPool) -> time::OffsetDateTime {
    sqlx::query_scalar("select delete_by from stored_files")
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g2_a_confirmed_claim_settles_the_debts(pool: PgPool) {
    let world = world(&pool).await;

    world
        .claims(&pool)
        .confirm(world.business, world.claim, "ana@reyes.test")
        .await
        .unwrap();
    let again = world
        .claims(&pool)
        .confirm(world.business, world.claim, "ana@reyes.test")
        .await;

    assert_eq!(
        row(&pool, world.debtor).await,
        ("active".to_owned(), "paid".to_owned(), Some("confirmed".to_owned()))
    );
    assert!(matches!(again, Err(ClaimError::AlreadySettled)), "{again:?}");
    assert_eq!(
        file_deadline(&pool).await,
        world.clock.now() + Duration::days(KEPT_AFTER_SETTLED_DAYS)
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g2_a_rejected_claim_resumes_reminders(pool: PgPool) {
    let world = world(&pool).await;

    world
        .claims(&pool)
        .reject(world.business, world.claim, "ana@reyes.test")
        .await
        .unwrap();
    world.clock.advance(Duration::days(7));
    common::scheduler(&pool, &world.clock).run().await.unwrap();

    assert_eq!(
        row(&pool, world.debtor).await,
        ("active".to_owned(), "open".to_owned(), Some("rejected".to_owned()))
    );
    let drafted: i64 = sqlx::query_scalar(
        "select count(*) from outbox_messages where to_address = 'lito@santos.test' and status = 'draft'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(drafted, 1, "the next reminder is drafted");
    assert_eq!(
        file_deadline(&pool).await,
        world.clock.now() - Duration::days(7) + Duration::days(KEPT_AFTER_SETTLED_DAYS)
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a2_another_business_cant_answer_a_claim(pool: PgPool) {
    let world = world(&pool).await;
    let other = balanse::balanse::businesses::create_business(&pool, world.clock.as_ref(), "Other Co", "bo@other.test")
        .await
        .unwrap();

    let result = world.claims(&pool).confirm(other, world.claim, "bo@other.test").await;

    assert!(matches!(result, Err(ClaimError::NotFound)), "{result:?}");
    assert_eq!(row(&pool, world.debtor).await.0, "payment_claimed");
}
