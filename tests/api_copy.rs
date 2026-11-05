//! H4 through the API: copying a message changes no approval, and "I sent
//! it on …" counts toward the cap.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use balanse::balanse::businesses::create_business;
use balanse::engine::clock::FixedClock;
use common::{app, business, call, clock, log_in};
use serde_json::{Value, json};
use sqlx::PgPool;

/// A Debtor with a mobile number owing one overdue invoice; returns the
/// app, the Owner's cookie and the Debtor's ID.
async fn debtor(pool: &PgPool, clock: &Arc<FixedClock>) -> (axum::Router, String, String) {
    business(pool, clock).await;
    let app = app(pool, clock);
    let cookie = log_in(&app, pool, "ana@reyes.test").await;
    let debtor = call(
        &app,
        "POST",
        "/api/debtors",
        Some(json!({ "business_name": "Santos Trading", "contact_name": "Lito",
                     "contact_email": "lito@santos.test", "contact_mobile": "0917 123 4567",
                     "language": "english" })),
        Some(&cookie),
    )
    .await;
    let id = debtor.body["id"].as_str().unwrap().to_owned();
    call(
        &app,
        "POST",
        &format!("/api/debtors/{id}/debts"),
        Some(
            json!({ "invoice_number": "INV-1", "invoice_date": "2026-09-30", "amount": "₱12,500.00",
                     "due_date": "2026-10-30", "note": "" }),
        ),
        Some(&cookie),
    )
    .await;
    (app, cookie, id)
}

async fn drafts(app: &axum::Router, cookie: &str) -> Value {
    call(app, "GET", "/api/drafts", None, Some(cookie)).await.body
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h4_copying_a_draft_changes_no_approval(pool: PgPool) {
    let clock = clock();
    let (app, cookie, _) = debtor(&pool, &clock).await;
    common::scheduler(&pool, &clock).run().await.unwrap();
    let before = drafts(&app, &cookie).await;
    let id = before[0]["id"].as_str().unwrap();

    let copy = call(&app, "GET", &format!("/api/messages/{id}/copy"), None, Some(&cookie)).await;

    assert_eq!(copy.status.as_u16(), 200, "{}", copy.body);
    assert_eq!(copy.body["text"], before[0]["body"]);
    let sms = copy.body["sms"].as_str().unwrap();
    assert!(!sms.contains("http") && sms.contains("INV-1"), "{sms}");
    assert!(
        copy.body["whatsapp_url"]
            .as_str()
            .unwrap()
            .starts_with("https://wa.me/639171234567?text=")
    );
    assert_eq!(drafts(&app, &cookie).await, before);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h4_a_message_sent_by_hand_counts_toward_the_cap(pool: PgPool) {
    let clock = clock();
    let (app, cookie, id) = debtor(&pool, &clock).await;

    let logged = call(
        &app,
        "POST",
        &format!("/api/debtors/{id}/sent-by-hand"),
        Some(json!({ "sent_on": "2026-11-02" })),
        Some(&cookie),
    )
    .await;
    common::scheduler(&pool, &clock).run().await.unwrap();

    assert_eq!(logged.status.as_u16(), 204, "{}", logged.body);
    assert_eq!(drafts(&app, &cookie).await, json!([]));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h4_a_message_cant_be_logged_as_sent_in_the_future(pool: PgPool) {
    let clock = clock();
    let (app, cookie, id) = debtor(&pool, &clock).await;

    let logged = call(
        &app,
        "POST",
        &format!("/api/debtors/{id}/sent-by-hand"),
        Some(json!({ "sent_on": "2026-11-03" })),
        Some(&cookie),
    )
    .await;

    assert_eq!(logged.status.as_u16(), 422, "{}", logged.body);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a2_another_business_cant_copy_or_log_for_a_debtor(pool: PgPool) {
    let clock = clock();
    let (app, cookie, id) = debtor(&pool, &clock).await;
    common::scheduler(&pool, &clock).run().await.unwrap();
    let message = drafts(&app, &cookie).await[0]["id"].as_str().unwrap().to_owned();
    create_business(&pool, clock.as_ref(), "Other Studio", "cara@other.test")
        .await
        .unwrap();
    let other = log_in(&app, &pool, "cara@other.test").await;

    let copy = call(
        &app,
        "GET",
        &format!("/api/messages/{message}/copy"),
        None,
        Some(&other),
    )
    .await;
    let logged = call(
        &app,
        "POST",
        &format!("/api/debtors/{id}/sent-by-hand"),
        Some(json!({ "sent_on": "2026-11-02" })),
        Some(&other),
    )
    .await;

    assert_eq!((copy.status.as_u16(), logged.status.as_u16()), (404, 404));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h4_a_draft_balanse_wouldnt_send_cant_be_copied(pool: PgPool) {
    let clock = clock();
    let (app, cookie, debtor) = debtor(&pool, &clock).await;
    common::scheduler(&pool, &clock).run().await.unwrap();
    let draft = drafts(&app, &cookie).await[0].clone();
    let id = draft["id"].as_str().unwrap();
    let threat = draft["body"]
        .as_str()
        .unwrap()
        .replace("Hi Lito,", "Hi Lito, makukulong ka kung hindi ka magbayad.");
    call(
        &app,
        "PUT",
        &format!("/api/drafts/{id}"),
        Some(json!({ "subject": draft["subject"], "body": threat })),
        Some(&cookie),
    )
    .await;

    let threatening = call(&app, "GET", &format!("/api/messages/{id}/copy"), None, Some(&cookie)).await;
    sqlx::query("update debtors set state = 'opted_out' where id = $1::uuid")
        .bind(&debtor)
        .execute(&pool)
        .await
        .unwrap();
    let opted_out = call(&app, "GET", &format!("/api/messages/{id}/copy"), None, Some(&cookie)).await;

    assert_eq!(threatening.status.as_u16(), 409, "{}", threatening.body);
    assert!(
        threatening.body["message"].as_str().unwrap().contains("arrest"),
        "{}",
        threatening.body
    );
    assert_eq!(opted_out.status.as_u16(), 409, "{}", opted_out.body);
}
