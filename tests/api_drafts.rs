//! The Drafts API: D1 and D2 through the web, as the approval page uses it,
//! and A2 for every Draft endpoint.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use axum::http::StatusCode;
use balanse::balanse::businesses::create_business;
use balanse::balanse::reminders::{ReplyAddresses, Scheduler};
use balanse::engine::clock::FixedClock;
use balanse::engine::files::MasterKey;
use balanse::engine::signed::Signer;
use common::{app, business, call, clock, log_in};
use serde_json::{Value, json};
use sqlx::PgPool;

/// Reyes Creative logged in, with one Debtor whose first reminder is
/// drafted; returns the app, the cookie and the Drafts list.
async fn with_a_draft(pool: &PgPool, clock: &Arc<FixedClock>) -> (axum::Router, String, Value) {
    business(pool, clock).await;
    let app = app(pool, clock);
    let cookie = log_in(&app, pool, "ana@reyes.test").await;
    let debtor = call(
        &app,
        "POST",
        "/api/debtors",
        Some(json!({ "business_name": "Santos Trading", "contact_name": "Lito",
                     "contact_email": "lito@santos.test", "contact_mobile": "", "language": "english" })),
        Some(&cookie),
    )
    .await;
    let id = debtor.body["id"].as_str().unwrap();
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
    let master = MasterKey::from_hex(&"ab".repeat(32)).unwrap();
    let replies = ReplyAddresses::new(Arc::new(Signer::new(&master, "test replies")), "reply.balanse.test");
    Scheduler::new(pool.clone(), clock.clone(), replies)
        .run()
        .await
        .unwrap();
    let drafts = call(&app, "GET", "/api/drafts", None, Some(&cookie)).await.body;
    (app, cookie, drafts)
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d1_the_drafts_page_shows_each_draft_in_full(pool: PgPool) {
    let clock = clock();

    let (_, _, drafts) = with_a_draft(&pool, &clock).await;

    let draft = &drafts[0];
    assert!(
        draft["debtor_name"] == "Santos Trading"
            && draft["to_address"] == "lito@santos.test"
            && draft["body"].as_str().unwrap().contains("INV-1")
            && draft["hash"].as_str().unwrap().len() == 64,
        "got {draft}"
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d2_approving_with_the_hash_shown_approves_the_draft(pool: PgPool) {
    let clock = clock();
    let (app, cookie, drafts) = with_a_draft(&pool, &clock).await;
    let item = json!({ "id": drafts[0]["id"], "hash": drafts[0]["hash"] });

    let answer = call(
        &app,
        "POST",
        "/api/drafts/approve",
        Some(json!({ "drafts": [item] })),
        Some(&cookie),
    )
    .await;

    let after = call(&app, "GET", "/api/drafts", None, Some(&cookie)).await.body;
    assert_eq!(
        (
            answer.status,
            answer.body[0]["approved"].clone(),
            after.as_array().unwrap().len()
        ),
        (StatusCode::OK, json!(true), 0)
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d2_a_draft_changed_since_it_was_shown_isnt_approved(pool: PgPool) {
    let clock = clock();
    let (app, cookie, drafts) = with_a_draft(&pool, &clock).await;
    let id = drafts[0]["id"].as_str().unwrap();
    let body = drafts[0]["body"].as_str().unwrap().replace("Hi Lito", "Hello Lito");
    call(
        &app,
        "PUT",
        &format!("/api/drafts/{id}"),
        Some(json!({ "subject": drafts[0]["subject"], "body": body })),
        Some(&cookie),
    )
    .await;
    let stale = json!({ "id": id, "hash": drafts[0]["hash"] });

    let answer = call(
        &app,
        "POST",
        "/api/drafts/approve",
        Some(json!({ "drafts": [stale] })),
        Some(&cookie),
    )
    .await;

    assert_eq!(answer.body[0]["approved"], json!(false), "got {}", answer.body);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d1_editing_a_draft_keeps_it_waiting_with_the_new_text(pool: PgPool) {
    let clock = clock();
    let (app, cookie, drafts) = with_a_draft(&pool, &clock).await;
    let id = drafts[0]["id"].as_str().unwrap();
    let body = drafts[0]["body"].as_str().unwrap().replace("Hi Lito", "Hello Lito");

    call(
        &app,
        "PUT",
        &format!("/api/drafts/{id}"),
        Some(json!({ "subject": drafts[0]["subject"], "body": body })),
        Some(&cookie),
    )
    .await;

    let after = call(&app, "GET", "/api/drafts", None, Some(&cookie)).await.body;
    assert!(
        after[0]["body"].as_str().unwrap().starts_with("Hello Lito"),
        "got {after}"
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d1_a_discarded_draft_is_never_sent(pool: PgPool) {
    let clock = clock();
    let (app, cookie, drafts) = with_a_draft(&pool, &clock).await;
    let id = drafts[0]["id"].as_str().unwrap();

    call(&app, "POST", &format!("/api/drafts/{id}/discard"), None, Some(&cookie)).await;

    let status: String = sqlx::query_scalar("select status from outbox_messages where id = $1::uuid")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status, "cancelled");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a2_another_business_cant_see_or_act_on_a_draft(pool: PgPool) {
    let clock = clock();
    let (app, _, drafts) = with_a_draft(&pool, &clock).await;
    create_business(&pool, clock.as_ref(), "Other Studio", "cara@other.test")
        .await
        .unwrap();
    let other = log_in(&app, &pool, "cara@other.test").await;
    let id = drafts[0]["id"].as_str().unwrap();
    let item = json!({ "id": id, "hash": drafts[0]["hash"] });

    let listed = call(&app, "GET", "/api/drafts", None, Some(&other)).await.body;
    let approved = call(
        &app,
        "POST",
        "/api/drafts/approve",
        Some(json!({ "drafts": [item] })),
        Some(&other),
    )
    .await;
    let edited = call(
        &app,
        "PUT",
        &format!("/api/drafts/{id}"),
        Some(json!({ "subject": "x", "body": "y" })),
        Some(&other),
    )
    .await;
    let discarded = call(&app, "POST", &format!("/api/drafts/{id}/discard"), None, Some(&other)).await;

    assert_eq!(
        (
            listed.as_array().unwrap().len(),
            approved.body[0]["approved"].clone(),
            edited.status,
            discarded.status
        ),
        (0, json!(false), StatusCode::NOT_FOUND, StatusCode::NOT_FOUND)
    );
}
