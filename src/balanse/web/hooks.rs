//! Machine endpoints under `/hooks/`, called by the email provider rather
//! than by a browser. Each request proves where it came from with a
//! signature over its body; the same-origin check for pages doesn't apply.
//!
//! - `POST /hooks/inbound-email`: one raw inbound email (F1), signed with
//!   `X-Balanse-Signature: <hex HMAC-SHA256 of the body>` under
//!   `INBOUND_SECRET`. Without that setting the endpoint doesn't exist.
//! - `POST /hooks/resend`: Resend's delivery events (F4), signed the Svix
//!   way under `RESEND_WEBHOOK_SECRET`. Without it, the same.

use axum::Router;
use axum::body::Bytes;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use secrecy::ExposeSecret;

use super::AppState;
use crate::balanse::replies::ReplyOutcome;
use crate::engine::inbound::{self, InboundError, MAX_RAW_BYTES};
use crate::engine::webhooks::{self, Signature};

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/hooks/inbound-email", post(inbound_email))
        .route("/hooks/resend", post(delivery_event))
        .layer(DefaultBodyLimit::max(MAX_RAW_BYTES))
}

async fn inbound_email(State(state): State<AppState>, headers: HeaderMap, body: Bytes) -> Response {
    let Some(inbound) = &state.inbound else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let signature = headers
        .get("x-balanse-signature")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    if !inbound::signed(inbound.secret.expose_secret().as_bytes(), &body, signature) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let email = match inbound::parse(&body) {
        Ok(email) => email,
        Err(InboundError::TooLarge) => return StatusCode::PAYLOAD_TOO_LARGE.into_response(),
        Err(InboundError::Unreadable) => return StatusCode::BAD_REQUEST.into_response(),
    };
    match inbound.replies.receive(&email).await {
        Ok(outcome) => {
            if outcome == ReplyOutcome::Paused {
                // The Owner's notice goes out now, not at the next drain.
                state.wake_sender.notify_one();
            }
            StatusCode::NO_CONTENT.into_response()
        }
        Err(err) => {
            tracing::error!(%err, "taking in an inbound email failed");
            // The provider retries a failed delivery.
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}

async fn delivery_event(State(state): State<AppState>, headers: HeaderMap, body: Bytes) -> Response {
    let Some(events) = &state.delivery_events else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let header = |name: &str| {
        headers
            .get(name)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default()
    };
    let signature = Signature {
        id: header("svix-id"),
        timestamp: header("svix-timestamp"),
        signatures: header("svix-signature"),
    };
    if !webhooks::verified(events.secret.expose_secret(), signature, &body, state.clock.now()) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let Some(event) = webhooks::parse(&body) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    match events.bounces.record(&event).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => {
            tracing::error!(%err, "recording a delivery event failed");
            // The provider retries a failed delivery.
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}
