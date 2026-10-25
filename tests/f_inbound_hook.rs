//! F1 through the web: the inbound provider posts each raw reply, signed,
//! to `/hooks/inbound-email`.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::http::StatusCode;
use balanse::balanse::debtors::{DebtorInput, Debtors};
use balanse::balanse::debts::{DebtInput, Debts};
use balanse::balanse::replies::Replies;
use balanse::balanse::web::{AppState, router};
use common::{BASE_URL, business, clock};
use hmac::{Hmac, KeyInit, Mac};
use sqlx::PgPool;
use tower::ServiceExt;

const SECRET: &str = "inbound-test-secret";

/// The app taking in replies, and the first reminder's reply address.
async fn world(pool: &PgPool) -> (axum::Router, String) {
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
    let reply_to: String = sqlx::query_scalar("select reply_to from outbox_messages")
        .fetch_one(pool)
        .await
        .unwrap();
    let replies = Replies::new(
        pool.clone(),
        clock.clone(),
        common::reply_addresses(),
        common::links(pool, &clock),
    );
    let state = AppState::new(pool.clone(), clock, &common::master(), BASE_URL).with_inbound(SECRET.into(), replies);
    (router(state), reply_to)
}

fn raw(to: &str) -> Vec<u8> {
    format!(
        "From: Lito <lito@santos.test>\r\nTo: {to}\r\nSubject: Re: Reminder\r\nMessage-ID: <r1@santos.test>\r\n\
         Content-Type: text/plain; charset=utf-8\r\n\r\nBabayaran ko po sa Friday.\r\n"
    )
    .into_bytes()
}

fn sign(body: &[u8]) -> String {
    let mut mac = <Hmac<sha2::Sha256> as KeyInit>::new_from_slice(SECRET.as_bytes()).unwrap();
    mac.update(body);
    mac.finalize().into_bytes().iter().map(|b| format!("{b:02x}")).collect()
}

async fn post(app: &axum::Router, body: Vec<u8>, signature: &str) -> StatusCode {
    let request = axum::http::Request::builder()
        .method("POST")
        .uri("/hooks/inbound-email")
        .header("x-balanse-signature", signature)
        .body(axum::body::Body::from(body))
        .unwrap();
    app.clone().oneshot(request).await.unwrap().status()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f1_an_unsigned_message_is_refused(pool: PgPool) {
    let (app, reply_to) = world(&pool).await;

    let status = post(&app, raw(&reply_to), "00").await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f1_a_signed_reply_pauses_the_debtor(pool: PgPool) {
    let (app, reply_to) = world(&pool).await;
    let body = raw(&reply_to);
    let signature = sign(&body);

    let status = post(&app, body, &signature).await;

    let state: String = sqlx::query_scalar("select state from debtors")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!((status, state.as_str()), (StatusCode::NO_CONTENT, "paused"));
}
