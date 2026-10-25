//! The Drafts page (D1, D2). Each approval binds the exact content the
//! Owner was shown, by its hash; editing voids an approval; the pre-send
//! checks run at approval and again before sending. Every endpoint is
//! scoped by the logged-in Business (A2).

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{ApiBusiness, ApiError, ApiJson, ApiMember, ApiPath};
use crate::balanse::drafts::approve_batch;
use crate::balanse::presend::BalanseChecks;
use crate::balanse::web::AppState;
use crate::engine::ids::MessageId;
use crate::engine::mail::Email;
use crate::engine::outbox::{ContentHash, OutboxError};

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/drafts", get(list))
        .route("/api/drafts/approve", post(approve))
        .route("/api/drafts/{id}", put(edit))
        .route("/api/drafts/{id}/discard", post(discard))
}

/// One reminder waiting for approval, shown in full.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DraftRow {
    /// Its ID.
    pub id: Uuid,
    /// The Debtor it goes to.
    pub debtor_id: Uuid,
    /// The Debtor's company name.
    pub debtor_name: String,
    /// `heads_up`, `first`, `second` or `third`.
    pub step: String,
    /// The Contact's address.
    pub to_address: String,
    /// The subject line.
    pub subject: String,
    /// The whole text.
    pub body: String,
    /// The hash of exactly this content; approving sends it back.
    pub hash: String,
    /// When it expires unapproved, RFC 3339.
    pub expires_at: Option<String>,
}

async fn list(
    State(state): State<AppState>,
    ApiBusiness(business): ApiBusiness,
) -> Result<Json<Vec<DraftRow>>, ApiError> {
    let rows = sqlx::query!(
        "select m.id, r.debtor_id, d.business_name, r.step, m.to_address, m.from_name, m.reply_to, m.subject,
                m.body_text, m.expires_at
         from reminders r
         join outbox_messages m on m.id = r.message_id and m.tenant_id = r.tenant_id
         join debtors d on d.id = r.debtor_id and d.tenant_id = r.tenant_id
         where r.tenant_id = $1 and m.status = 'draft'
         order by m.created_at, m.id",
        business.as_uuid(),
    )
    .fetch_all(&state.pool)
    .await
    .map_err(ApiError::internal)?;
    Ok(Json(
        rows.into_iter()
            .map(|row| {
                let email = Email {
                    to_address: row.to_address,
                    from_name: row.from_name,
                    reply_to: row.reply_to,
                    subject: row.subject,
                    body_text: row.body_text,
                };
                DraftRow {
                    id: row.id,
                    debtor_id: row.debtor_id,
                    debtor_name: row.business_name,
                    step: row.step,
                    hash: ContentHash::of(&email).to_hex(),
                    to_address: email.to_address,
                    subject: email.subject,
                    body: email.body_text,
                    expires_at: row
                        .expires_at
                        .and_then(|at| at.format(&time::format_description::well_known::Rfc3339).ok()),
                }
            })
            .collect(),
    ))
}

/// Drafts the Owner reviewed, each with the hash of what they saw.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Approval {
    /// The Drafts.
    pub drafts: Vec<Reviewed>,
}

/// One reviewed Draft.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Reviewed {
    /// Its ID.
    pub id: Uuid,
    /// The hash shown with it.
    pub hash: String,
}

/// What became of one Draft in a batch.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Approved {
    /// Its ID.
    pub id: Uuid,
    /// Whether it was approved.
    pub approved: bool,
    /// Why not, for the Owner.
    pub reason: Option<String>,
}

async fn approve(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiJson(approval): ApiJson<Approval>,
) -> Result<Json<Vec<Approved>>, ApiError> {
    let mut answers = Vec::with_capacity(approval.drafts.len());
    let mut seen = Vec::with_capacity(approval.drafts.len());
    for reviewed in approval.drafts {
        match ContentHash::from_hex(&reviewed.hash) {
            Some(hash) => seen.push((MessageId::from(reviewed.id), hash)),
            None => answers.push(Approved {
                id: reviewed.id,
                approved: false,
                reason: Some("That draft wasn't understood; reload the page.".to_owned()),
            }),
        }
    }
    let checks = BalanseChecks::for_approval(state.pool.clone(), state.links.clone());
    let results = approve_batch(&state.outbox, member.tenant, &seen, &member.email, &checks).await;
    let mut any = false;
    for (id, result) in results {
        any |= result.is_ok();
        answers.push(Approved {
            id: id.as_uuid(),
            approved: result.is_ok(),
            reason: result.err().map(|err| reason(&err)),
        });
    }
    if any {
        state.wake_sender.notify_one();
    }
    Ok(Json(answers))
}

/// Why a Draft wasn't approved, in the Owner's words.
fn reason(err: &OutboxError) -> String {
    match err {
        OutboxError::NotFound => "That draft no longer exists.".to_owned(),
        OutboxError::StaleApproval => "It changed since you looked at it; review it again.".to_owned(),
        OutboxError::CheckFailed(why) => format!("It can't go out: {why}."),
        OutboxError::CheckUnavailable(_) => "It couldn't be checked just now; try again.".to_owned(),
        OutboxError::Expired => "It expired before it was approved.".to_owned(),
        OutboxError::WrongStatus(_) => "It's no longer waiting for approval.".to_owned(),
        other => {
            tracing::error!(%other, "approving a draft failed");
            "Something went wrong; try again.".to_owned()
        }
    }
}

/// A Draft's new wording.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Edit {
    /// The subject line.
    pub subject: String,
    /// The whole text.
    pub body: String,
}

async fn edit(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(edit): ApiJson<Edit>,
) -> Result<StatusCode, ApiError> {
    let id = MessageId::from(id);
    let message = state.outbox.get(member.tenant, id).await.map_err(outbox_error)?;
    let email = Email {
        subject: edit.subject,
        body_text: edit.body,
        ..message.email
    };
    state
        .outbox
        .edit(member.tenant, id, &email, &member.email)
        .await
        .map_err(outbox_error)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn discard(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
) -> Result<StatusCode, ApiError> {
    state
        .outbox
        .cancel(member.tenant, id.into(), &member.email)
        .await
        .map_err(outbox_error)?;
    Ok(StatusCode::NO_CONTENT)
}

fn outbox_error(err: OutboxError) -> ApiError {
    match err {
        OutboxError::NotFound => ApiError::NotFound,
        OutboxError::InvalidMessage(why) => ApiError::invalid(why),
        OutboxError::WrongStatus(_) => ApiError::Refused {
            status: StatusCode::CONFLICT,
            message: "It's no longer waiting for approval.".to_owned(),
            problems: Vec::new(),
        },
        other => ApiError::internal(other),
    }
}
