//! A3: sign-up asks whether the business lends money and refuses lenders,
//! keeping nothing; other requests wait for the operator's approval.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::StatusCode;
use balanse::balanse::signups::approve_signup;
use balanse::engine::accounts::Signups;
use common::{app, call, clock, log_in};
use serde_json::json;
use sqlx::PgPool;

fn request(lends_money: bool) -> serde_json::Value {
    json!({
        "business_name": "Reyes Creative",
        "email": "ana@reyes.test",
        "accepted_terms": true,
        "lends_money": lends_money,
    })
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a3_a_business_that_lends_money_is_refused_and_nothing_is_kept(pool: PgPool) {
    let clock = clock();
    let app = app(&pool, &clock);

    let answer = call(&app, "POST", "/api/signup", Some(request(true)), None).await;

    let kept = Signups::new(pool, clock).pending().await.unwrap().len();
    assert_eq!((answer.status, kept), (StatusCode::UNPROCESSABLE_ENTITY, 0));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a3_the_refusal_says_why(pool: PgPool) {
    let clock = clock();
    let app = app(&pool, &clock);

    let answer = call(&app, "POST", "/api/signup", Some(request(true)), None).await;

    let message = answer.body["message"].as_str().unwrap();
    assert!(message.contains("lend"), "got {message}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a3_the_lending_question_must_be_answered(pool: PgPool) {
    let clock = clock();
    let app = app(&pool, &clock);
    let mut unanswered = request(false);
    unanswered.as_object_mut().unwrap().remove("lends_money");

    let answer = call(&app, "POST", "/api/signup", Some(unanswered), None).await;

    assert_eq!(answer.status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a3_another_business_is_kept_for_the_operator(pool: PgPool) {
    let clock = clock();
    let app = app(&pool, &clock);

    let answer = call(&app, "POST", "/api/signup", Some(request(false)), None).await;

    let kept = Signups::new(pool, clock).pending().await.unwrap().len();
    assert_eq!((answer.status, kept), (StatusCode::ACCEPTED, 1));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a3_an_approved_request_becomes_a_business_that_can_log_in(pool: PgPool) {
    let clock = clock();
    let app = app(&pool, &clock);
    call(&app, "POST", "/api/signup", Some(request(false)), None).await;
    let id = Signups::new(pool.clone(), clock.clone()).pending().await.unwrap()[0].id;

    approve_signup(&pool, clock.clone(), id).await.unwrap();

    let cookie = log_in(&app, &pool, "ana@reyes.test").await;
    let me = call(&app, "GET", "/api/me", None, Some(&cookie)).await;
    assert_eq!(me.body["business_name"], "Reyes Creative");
}
