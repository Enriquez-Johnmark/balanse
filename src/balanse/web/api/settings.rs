//! The Business's settings. For now, the Owner's payment instructions:
//! their own words, shown on every Debtor's page (R12). Balanse adds no
//! payment details of its own, and changing them needs a fresh login, so a
//! stolen session can't redirect Debtors' money (G6).

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, put};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{ApiBusiness, ApiError, ApiFreshOwner, ApiJson};
use crate::balanse::web::AppState;
use crate::engine::audit::{self, AuditEvent};

/// The longest payment instructions accepted, in characters.
pub const MAX_INSTRUCTIONS_CHARS: usize = 1_000;

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/settings", get(show))
        .route("/api/settings/payment-instructions", put(change_instructions))
}

/// The Business's settings.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Settings {
    /// How Debtors pay, in the Owner's words; may be empty.
    pub payment_instructions: String,
}

async fn show(State(state): State<AppState>, ApiBusiness(business): ApiBusiness) -> Result<Json<Settings>, ApiError> {
    let payment_instructions = sqlx::query_scalar!(
        "select payment_instructions from businesses where tenant_id = $1",
        business.as_uuid(),
    )
    .fetch_one(&state.pool)
    .await
    .map_err(ApiError::internal)?;
    Ok(Json(Settings { payment_instructions }))
}

/// New payment instructions.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct NewInstructions {
    /// How Debtors pay, in the Owner's words; may be empty.
    pub payment_instructions: String,
}

async fn change_instructions(
    State(state): State<AppState>,
    ApiFreshOwner(owner): ApiFreshOwner,
    ApiJson(form): ApiJson<NewInstructions>,
) -> Result<StatusCode, ApiError> {
    let text = form.payment_instructions.trim();
    if text.chars().count() > MAX_INSTRUCTIONS_CHARS {
        return Err(ApiError::invalid("Keep the payment instructions to 1,000 characters."));
    }
    if text.chars().any(|c| c.is_control() && c != '\n') {
        return Err(ApiError::invalid(
            "The payment instructions can't contain special characters.",
        ));
    }
    let now = state.clock.now();
    let mut tx = state.pool.begin().await.map_err(ApiError::internal)?;
    let before = sqlx::query_scalar!(
        "select payment_instructions from businesses where tenant_id = $1 for update",
        owner.tenant.as_uuid(),
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(ApiError::internal)?;
    sqlx::query!(
        "update businesses set payment_instructions = $2 where tenant_id = $1",
        owner.tenant.as_uuid(),
        text,
    )
    .execute(&mut *tx)
    .await
    .map_err(ApiError::internal)?;
    let event = AuditEvent {
        tenant: owner.tenant,
        at: now,
        actor: &owner.email,
        action: "business.payment_instructions_changed",
        subject_type: "business",
        subject_id: owner.tenant.as_uuid(),
        details: json!({ "from": before, "to": text }),
    };
    audit::record(&mut *tx, &event).await.map_err(ApiError::internal)?;
    tx.commit().await.map_err(ApiError::internal)?;
    Ok(StatusCode::NO_CONTENT)
}
