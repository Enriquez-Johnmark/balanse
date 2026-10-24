//! Balanse's web layer: the JSON API the React app calls (`api`).
//!
//! - Every Business endpoint takes an [`api::ApiBusiness`] from the session
//!   cookie and passes its ID to the repositories, so another Business's IDs
//!   are "not found" (A2).
//! - Changes must come from our own pages: the `Origin` header has to match
//!   the configured base URL. Together with `SameSite=Lax` cookies this stops
//!   cross-site posts.

mod api;

use std::sync::Arc;

use axum::Router;
use axum::extract::{Request, State};
use axum::http::request::Parts;
use axum::http::{HeaderValue, Method, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use sqlx::PgPool;
use tokio::sync::Notify;
use tower_http::trace::TraceLayer;

use crate::balanse::debtors::Debtors;
use crate::balanse::debts::Debts;
use crate::balanse::outcomes::Outcomes;
use crate::engine::accounts::{Logins, Signups};
use crate::engine::clock::Clock;

/// The session cookie's name.
const SESSION_COOKIE: &str = "balanse_session";

/// What every handler can reach.
#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    logins: Logins,
    signups: Signups,
    debtors: Debtors,
    debts: Debts,
    outcomes: Outcomes,
    /// Where links in emails point, e.g. `https://app.balanseapp.com`.
    base_url: String,
    /// `scheme://host[:port]` of the base URL; posts must come from it.
    origin: String,
    /// Mark cookies `Secure` (when served over HTTPS).
    secure_cookies: bool,
    /// Wakes the outbox sender so a login email goes out at once.
    wake_sender: Arc<Notify>,
}

impl AppState {
    /// The app's state; links in emails point at `base_url`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, base_url: &str) -> Self {
        let base_url = base_url.trim_end_matches('/').to_owned();
        let origin = match base_url.split_once("://") {
            Some((scheme, rest)) => format!("{scheme}://{}", rest.split('/').next().unwrap_or(rest)),
            None => base_url.clone(),
        };
        Self {
            logins: Logins::new(pool.clone(), clock.clone()),
            signups: Signups::new(pool.clone(), clock.clone()),
            debtors: Debtors::new(pool.clone(), clock.clone()),
            debts: Debts::new(pool.clone(), clock.clone()),
            outcomes: Outcomes::new(pool.clone(), clock.clone()),
            secure_cookies: base_url.starts_with("https://"),
            wake_sender: Arc::new(Notify::new()),
            pool,
            clock,
            base_url,
            origin,
        }
    }

    /// Notified whenever an email is approved and should go out now.
    pub fn sender_wake_up(&self) -> Arc<Notify> {
        self.wake_sender.clone()
    }
}

/// The web app.
pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(api::routes())
        .route("/health", get(health))
        .layer(middleware::from_fn_with_state(state.clone(), same_origin_posts))
        .layer(middleware::from_fn(security_headers))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// What a page may load: our own scripts, styles, images and fonts, nothing
/// from anywhere else, and no inline scripts.
const CONTENT_SECURITY_POLICY: &str = "default-src 'none'; script-src 'self'; style-src 'self' 'unsafe-inline'; \
     img-src 'self'; font-src 'self'; connect-src 'self'; manifest-src 'self'; form-action 'self'; \
     frame-ancestors 'none'; base-uri 'none'";

/// Security headers on every response.
async fn security_headers(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(CONTENT_SECURITY_POLICY),
    );
    headers.insert(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(header::REFERRER_POLICY, HeaderValue::from_static("same-origin"));
    response
}

/// "ok" while the database answers, for an uptime monitor.
async fn health(State(state): State<AppState>) -> Response {
    match sqlx::query_scalar!(r#"select 1 as "one!""#)
        .fetch_one(&state.pool)
        .await
    {
        Ok(_) => "ok".into_response(),
        Err(err) => {
            tracing::error!(%err, "health check: the database didn't answer");
            (StatusCode::SERVICE_UNAVAILABLE, "unavailable").into_response()
        }
    }
}

/// Refuses changes whose `Origin` isn't our own.
async fn same_origin_posts(State(state): State<AppState>, request: Request, next: Next) -> Response {
    // Machine endpoints prove themselves with signatures instead.
    let safe = matches!(*request.method(), Method::GET | Method::HEAD) || request.uri().path().starts_with("/hooks/");
    let ours = request
        .headers()
        .get(header::ORIGIN)
        .is_some_and(|origin| origin.as_bytes() == state.origin.as_bytes());
    if safe || ours {
        next.run(request).await
    } else {
        (StatusCode::FORBIDDEN, "Forbidden").into_response()
    }
}

/// The session secret from the request's cookies.
fn session_cookie(parts: &Parts) -> Option<String> {
    parts
        .headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .filter_map(|pair| pair.trim().split_once('='))
        .find(|(name, _)| *name == SESSION_COOKIE)
        .map(|(_, value)| value.to_owned())
}
