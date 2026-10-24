//! The public sign-up form (A3). It asks whether the business lends money;
//! lenders are refused with the reason, and nothing about them is kept
//! (R16). Other requests wait for the operator's approval; the answer is the
//! same whether or not the address already has an account.

use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use serde::Deserialize;

use super::{ApiError, ApiJson};
use crate::balanse::web::AppState;
use crate::engine::accounts::SignupError;

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/api/signup", post(request_account))
}

/// A request for an account.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct SignupRequest {
    /// The business's name, which signs its reminders.
    pub business_name: String,
    /// The owner's address, which logs in.
    pub email: String,
    /// Whether the terms and data processing agreement were accepted.
    pub accepted_terms: bool,
    /// Whether the business lends money: loans, financing, pawning or
    /// "5-6". It must be answered.
    pub lends_money: bool,
}

async fn request_account(
    State(state): State<AppState>,
    ApiJson(request): ApiJson<SignupRequest>,
) -> Result<StatusCode, ApiError> {
    if request.lends_money {
        return Err(ApiError::invalid(
            "Balanse can't be used to collect loans. Businesses that lend money, finance purchases, pawn \
             items or run informal lending follow separate collection rules, so they can't sign up.",
        ));
    }
    match state
        .signups
        .request(&request.business_name, &request.email, request.accepted_terms)
        .await
    {
        Ok(()) => Ok(StatusCode::ACCEPTED),
        Err(SignupError::Invalid(problems)) => Err(ApiError::problems("Please check the form.", problems)),
        Err(SignupError::TooMany) => Err(ApiError::Refused {
            status: StatusCode::SERVICE_UNAVAILABLE,
            message: "We're not taking new requests right now. Please try again tomorrow.".to_owned(),
            problems: Vec::new(),
        }),
        Err(err) => Err(ApiError::internal(err)),
    }
}
