//! A1: logging in by a one-time email link, and the API needing a session.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::StatusCode;
use common::{app, business, call, clock, log_in, login_token};
use serde_json::json;
use sqlx::PgPool;
use time::Duration;

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a1_a_login_link_works_once(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    call(
        &app,
        "POST",
        "/api/login",
        Some(json!({ "email": "ana@reyes.test" })),
        None,
    )
    .await;
    let token = login_token(&pool, "ana@reyes.test").await.unwrap();
    let first = call(&app, "POST", &format!("/api/login/{token}"), None, None).await;

    let second = call(&app, "POST", &format!("/api/login/{token}"), None, None).await;

    assert_eq!(
        (first.status, first.cookie.is_some(), second.status),
        (StatusCode::OK, true, StatusCode::UNPROCESSABLE_ENTITY)
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a1_a_login_link_expires_after_15_minutes(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    call(
        &app,
        "POST",
        "/api/login",
        Some(json!({ "email": "ana@reyes.test" })),
        None,
    )
    .await;
    let token = login_token(&pool, "ana@reyes.test").await.unwrap();
    clock.advance(Duration::minutes(15) + Duration::seconds(1));

    let late = call(&app, "POST", &format!("/api/login/{token}"), None, None).await;

    assert_eq!(late.status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a1_an_unknown_address_gets_the_same_answer_and_no_email(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let known = call(
        &app,
        "POST",
        "/api/login",
        Some(json!({ "email": "ana@reyes.test" })),
        None,
    )
    .await;

    let unknown = call(
        &app,
        "POST",
        "/api/login",
        Some(json!({ "email": "who@else.test" })),
        None,
    )
    .await;

    login_token(&pool, "ana@reyes.test").await.unwrap();
    assert_eq!(
        (known.status, unknown.status, login_token(&pool, "who@else.test").await),
        (StatusCode::ACCEPTED, StatusCode::ACCEPTED, None)
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a1_the_api_needs_a_session(pool: PgPool) {
    let clock = clock();
    let app = app(&pool, &clock);

    let answer = call(&app, "GET", "/api/me", None, None).await;

    assert_eq!(answer.status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a1_a_session_names_the_business_and_the_person(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let cookie = log_in(&app, &pool, "ana@reyes.test").await;

    let me = call(&app, "GET", "/api/me", None, Some(&cookie)).await;

    assert_eq!(
        me.body,
        json!({ "business_name": "Reyes Creative", "email": "ana@reyes.test", "role": "owner" })
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a1_logging_out_ends_the_session(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let cookie = log_in(&app, &pool, "ana@reyes.test").await;
    call(&app, "POST", "/api/logout", None, Some(&cookie)).await;

    let after = call(&app, "GET", "/api/me", None, Some(&cookie)).await;

    assert_eq!(after.status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a1_a_post_from_another_site_is_refused(pool: PgPool) {
    use tower::ServiceExt;
    let clock = clock();
    let app = app(&pool, &clock);
    let request = axum::http::Request::builder()
        .method("POST")
        .uri("/api/login")
        .header("origin", "http://evil.test")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(r#"{"email":"ana@reyes.test"}"#))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn a1_parallel_requests_still_get_at_most_five_login_emails(pool: PgPool) {
    use balanse::balanse::presend::BalanseChecks;
    use balanse::engine::accounts::{LoginLink, LoginOutcome, Logins};
    use balanse::engine::mail::Email;

    let clock = clock();
    business(&pool, &clock).await;
    let logins = Logins::new(pool.clone(), clock.clone());
    let checks = BalanseChecks::for_approval(pool.clone(), common::links(&pool, &clock));
    let compose = |link: &LoginLink<'_>| Email {
        to_address: link.to_address.to_owned(),
        from_name: "Balanse".to_owned(),
        reply_to: None,
        subject: "Your Balanse login link".to_owned(),
        body_text: format!("Log in: http://balanse.test/login/{}", link.token),
    };

    let requests: Vec<_> = (0..12)
        .map(|_| {
            let (logins, checks) = (logins.clone(), checks.clone());
            tokio::spawn(async move { logins.send_link("ana@reyes.test", compose, &checks).await.unwrap() })
        })
        .collect();
    let mut sent = 0;
    for request in requests {
        sent += usize::from(request.await.unwrap() == LoginOutcome::Sent);
    }

    assert_eq!(sent, 5);
}
