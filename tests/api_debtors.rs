//! The Debtors and Debts API: B1, B3, F3 and A2 through the web, as the
//! React app uses it.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::StatusCode;
use balanse::balanse::businesses::create_business;
use common::{app, business, call, clock, log_in};
use serde_json::{Value, json};
use sqlx::PgPool;

fn debtor() -> Value {
    json!({
        "business_name": "Santos Trading",
        "contact_name": "Lito",
        "contact_email": "lito@santos.test",
        "contact_mobile": "",
        "language": "taglish",
    })
}

fn debt() -> Value {
    json!({
        "invoice_number": "INV-0042",
        "invoice_date": "2026-09-15",
        "amount": "₱12,500.00",
        "due_date": "2026-10-15",
        "note": "",
    })
}

/// Logs in as Reyes Creative's owner and adds a Debtor with one Debt,
/// returning the cookie and the Debtor's ID.
async fn reyes(app: &axum::Router, pool: &PgPool) -> (String, String) {
    let cookie = log_in(app, pool, "ana@reyes.test").await;
    let created = call(app, "POST", "/api/debtors", Some(debtor()), Some(&cookie)).await;
    let id = created.body["id"].as_str().unwrap().to_owned();
    call(
        app,
        "POST",
        &format!("/api/debtors/{id}/debts"),
        Some(debt()),
        Some(&cookie),
    )
    .await;
    (cookie, id)
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b1_the_api_refuses_a_debt_without_its_invoice_details(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let (cookie, id) = reyes(&app, &pool).await;

    let answer = call(
        &app,
        "POST",
        &format!("/api/debtors/{id}/debts"),
        Some(json!({ "invoice_number": "", "invoice_date": "", "amount": "", "due_date": "", "note": "" })),
        Some(&cookie),
    )
    .await;

    assert_eq!(
        (answer.status, answer.body["problems"].as_array().unwrap().len()),
        (StatusCode::UNPROCESSABLE_ENTITY, 4)
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b3_a_debtors_page_shows_the_amount_exactly_as_typed(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let (cookie, id) = reyes(&app, &pool).await;

    let page = call(&app, "GET", &format!("/api/debtors/{id}"), None, Some(&cookie)).await;

    assert_eq!(page.body["debts"][0]["amount"], "₱12,500.00");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b3_a_partial_payment_changes_the_amount_still_owed(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let (cookie, id) = reyes(&app, &pool).await;
    let page = call(&app, "GET", &format!("/api/debtors/{id}"), None, Some(&cookie)).await;
    let debt_id = page.body["debts"][0]["id"].as_str().unwrap().to_owned();

    call(
        &app,
        "POST",
        &format!("/api/debts/{debt_id}/amount"),
        Some(json!({ "amount": "₱5,000.00" })),
        Some(&cookie),
    )
    .await;

    let page = call(&app, "GET", &format!("/api/debtors/{id}"), None, Some(&cookie)).await;
    assert_eq!(page.body["debts"][0]["amount"], "₱5,000.00");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f3_the_owner_records_that_the_debtor_paid(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let (cookie, id) = reyes(&app, &pool).await;

    let answer = call(
        &app,
        "POST",
        &format!("/api/debtors/{id}/outcome"),
        Some(json!({ "outcome": "paid" })),
        Some(&cookie),
    )
    .await;

    let page = call(&app, "GET", &format!("/api/debtors/{id}"), None, Some(&cookie)).await;
    assert_eq!(
        (answer.status, page.body["debts"][0]["status"].clone()),
        (StatusCode::NO_CONTENT, json!("paid"))
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h2_the_debtor_list_carries_no_total_of_amounts(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let (cookie, _) = reyes(&app, &pool).await;

    let list = call(&app, "GET", "/api/debtors", None, Some(&cookie)).await;

    let text = list.body.to_string();
    assert!(
        list.body[0]["open_debts"] == 1 && !text.contains("12,500") && !text.contains("total"),
        "got {text}"
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a2_another_business_gets_not_found_for_every_debtor_endpoint(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    create_business(&pool, clock.as_ref(), "Other Studio", "cara@other.test")
        .await
        .unwrap();
    let app = app(&pool, &clock);
    let (reyes_cookie, id) = reyes(&app, &pool).await;
    let page = call(&app, "GET", &format!("/api/debtors/{id}"), None, Some(&reyes_cookie)).await;
    let debt_id = page.body["debts"][0]["id"].as_str().unwrap().to_owned();
    let other = log_in(&app, &pool, "cara@other.test").await;

    let statuses = [
        call(&app, "GET", &format!("/api/debtors/{id}"), None, Some(&other))
            .await
            .status,
        call(
            &app,
            "POST",
            &format!("/api/debtors/{id}/debts"),
            Some(debt()),
            Some(&other),
        )
        .await
        .status,
        call(
            &app,
            "POST",
            &format!("/api/debts/{debt_id}/amount"),
            Some(json!({ "amount": "1" })),
            Some(&other),
        )
        .await
        .status,
        call(
            &app,
            "POST",
            &format!("/api/debtors/{id}/outcome"),
            Some(json!({ "outcome": "paid" })),
            Some(&other),
        )
        .await
        .status,
    ];

    assert_eq!(statuses, [StatusCode::NOT_FOUND; 4]);
}
