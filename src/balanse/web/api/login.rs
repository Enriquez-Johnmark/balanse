//! Logging in by email link (A1). The React app shows the link's page with
//! a "Log in" button; only pressing it posts here and uses the link, so
//! email link scanners can't use it up.

use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use super::{ApiError, ApiJson, ApiMember, ApiPath};
use crate::balanse::presend::BalanseChecks;
use crate::balanse::web::{AppState, SESSION_COOKIE, session_cookie};
use crate::engine::accounts::{Accounts, LoginLink, LoginOutcome, SESSION_LIFETIME};
use crate::engine::mail::Email;

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/login", post(request_link))
        .route("/api/login/{token}", post(use_link))
        .route("/api/logout", post(log_out))
        .route("/api/me", get(me))
}

/// Asking for a login link.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// The address to send it to.
    pub email: String,
}

async fn request_link(State(state): State<AppState>, ApiJson(request): ApiJson<LoginRequest>) -> StatusCode {
    // The lookup and the email happen after the answer, so neither the
    // answer nor how long it takes reveals who has an account (A1).
    tokio::spawn(async move {
        let checks = BalanseChecks::for_approval(state.pool.clone());
        let base_url = state.base_url.clone();
        let compose = |link: &LoginLink<'_>| login_email(&base_url, link);
        match state.logins.send_link(&request.email, compose, &checks).await {
            Ok(LoginOutcome::Sent) => state.wake_sender.notify_one(),
            Ok(LoginOutcome::NotSent) => {}
            Err(err) => tracing::error!(%err, "sending a login link failed"),
        }
    });
    StatusCode::ACCEPTED
}

/// The login email; the engine sends it to the member's own address.
fn login_email(base_url: &str, link: &LoginLink<'_>) -> Email {
    Email {
        to_address: link.to_address.to_owned(),
        from_name: "Balanse".to_owned(),
        reply_to: None,
        subject: "Your Balanse login link".to_owned(),
        body_text: format!(
            "Hi,\n\nOpen this link to log in to {} on Balanse:\n\n{base_url}/login/{}\n\nIt works once, for the \
             next 15 minutes. After that, ask for a new one on Balanse's login page. If you didn't expect this \
             email, ignore it.\n",
            link.account_name, link.token,
        ),
    }
}

async fn use_link(State(state): State<AppState>, ApiPath(token): ApiPath<String>) -> Result<Response, ApiError> {
    let Some(session) = state.logins.log_in(&token).await.map_err(ApiError::internal)? else {
        return Err(ApiError::invalid(
            "This login link has expired or was already used. Ask for a new one.",
        ));
    };
    let secure = if state.secure_cookies { "; Secure" } else { "" };
    let cookie = format!(
        "{SESSION_COOKIE}={session}; Path=/; Max-Age={}; HttpOnly; SameSite=Lax{secure}",
        SESSION_LIFETIME.whole_seconds()
    );
    Ok(([(header::SET_COOKIE, cookie)], Json(serde_json::json!({}))).into_response())
}

async fn log_out(State(state): State<AppState>, request: axum::extract::Request) -> Result<Response, ApiError> {
    let (parts, _) = request.into_parts();
    if let Some(session) = session_cookie(&parts) {
        state.logins.log_out(&session).await.map_err(ApiError::internal)?;
    }
    let cookie = format!("{SESSION_COOKIE}=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax");
    Ok(([(header::SET_COOKIE, cookie)], StatusCode::NO_CONTENT).into_response())
}

/// Who is logged in.
#[derive(Debug, Serialize)]
pub struct Me {
    /// Their Business's name.
    pub business_name: String,
    /// Their address.
    pub email: String,
    /// `owner` or `staff`.
    pub role: &'static str,
}

async fn me(State(state): State<AppState>, ApiMember(member): ApiMember) -> Result<Json<Me>, ApiError> {
    let account = Accounts::new(state.pool.clone(), state.clock.clone())
        .get(member.tenant)
        .await
        .map_err(ApiError::internal)?;
    Ok(Json(Me {
        business_name: account.name,
        email: member.email,
        role: member.role.as_str(),
    }))
}
