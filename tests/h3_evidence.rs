//! H3: the Owner can export one Debtor's evidence log; logs are deleted 1
//! year after the Debt is closed.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use balanse::balanse::evidence::{Entry, Evidence, to_csv};
use balanse::balanse::outcomes::Outcomes;
use balanse::balanse::retention::Retention;
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

/// `GET path` with `cookie`: the status, the content-disposition header and
/// the body.
async fn download(app: &axum::Router, path: &str, cookie: &str) -> (u16, String, String) {
    use tower::ServiceExt;
    let request = axum::http::Request::get(path)
        .header("cookie", cookie)
        .body(axum::body::Body::empty())
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status().as_u16();
    let disposition = response
        .headers()
        .get("content-disposition")
        .map(|value| value.to_str().unwrap().to_owned())
        .unwrap_or_default();
    let bytes = axum::body::to_bytes(response.into_body(), 1 << 20).await.unwrap();
    (status, disposition, String::from_utf8(bytes.to_vec()).unwrap())
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h3_the_owner_downloads_a_debtors_evidence_log(pool: PgPool) {
    let clock = clock();
    let (_, debtor) = sent(&pool, &clock).await;
    let app = common::app(&pool, &clock);
    let ana = common::log_in(&app, &pool, "ana@reyes.test").await;
    balanse::balanse::businesses::create_business(&pool, clock.as_ref(), "Other Studio", "cara@other.test")
        .await
        .unwrap();
    let cara = common::log_in(&app, &pool, "cara@other.test").await;
    let path = format!("/api/debtors/{}/evidence-log", debtor.as_uuid());

    let (status, disposition, body) = download(&app, &path, &ana).await;
    let (other, _, _) = download(&app, &path, &cara).await;

    assert_eq!(status, 200);
    assert!(
        disposition.starts_with("attachment;") && disposition.contains("evidence-log"),
        "{disposition}"
    );
    assert!(
        body.starts_with("When (Manila time),What,By,Details\n") && body.contains("Reminder sent"),
        "{body}"
    );
    assert_eq!(other, 404, "another Business can't download it (A2)");
}

async fn count(pool: &PgPool, table: &str) -> i64 {
    // `table` is one of this file's own constants.
    sqlx::query_scalar(sqlx::AssertSqlSafe(format!("select count(*) from {table}")))
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h3_a_debtor_settled_over_a_year_ago_is_deleted_with_its_messages(pool: PgPool) {
    let clock = clock();
    let (business, debtor) = sent(&pool, &clock).await;
    Outcomes::new(pool.clone(), clock.clone())
        .paid(business, debtor, "ana@reyes.test")
        .await
        .unwrap();
    let retention = Retention::new(pool.clone(), clock.clone());

    clock.advance(time::Duration::days(364));
    retention.sweep().await.unwrap();
    let kept = (count(&pool, "debtors").await, count(&pool, "outbox_messages").await);
    clock.advance(time::Duration::days(2));
    retention.sweep().await.unwrap();

    assert_eq!(kept, (1, 1));
    assert_eq!(
        (
            count(&pool, "debtors").await,
            count(&pool, "debts").await,
            count(&pool, "outbox_messages").await
        ),
        (0, 0, 0)
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h3_an_old_closed_debt_goes_while_an_open_one_keeps_the_debtor(pool: PgPool) {
    let clock = clock();
    let (business, debtor) = sent(&pool, &clock).await;
    sqlx::query("update debts set status = 'paid', closed_at = $1")
        .bind(clock.now() - time::Duration::days(400))
        .execute(&pool)
        .await
        .unwrap();
    balanse::balanse::debts::Debts::new(pool.clone(), clock.clone())
        .create(
            business,
            debtor,
            &balanse::balanse::debts::DebtInput {
                invoice_number: "INV-2".to_owned(),
                invoice_date: "2026-10-01".to_owned(),
                amount: "₱2,000".to_owned(),
                due_date: "2026-11-01".to_owned(),
                note: String::new(),
            },
        )
        .await
        .unwrap();

    Retention::new(pool.clone(), clock.clone()).sweep().await.unwrap();

    let left: Vec<String> = sqlx::query_scalar("select invoice_number from debts")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!((left, count(&pool, "debtors").await), (vec!["INV-2".to_owned()], 1));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h3_a_settled_debtor_with_a_message_still_going_out_is_erased_later(pool: PgPool) {
    let clock = clock();
    let (business, debtor) = sent(&pool, &clock).await;
    Outcomes::new(pool.clone(), clock.clone())
        .paid(business, debtor, "ana@reyes.test")
        .await
        .unwrap();
    // The message is still being sent when the year is up.
    sqlx::query("update outbox_messages set status = 'sending', send_started_at = sent_at")
        .execute(&pool)
        .await
        .unwrap();
    let retention = Retention::new(pool.clone(), clock.clone());
    clock.advance(time::Duration::days(366));

    retention.sweep().await.unwrap();
    let waiting = (count(&pool, "debtors").await, count(&pool, "debts").await);
    sqlx::query("update outbox_messages set status = 'sent'")
        .execute(&pool)
        .await
        .unwrap();
    retention.sweep().await.unwrap();

    assert_eq!(waiting, (1, 1), "kept whole until its message is done");
    assert_eq!(
        (count(&pool, "debtors").await, count(&pool, "outbox_messages").await),
        (0, 0)
    );
}
