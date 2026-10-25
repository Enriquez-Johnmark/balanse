//! Balanse's web layer: the JSON API the React app calls (`api`) and the
//! built React app itself (`spa`), and the Debtor's page (`debtor_page`),
//! server-rendered and opened by a signed link.
//!
//! - Every Business endpoint takes an [`api::ApiBusiness`] from the session
//!   cookie and passes its ID to the repositories, so another Business's IDs
//!   are "not found" (A2).
//! - Changes must come from our own pages: the `Origin` header has to match
//!   the configured base URL. Together with `SameSite=Lax` cookies this stops
//!   cross-site posts.

mod api;
mod debtor_page;
mod hooks;
mod spa;

use std::sync::Arc;

use axum::Router;
use axum::extract::{Request, State};
use axum::http::request::Parts;
use axum::http::{HeaderValue, Method, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use secrecy::SecretString;
use sqlx::PgPool;
use tokio::sync::Notify;
use tower_http::trace::TraceLayer;

use crate::balanse::contact_actions::ContactActions;
use crate::balanse::debtor_links::DebtorLinks;
use crate::balanse::debtors::Debtors;
use crate::balanse::debts::Debts;
use crate::balanse::outcomes::Outcomes;
use crate::balanse::replies::Replies;
use crate::engine::accounts::{Logins, Signups};
use crate::engine::clock::Clock;
use crate::engine::files::MasterKey;
use crate::engine::outbox::Outbox;

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
    outbox: Outbox,
    links: DebtorLinks,
    actions: ContactActions,
    /// Where links in emails point, e.g. `https://app.balanseapp.com`.
    base_url: String,
    /// `scheme://host[:port]` of the base URL; posts must come from it.
    origin: String,
    /// Mark cookies `Secure` (when served over HTTPS).
    secure_cookies: bool,
    /// Wakes the outbox sender so a login email goes out at once.
    wake_sender: Arc<Notify>,
    /// Inbound email, when `INBOUND_SECRET` is set.
    inbound: Option<Arc<Inbound>>,
    /// The built React app (`WEB_DIR`), when served by this server.
    web_dir: Option<Arc<std::path::PathBuf>>,
}

/// Taking in replies.
struct Inbound {
    secret: SecretString,
    replies: Replies,
}

impl AppState {
    /// The app's state; links in emails point at `base_url`, and links to
    /// Debtor pages are signed with a key derived from `master`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, master: &MasterKey, base_url: &str) -> Self {
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
            outbox: Outbox::new(pool.clone(), clock.clone()),
            links: DebtorLinks::new(pool.clone(), clock.clone(), master, &base_url),
            actions: ContactActions::new(
                pool.clone(),
                clock.clone(),
                DebtorLinks::new(pool.clone(), clock.clone(), master, &base_url),
            ),
            secure_cookies: base_url.starts_with("https://"),
            wake_sender: Arc::new(Notify::new()),
            inbound: None,
            web_dir: None,
            pool,
            clock,
            base_url,
            origin,
        }
    }

    /// Takes in replies signed with `secret`.
    #[must_use]
    pub fn with_inbound(mut self, secret: SecretString, replies: Replies) -> Self {
        self.inbound = Some(Arc::new(Inbound { secret, replies }));
        self
    }

    /// Serves the built React app from `dir` (`web/dist`).
    #[must_use]
    pub fn with_web_dir(mut self, dir: &std::path::Path) -> Self {
        if !spa::is_built(dir) {
            tracing::warn!(dir = %dir.display(), "the React app isn't built there; run `pnpm --dir web build`");
        }
        self.web_dir = Some(Arc::new(dir.to_owned()));
        self
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
        .merge(hooks::routes())
        .merge(debtor_page::routes())
        .route("/static/balanse.css", get(stylesheet))
        .route("/health", get(health))
        .fallback(spa::app)
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

/// The Debtor page's stylesheet (`static/balanse.css`), built into the binary.
async fn stylesheet() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        include_str!("../../../static/balanse.css"),
    )
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
