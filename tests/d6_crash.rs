//! D6: a crash in the middle of sending never sends a message twice. The
//! real `balanse` binary is killed (`abort()`) after the provider accepted
//! an email but before the send was recorded, then run again. Needs
//! `--features failpoints`.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::collections::HashMap;
use std::process::{Command, ExitStatus};
use std::sync::{Arc, Mutex};

use balanse::engine::ids::{MessageId, TenantId};
use balanse::engine::mail::Email;
use balanse::engine::outbox::{MessageStatus, NoExtraChecks, Outbox};
use common::{business, clock};
use serde_json::json;
use sqlx::PgPool;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

const FAILPOINT: &str = "after_send_before_record";
const BASE_URL: &str = "http://balanse.test";

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d6_crash_mid_send_with_resend_delivers_exactly_one_email(pool: PgPool) {
    let resend = FakeResend::default();
    let provider = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/emails"))
        .respond_with(resend.clone())
        .mount(&provider)
        .await;
    let (tenant, id) = approved(&pool).await;

    let crashed = drain(&pool, &resend_env(&provider), Some(FAILPOINT)).await;
    assert!(!crashed.success(), "the failpoint should have killed the process");
    drain(&pool, &resend_env(&provider), None).await;

    assert_eq!(resend.delivered(), 1, "Resend would have delivered this many emails");
    assert_eq!(status(&pool, tenant, id).await, MessageStatus::Sent);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d6_crash_mid_send_without_duplicate_blocking_never_resends(pool: PgPool) {
    let provider = fake_provider("/api/v1/send", json!({ "ID": "mp_1" })).await;
    let (tenant, id) = approved(&pool).await;

    drain(&pool, &mailpit_env(&provider), Some(FAILPOINT)).await;
    drain(&pool, &mailpit_env(&provider), None).await;

    assert_eq!(
        provider.received_requests().await.unwrap().len(),
        1,
        "the email was sent twice"
    );
    assert_eq!(status(&pool, tenant, id).await, MessageStatus::Unconfirmed);
}

/// Behaves like Resend's idempotency: a repeat of a key with the same body
/// returns the first result without sending again; a repeat with a different
/// body is refused with 409. Counts the emails it would actually deliver.
#[derive(Clone, Default)]
struct FakeResend {
    seen: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl FakeResend {
    fn delivered(&self) -> usize {
        self.seen.lock().unwrap().len()
    }
}

impl Respond for FakeResend {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        let key = request
            .headers
            .get("Idempotency-Key")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_owned();
        let mut seen = self.seen.lock().unwrap();
        match seen.get(&key) {
            Some(body) if *body != request.body => ResponseTemplate::new(409)
                .set_body_json(json!({ "name": "invalid_idempotent_request", "message": "different payload" })),
            Some(_) => ResponseTemplate::new(200).set_body_json(json!({ "id": format!("re_{key}") })),
            None => {
                seen.insert(key.clone(), request.body.clone());
                ResponseTemplate::new(200).set_body_json(json!({ "id": format!("re_{key}") }))
            }
        }
    }
}

async fn fake_provider(route: &str, body: serde_json::Value) -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(route))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;
    server
}

/// An approved login email to the Owner: a message the real binary's
/// pre-send checks accept at any hour, so the crash is what's under test.
async fn approved(pool: &PgPool) -> (TenantId, MessageId) {
    let clock = clock();
    let outbox = Outbox::new(pool.clone(), clock.clone());
    let tenant = business(pool, &clock).await;
    let login = Email {
        to_address: "ana@reyes.test".to_owned(),
        from_name: "Balanse".to_owned(),
        reply_to: None,
        subject: "Your Balanse login link".to_owned(),
        body_text: format!("Open this link to log in:\n\n{BASE_URL}/login/abc123\n"),
    };
    let id = outbox.create_draft(tenant, &login).await.unwrap();
    let seen = outbox.get(tenant, id).await.unwrap().content_hash();
    outbox
        .approve(tenant, id, "system:login", &seen, &NoExtraChecks)
        .await
        .unwrap();
    (tenant, id)
}

async fn status(pool: &PgPool, tenant: TenantId, id: MessageId) -> MessageStatus {
    Outbox::new(pool.clone(), clock()).get(tenant, id).await.unwrap().status
}

fn resend_env(provider: &MockServer) -> Vec<(&'static str, String)> {
    vec![
        ("MAIL_PROVIDER", "resend".to_owned()),
        ("RESEND_BASE_URL", provider.uri()),
        ("RESEND_API_KEY", "re_test".to_owned()),
    ]
}

fn mailpit_env(provider: &MockServer) -> Vec<(&'static str, String)> {
    vec![("MAIL_PROVIDER", "mailpit".to_owned()), ("MAILPIT_URL", provider.uri())]
}

/// Runs `balanse outbox-drain` against this test's own database.
async fn drain(pool: &PgPool, env: &[(&'static str, String)], failpoint: Option<&str>) -> ExitStatus {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_balanse"));
    cmd.arg("outbox-drain")
        .env("DATABASE_URL", database_url(pool).await)
        .env("MAIL_FROM_ADDRESS", "reminders@balanseapp.test")
        .env("PUBLIC_BASE_URL", BASE_URL)
        .env("MASTER_KEY", "ab".repeat(32))
        .env("RUST_LOG", "warn")
        .envs(env.iter().map(|(k, v)| (*k, v.as_str())));
    match failpoint {
        Some(name) => cmd.env("NUDGEWRIGHT_FAILPOINT", name),
        None => cmd.env_remove("NUDGEWRIGHT_FAILPOINT"),
    };
    tokio::task::spawn_blocking(move || cmd.status().unwrap())
        .await
        .unwrap()
}

/// `#[sqlx::test]` gives each test a fresh database on the server named by
/// `DATABASE_URL`; point the child process at that same database.
async fn database_url(pool: &PgPool) -> String {
    let name: String = sqlx::query_scalar("select current_database()")
        .fetch_one(pool)
        .await
        .unwrap();
    let base = std::env::var("DATABASE_URL").unwrap();
    let (server, _) = base.rsplit_once('/').unwrap();
    format!("{server}/{name}")
}
