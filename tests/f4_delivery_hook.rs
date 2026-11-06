//! F4 through the web: Resend posts each delivery event, signed the Svix
//! way, to `/hooks/resend`.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use balanse::balanse::bounces::Bounces;
use balanse::balanse::web::{AppState, router};
use balanse::engine::clock::{Clock, FixedClock};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use common::{BASE_URL, clock, master, sent};
use hmac::{Hmac, KeyInit, Mac};
use secrecy::SecretString;
use sqlx::PgPool;
use tower::ServiceExt;

const KEY: &[u8] = b"resend-webhook-test-key";

fn app(pool: &PgPool, clock: &Arc<FixedClock>, with_secret: bool) -> axum::Router {
    let mut state = AppState::new(pool.clone(), clock.clone(), &master(), BASE_URL);
    if with_secret {
        let secret = SecretString::from(format!("whsec_{}", STANDARD.encode(KEY)));
        state = state.with_delivery_events(secret, Bounces::new(pool.clone(), clock.clone()));
    }
    router(state)
}

/// Posts `body` to the hook, signed with `key` as Svix signs it.
async fn post(app: axum::Router, clock: &FixedClock, body: &str, key: &[u8]) -> u16 {
    let (id, timestamp) = ("msg_1", clock.now().unix_timestamp().to_string());
    let mut mac = <Hmac<sha2::Sha256> as KeyInit>::new_from_slice(key).unwrap();
    mac.update(format!("{id}.{timestamp}.{body}").as_bytes());
    let signature = format!("v1,{}", STANDARD.encode(mac.finalize().into_bytes()));
    let request = axum::http::Request::post("/hooks/resend")
        .header("svix-id", id)
        .header("svix-timestamp", timestamp)
        .header("svix-signature", signature)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(body.to_owned()))
        .unwrap();
    app.oneshot(request).await.unwrap().status().as_u16()
}

const BOUNCE: &str =
    r#"{"type":"email.bounced","data":{"email_id":"re_1","bounce":{"type":"Permanent","message":"no such mailbox"}}}"#;

async fn state(pool: &PgPool) -> String {
    sqlx::query_scalar("select state from debtors")
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f4_a_signed_bounce_stops_the_debtors_reminders(pool: PgPool) {
    let clock = clock();
    sent(&pool, &clock).await;

    let status = post(app(&pool, &clock, true), &clock, BOUNCE, KEY).await;

    assert_eq!((status, state(&pool).await.as_str()), (204, "bounced"));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f4_a_badly_signed_event_changes_nothing(pool: PgPool) {
    let clock = clock();
    sent(&pool, &clock).await;

    let status = post(app(&pool, &clock, true), &clock, BOUNCE, b"someone-elses-key").await;

    assert_eq!((status, state(&pool).await.as_str()), (401, "active"));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f4_without_a_secret_the_hook_doesnt_exist(pool: PgPool) {
    let clock = clock();

    let status = post(app(&pool, &clock, false), &clock, BOUNCE, KEY).await;

    assert_eq!(status, 404);
}
