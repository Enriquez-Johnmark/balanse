//! The Owner's side of a claim of payment (G2, G5): confirm it, reject it,
//! or download a proof file. Every endpoint is scoped by the logged-in
//! Business, so another Business's claims and files are "not found" (A2).

use axum::Router;
use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use uuid::Uuid;

use super::{ApiBusiness, ApiError, ApiMember, ApiPath};
use crate::balanse::claims::ClaimError;
use crate::balanse::proofs::ProofError;
use crate::balanse::web::AppState;

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/claims/{id}/confirm", post(confirm))
        .route("/api/claims/{id}/reject", post(reject))
        .route("/api/claims/{claim}/files/{file}", get(download))
}

async fn confirm(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
) -> Result<StatusCode, ApiError> {
    state
        .claims
        .confirm(member.tenant, id.into(), &member.email)
        .await
        .map_err(claim_error)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn reject(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
) -> Result<StatusCode, ApiError> {
    state
        .claims
        .reject(member.tenant, id.into(), &member.email)
        .await
        .map_err(claim_error)?;
    Ok(StatusCode::NO_CONTENT)
}

/// The file, as an attachment with a name Balanse made, so nothing a
/// Contact sent is ever shown in the browser or named by them (G5).
async fn download(
    State(state): State<AppState>,
    ApiBusiness(business): ApiBusiness,
    ApiPath((claim, file)): ApiPath<(Uuid, Uuid)>,
) -> Result<Response, ApiError> {
    let proofs = state.proofs.as_ref().ok_or(ApiError::NotFound)?;
    let debtor_name = sqlx::query_scalar!(
        "select d.business_name from claims c
         join claim_files f on f.claim_id = c.id and f.tenant_id = c.tenant_id
         join debtors d on d.id = c.debtor_id and d.tenant_id = c.tenant_id
         where c.tenant_id = $1 and c.id = $2 and f.id = $3",
        business.as_uuid(),
        claim,
        file,
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(ApiError::internal)?
    .ok_or(ApiError::NotFound)?;
    let download = match proofs.download(business, file.into(), &debtor_name).await {
        Ok(download) => download,
        Err(ProofError::NotFound) => return Err(ApiError::NotFound),
        Err(err) => return Err(ApiError::internal(err)),
    };
    Ok((
        [
            (header::CONTENT_TYPE, download.kind.media_type().to_owned()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", download.name),
            ),
            (header::X_CONTENT_TYPE_OPTIONS, "nosniff".to_owned()),
            (header::CACHE_CONTROL, "no-store".to_owned()),
        ],
        download.bytes,
    )
        .into_response())
}

fn claim_error(err: ClaimError) -> ApiError {
    match err {
        ClaimError::NotFound => ApiError::NotFound,
        ClaimError::AlreadySettled => ApiError::Refused {
            status: StatusCode::CONFLICT,
            message: "This claim was already answered.".to_owned(),
            problems: Vec::new(),
        },
        other => ApiError::internal(other),
    }
}
