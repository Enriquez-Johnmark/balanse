//! The JSON API under `/api/` that the React app calls.
//!
//! - Every Business endpoint takes an [`ApiBusiness`] from the session
//!   cookie and passes its ID to the repositories, so another Business's IDs
//!   are "not found" (A2).
//! - Changes must come from our own origin and carry a JSON body, which a
//!   cross-site form can't send.
//! - Errors are JSON ([`ErrorBody`]) without internal details.

mod debtors;
mod drafts;
mod login;
mod signup;

use axum::extract::rejection::{JsonRejection, PathRejection};
use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use axum::{Json, Router};
use serde::Serialize;
use serde::de::DeserializeOwned;

use super::{AppState, session_cookie};
use crate::balanse::businesses::BusinessId;
use crate::engine::accounts::Session;

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .merge(debtors::routes())
        .merge(drafts::routes())
        .merge(login::routes())
        .merge(signup::routes())
        .route("/api/{*rest}", any(not_found))
}

async fn not_found() -> ApiError {
    ApiError::NotFound
}

/// What every API error answers.
#[derive(Debug, Serialize)]
pub struct ErrorBody {
    /// One sentence for the Owner.
    pub message: String,
    /// Each problem found, e.g. one per invalid field; may be empty.
    pub problems: Vec<String>,
}

/// An API failure.
#[derive(Debug)]
pub enum ApiError {
    /// Missing, or another Business's (A2).
    NotFound,
    /// No live session.
    NotLoggedIn,
    /// Understood but refused, with the reason and any problems found.
    Refused {
        /// 403, 409, 422 or 503.
        status: StatusCode,
        /// One sentence for the Owner.
        message: String,
        /// Each problem, e.g. one per invalid field.
        problems: Vec<String>,
    },
    /// Logged; the Owner sees a generic error.
    Internal,
}

impl ApiError {
    /// Logs `err` and answers with a generic error.
    pub fn internal(err: impl std::fmt::Display) -> Self {
        tracing::error!(%err, "API request failed");
        Self::Internal
    }

    /// Refused as unprocessable, listing each problem.
    pub fn problems(message: impl Into<String>, problems: Vec<String>) -> Self {
        Self::Refused {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            message: message.into(),
            problems,
        }
    }

    /// Refused as unprocessable, with `message`.
    pub fn invalid(message: impl Into<String>) -> Self {
        Self::Refused {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            message: message.into(),
            problems: Vec::new(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message, problems) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found.".to_owned(), Vec::new()),
            Self::NotLoggedIn => (StatusCode::UNAUTHORIZED, "Log in first.".to_owned(), Vec::new()),
            Self::Refused {
                status,
                message,
                problems,
            } => (status, message, problems),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong. Please try again.".to_owned(),
                Vec::new(),
            ),
        };
        (status, Json(ErrorBody { message, problems })).into_response()
    }
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self::Refused {
            status: rejection.status(),
            message: "That wasn't understood; check what you typed and try again.".to_owned(),
            problems: vec![rejection.body_text()],
        }
    }
}

impl From<PathRejection> for ApiError {
    /// A malformed ID in the address names nothing: "not found" (A2).
    fn from(_: PathRejection) -> Self {
        Self::NotFound
    }
}

/// A JSON body; one that can't be read is refused as an [`ErrorBody`].
#[derive(Debug)]
pub struct ApiJson<T>(pub T);

impl<T: DeserializeOwned, S: Send + Sync> FromRequest<S> for ApiJson<T> {
    type Rejection = ApiError;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(request, state).await?;
        Ok(Self(value))
    }
}

/// Values from the path; a malformed one is "not found" (A2).
#[derive(Debug)]
pub struct ApiPath<T>(pub T);

impl<T: DeserializeOwned + Send, S: Send + Sync> FromRequestParts<S> for ApiPath<T> {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let axum::extract::Path(value) = axum::extract::Path::<T>::from_request_parts(parts, state).await?;
        Ok(Self(value))
    }
}

/// The logged-in person, owner or staff; 401 without a live session.
pub struct ApiMember(pub Session);

impl FromRequestParts<AppState> for ApiMember {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let session = session_cookie(parts).ok_or(ApiError::NotLoggedIn)?;
        match state.logins.session(&session).await {
            Ok(Some(member)) => Ok(Self(member)),
            Ok(None) => Err(ApiError::NotLoggedIn),
            Err(err) => Err(ApiError::internal(err)),
        }
    }
}

/// The logged-in person's Business.
#[derive(Debug, Clone, Copy)]
pub struct ApiBusiness(pub BusinessId);

impl FromRequestParts<AppState> for ApiBusiness {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let ApiMember(member) = ApiMember::from_request_parts(parts, state).await?;
        Ok(Self(member.tenant))
    }
}
