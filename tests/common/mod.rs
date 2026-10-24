//! Shared test helpers.

#![allow(dead_code, reason = "each test binary uses a different subset")]

use std::sync::Arc;

use balanse::balanse::businesses::{BusinessId, create_business};
use balanse::engine::clock::FixedClock;
use sqlx::PgPool;
use time::macros::datetime;

/// A clock fixed at Monday 2026-11-02 09:00 Manila time (01:00 UTC); tests
/// move it forward by hand.
pub fn clock() -> Arc<FixedClock> {
    Arc::new(FixedClock::new(datetime!(2026-11-02 01:00 UTC)))
}

/// A new Business, "Reyes Creative", owned by ana@reyes.test.
pub async fn business(pool: &PgPool, clock: &Arc<FixedClock>) -> BusinessId {
    create_business(pool, clock.as_ref(), "Reyes Creative", "ana@reyes.test")
        .await
        .unwrap()
}

/// A mailer that records every send instead of delivering it; its provider
/// blocks duplicate keys, like Resend.
#[derive(Clone, Default)]
pub struct RecordingMailer {
    sent: Arc<std::sync::Mutex<Vec<balanse::engine::mail::Email>>>,
}

impl RecordingMailer {
    pub fn sent(&self) -> Vec<balanse::engine::mail::Email> {
        self.sent.lock().unwrap().clone()
    }
}

impl balanse::engine::mail::Mailer for RecordingMailer {
    fn blocks_duplicate_keys(&self) -> bool {
        true
    }

    async fn send(
        &self,
        email: &balanse::engine::mail::Email,
        idempotency_key: &str,
    ) -> Result<balanse::engine::mail::SendReceipt, balanse::engine::mail::MailError> {
        self.sent.lock().unwrap().push(email.clone());
        Ok(balanse::engine::mail::SendReceipt {
            provider_message_id: format!("fake-{idempotency_key}"),
        })
    }
}

/// Where the test app thinks it is served; posts must come from here.
pub const BASE_URL: &str = "http://balanse.test";

/// The web app over `pool`, at [`BASE_URL`].
pub fn app(pool: &PgPool, clock: &Arc<FixedClock>) -> axum::Router {
    balanse::balanse::web::router(balanse::balanse::web::AppState::new(
        pool.clone(),
        clock.clone(),
        BASE_URL,
    ))
}

/// A response's status and body.
pub struct Answer {
    pub status: axum::http::StatusCode,
    pub cookie: Option<String>,
    pub body: serde_json::Value,
}

/// Sends `method path` with `body` (JSON) and `cookie`, from our own origin.
pub async fn call(
    app: &axum::Router,
    method: &str,
    path: &str,
    body: Option<serde_json::Value>,
    cookie: Option<&str>,
) -> Answer {
    use tower::ServiceExt;
    let mut request = axum::http::Request::builder()
        .method(method)
        .uri(path)
        .header("origin", BASE_URL);
    if let Some(cookie) = cookie {
        request = request.header("cookie", cookie);
    }
    let request = match body {
        Some(body) => request
            .header("content-type", "application/json")
            .body(axum::body::Body::from(body.to_string())),
        None => request.body(axum::body::Body::empty()),
    }
    .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let cookie = response
        .headers()
        .get("set-cookie")
        .map(|value| value.to_str().unwrap().split(';').next().unwrap().to_owned());
    let bytes = axum::body::to_bytes(response.into_body(), 1 << 20).await.unwrap();
    let body = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
    Answer { status, cookie, body }
}

/// The token in the newest login email to `email`, waiting up to two
/// seconds for it (login emails are created after the answer).
pub async fn login_token(pool: &PgPool, email: &str) -> Option<String> {
    for _ in 0..40 {
        let body: Option<String> = sqlx::query_scalar(
            "select body_text from outbox_messages where lower(to_address) = lower($1) order by created_at desc limit 1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await
        .unwrap();
        if let Some(body) = body {
            let link = body.split_whitespace().find(|word| word.contains("/login/"))?;
            return Some(link.rsplit('/').next()?.to_owned());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    None
}

/// Logs in as `email` and returns the session cookie.
pub async fn log_in(app: &axum::Router, pool: &PgPool, email: &str) -> String {
    call(
        app,
        "POST",
        "/api/login",
        Some(serde_json::json!({ "email": email })),
        None,
    )
    .await;
    let token = login_token(pool, email).await.unwrap();
    call(app, "POST", &format!("/api/login/{token}"), None, None)
        .await
        .cookie
        .unwrap()
}
