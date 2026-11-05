//! Copy and open (H4): a message's texts for Viber, Messenger, WhatsApp
//! and SMS, and "I sent it on …". Reading the texts changes nothing; only
//! the outbox approves or sends. Only a Draft or approved reminder that
//! still passes the checks at approval is given out, so nothing Balanse
//! wouldn't send by email (a threat, a Debtor who asked to stop) can be
//! copied either. Scoped by the logged-in Business (A2).

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use time::macros::{format_description, time};
use time::{Date, Duration, PrimitiveDateTime};
use uuid::Uuid;

use super::{ApiBusiness, ApiError, ApiJson, ApiMember, ApiPath};
use crate::balanse::copy::{for_sms, whatsapp_link};
use crate::balanse::debtors::DebtorError;
use crate::balanse::presend::BalanseChecks;
use crate::balanse::reminders::{MANILA, log_manual_send, open_debts};
use crate::balanse::web::AppState;
use crate::balanse::wording::debt_block;
use crate::engine::mail::Email;
use crate::engine::outbox::{CheckFailed, PreSendCheck};

/// How far back a message sent by hand may be logged.
const LOG_BACK: Duration = Duration::days(30);

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/messages/{id}/copy", get(copy))
        .route("/api/debtors/{id}/sent-by-hand", post(sent_by_hand))
}

/// A reminder's texts to paste elsewhere.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct CopyTexts {
    /// For Viber or Messenger: the whole text, with the Debtor's page link.
    pub text: String,
    /// For SMS: no link and no phone number (R13).
    pub sms: String,
    /// Opens WhatsApp with the text filled in, if the Contact has a mobile.
    pub whatsapp_url: Option<String>,
}

async fn copy(
    State(state): State<AppState>,
    ApiBusiness(business): ApiBusiness,
    ApiPath(id): ApiPath<Uuid>,
) -> Result<Json<CopyTexts>, ApiError> {
    let row = sqlx::query!(
        "select r.debtor_id, d.contact_mobile, m.status, m.to_address, m.from_name, m.reply_to, m.subject,
                m.body_text
         from reminders r
         join outbox_messages m on m.id = r.message_id and m.tenant_id = r.tenant_id
         join debtors d on d.id = r.debtor_id and d.tenant_id = r.tenant_id
         where r.tenant_id = $1 and m.id = $2",
        business.as_uuid(),
        id,
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(ApiError::internal)?
    .ok_or(ApiError::NotFound)?;
    let refused = |reason: &str| ApiError::Refused {
        status: StatusCode::CONFLICT,
        message: format!("This message can't be copied: {reason}."),
        problems: Vec::new(),
    };
    if !matches!(row.status.as_str(), "draft" | "approved") {
        return Err(refused("only drafts and approved reminders can be"));
    }
    let email = Email {
        to_address: row.to_address,
        from_name: row.from_name,
        reply_to: row.reply_to,
        subject: row.subject,
        body_text: row.body_text,
    };
    BalanseChecks::for_approval(state.pool.clone(), state.links.clone())
        .check(business, id.into(), &email)
        .await
        .map_err(|failed| match failed {
            CheckFailed::Refused(reason) => refused(&reason),
            // Unavailable, or a time check, which approval's rules don't run.
            other => ApiError::internal(format!("{other:?}")),
        })?;
    let body = email.body_text;
    let debts = open_debts(&state.pool, business, row.debtor_id.into())
        .await
        .map_err(ApiError::internal)?;
    let invoice_lines: Vec<String> = debt_block(&debts).lines().map(str::to_owned).collect();
    Ok(Json(CopyTexts {
        sms: for_sms(&body, &invoice_lines),
        whatsapp_url: row
            .contact_mobile
            .as_deref()
            .and_then(|mobile| whatsapp_link(mobile, &body)),
        text: body,
    }))
}

/// "I sent it on …", as the page sends it.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct SentByHand {
    /// The day it was sent, `YYYY-MM-DD`, Manila time.
    pub sent_on: String,
}

async fn sent_by_hand(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(input): ApiJson<SentByHand>,
) -> Result<StatusCode, ApiError> {
    let business = member.tenant;
    let debtor = id.into();
    state.debtors.get(business, debtor).await.map_err(|err| match err {
        DebtorError::NotFound => ApiError::NotFound,
        other => ApiError::internal(other),
    })?;
    let now = state.clock.now();
    let today = now.to_offset(MANILA).date();
    let day = Date::parse(input.sent_on.trim(), format_description!("[year]-[month]-[day]"))
        .map_err(|_| ApiError::invalid("Choose the day you sent it."))?;
    if day > today || day < today - LOG_BACK {
        return Err(ApiError::invalid("Choose a day in the last 30 days, up to today."));
    }
    // Today means now; an earlier day, its midday.
    let sent_at = if day == today {
        now
    } else {
        PrimitiveDateTime::new(day, time!(12:00)).assume_offset(MANILA)
    };
    log_manual_send(&state.pool, business, debtor, sent_at, &member.email, now)
        .await
        .map_err(ApiError::internal)?;
    Ok(StatusCode::NO_CONTENT)
}
