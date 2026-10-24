//! B1, B3: Debts in, each with the Business's own invoice reference, and
//! amounts kept exactly as typed.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use balanse::balanse::debtors::{DebtorError, DebtorInput, Debtors};
use balanse::balanse::debts::{DebtError, DebtInput, Debts};
use common::{business, clock};
use sqlx::PgPool;

fn debtor() -> DebtorInput {
    DebtorInput {
        business_name: "Santos Trading".to_owned(),
        contact_name: "Lito".to_owned(),
        contact_email: "lito@santos.test".to_owned(),
        contact_mobile: String::new(),
        language: "taglish".to_owned(),
    }
}

fn debt() -> DebtInput {
    DebtInput {
        invoice_number: "INV-0042".to_owned(),
        invoice_date: "2026-09-15".to_owned(),
        amount: "₱12,500.00".to_owned(),
        due_date: "2026-10-15".to_owned(),
        note: String::new(),
    }
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b1_a_debtor_needs_a_contact_email(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtors = Debtors::new(pool, clock);

    let err = debtors
        .create(
            business,
            &DebtorInput {
                contact_email: String::new(),
                ..debtor()
            },
        )
        .await
        .unwrap_err();

    assert!(matches!(err, DebtorError::Invalid(_)), "got {err:?}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b1_a_debt_needs_its_invoice_number_dates_and_amount(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = Debtors::new(pool.clone(), clock.clone())
        .create(business, &debtor())
        .await
        .unwrap();
    let debts = Debts::new(pool, clock);

    let err = debts
        .create(
            business,
            debtor,
            &DebtInput {
                invoice_number: String::new(),
                invoice_date: String::new(),
                amount: String::new(),
                due_date: String::new(),
                note: String::new(),
            },
        )
        .await
        .unwrap_err();

    let DebtError::Invalid(problems) = err else {
        panic!("expected Invalid, got {err:?}");
    };
    assert_eq!(problems.len(), 4, "got {problems:?}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b1_a_complete_debt_is_saved_open(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = Debtors::new(pool.clone(), clock.clone())
        .create(business, &debtor())
        .await
        .unwrap();
    let debts = Debts::new(pool, clock);

    let id = debts.create(business, debtor, &debt()).await.unwrap();

    let saved = debts.get(business, id).await.unwrap();
    assert_eq!(
        (saved.invoice_number.as_str(), saved.status.as_str()),
        ("INV-0042", "open")
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b1_an_amount_that_is_not_a_peso_amount_is_refused(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = Debtors::new(pool.clone(), clock.clone())
        .create(business, &debtor())
        .await
        .unwrap();
    let debts = Debts::new(pool, clock);

    let err = debts
        .create(
            business,
            debtor,
            &DebtInput {
                amount: "about 12k".to_owned(),
                ..debt()
            },
        )
        .await
        .unwrap_err();

    assert!(matches!(err, DebtError::Invalid(_)), "got {err:?}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b3_an_amount_is_kept_exactly_as_typed(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = Debtors::new(pool.clone(), clock.clone())
        .create(business, &debtor())
        .await
        .unwrap();
    let debts = Debts::new(pool, clock);

    let id = debts
        .create(
            business,
            debtor,
            &DebtInput {
                amount: "PHP 7,250.5".to_owned(),
                ..debt()
            },
        )
        .await
        .unwrap();

    assert_eq!(debts.get(business, id).await.unwrap().amount, "PHP 7,250.5");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b3_changing_an_amount_is_recorded_in_the_evidence_log(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = Debtors::new(pool.clone(), clock.clone())
        .create(business, &debtor())
        .await
        .unwrap();
    let debts = Debts::new(pool.clone(), clock);
    let id = debts.create(business, debtor, &debt()).await.unwrap();

    debts
        .change_amount(business, id, "₱5,000.00", "ana@reyes.test")
        .await
        .unwrap();

    let details: serde_json::Value =
        sqlx::query_scalar("select details from audit_events where tenant_id = $1 and action = 'debt.amount_changed'")
            .bind(business.as_uuid())
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(details, serde_json::json!({ "from": "₱12,500.00", "to": "₱5,000.00" }));
}
