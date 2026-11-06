//! H3: the Owner can export one Debtor's evidence log; logs are deleted 1
//! year after the Debt is closed.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use balanse::balanse::evidence::{Entry, Evidence, to_csv};
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

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h3_the_evidence_log_holds_the_approved_text_the_send_and_status_changes(pool: PgPool) {
    let clock = clock();
    let (business, debtor) = sent(&pool, &clock).await;
    clock.advance(time::Duration::days(1));
    Outcomes::new(pool.clone(), clock.clone())
        .paid(business, debtor, "ana@reyes.test")
        .await
        .unwrap();

    let log = Evidence::new(pool.clone()).for_debtor(business, debtor).await.unwrap();

    let what: Vec<&str> = log.iter().map(|entry| entry.what.as_str()).collect();
    assert_eq!(
        what,
        [
            "Invoice INV-1 added",
            "Reminder approved",
            "Reminder sent",
            "Marked as paid"
        ]
    );
    assert!(
        log[1].details.contains("Hi Lito") && log[1].by == "ana@reyes.test",
        "{:?}",
        log[1]
    );
    assert!(log[2].details.contains("lito@santos.test"), "{:?}", log[2]);
}

#[test]
fn h3_the_exported_log_is_csv_with_formulas_defused() {
    let at = time::macros::datetime!(2026-11-04 23:30 UTC);
    let log = [Entry {
        at,
        what: "Reminder approved".to_owned(),
        by: "ana@reyes.test".to_owned(),
        details: "=HYPERLINK(\"http://evil.test\")".to_owned(),
    }];

    let csv = to_csv(&log);

    assert_eq!(
        csv,
        "When (Manila time),What,By,Details\n2026-11-05 07:30,Reminder approved,ana@reyes.test,\"'=HYPERLINK(\"\"http://evil.test\"\")\"\n"
    );
}
