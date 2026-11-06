//! Leaving (H5): downloading all of the Business's data. It needs the
//! Owner, logged in within the last few minutes, so a stolen session can't
//! take everything.

use axum::Router;
use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;

use super::{ApiError, ApiFreshOwner};
use crate::balanse::export::business_data;
use crate::balanse::reminders::MANILA;
use crate::balanse::web::AppState;

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/api/export", get(export))
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
