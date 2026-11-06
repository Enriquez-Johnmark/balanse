//! Leaving (H5): downloading all of the Business's data, and closing the
//! account, which stops everything at once and erases it after the
//! engine's grace period (7 days, inside the 30 the spec allows). Both need
//! the Owner, logged in within the last few minutes, so a stolen session
//! can't take or delete everything.

use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use super::{ApiError, ApiFreshOwner, ApiJson};
use crate::balanse::export::business_data;
use crate::balanse::reminders::MANILA;
use crate::balanse::web::AppState;
use crate::balanse::web::SESSION_COOKIE;
use crate::engine::accounts::{Accounts, CloseError};

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/export", get(export))
        .route("/api/account/close", post(close))
}

/// Everything, as one JSON file.
async fn export(
    State(state): State<AppState>,
    ApiFreshOwner(owner): ApiFreshOwner,
) -> Result<impl IntoResponse, ApiError> {
    let data = business_data(&state.pool, owner.tenant)
        .await
        .map_err(ApiError::internal)?;
    let today = state.clock.now().to_offset(MANILA).date();
    let body = serde_json::to_string_pretty(&data).map_err(ApiError::internal)?;
    Ok((
        [
            (header::CONTENT_TYPE, "application/json".to_owned()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"balanse-data-{today}.json\""),
            ),
        ],
        body,
    ))
}

/// Closing the account, confirmed by typing its name.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CloseAccount {
    /// The Business's name, as typed.
    pub typed_name: String,
}

/// When a closed account is erased.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Closed {
    /// RFC 3339.
    pub erase_after: String,
}

async fn close(
    State(state): State<AppState>,
    ApiFreshOwner(owner): ApiFreshOwner,
    ApiJson(form): ApiJson<CloseAccount>,
) -> Result<impl IntoResponse, ApiError> {
    let erase_after = Accounts::new(state.pool.clone(), state.clock.clone())
        .close(owner.tenant, &form.typed_name, &owner.email)
        .await
        .map_err(|err| match err {
            CloseError::NameMismatch => {
                ApiError::invalid("Type your business name exactly as it is to close the account.")
            }
            CloseError::NotFound => ApiError::NotFound,
            other => ApiError::internal(other),
        })?;
    let erase_after = erase_after
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(ApiError::internal)?;
    // Every session already ended; the browser forgets this one too.
    let cookie = format!("{SESSION_COOKIE}=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax");
    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(Closed { erase_after }),
    ))
}
