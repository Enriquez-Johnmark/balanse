//! Debtors and their Debts (B1, B3, F3). Every endpoint is scoped by the
//! logged-in Business, so another Business's IDs are "not found" (A2).
//! Amounts appear exactly as typed and are never summed across Debtors
//! (H2, R10–R11).

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

use super::{ApiBusiness, ApiError, ApiJson, ApiMember, ApiPath};
use crate::balanse::debtors::{DebtorError, DebtorInput};
use crate::balanse::debts::{DebtError, DebtInput, parse_date};
use crate::balanse::outcomes::OutcomeError;
use crate::balanse::web::AppState;

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/debtors", get(list).post(create))
        .route("/api/debtors/{id}", get(page))
        .route("/api/debtors/{id}/debts", post(add_debt))
        .route("/api/debtors/{id}/outcome", post(outcome))
        .route("/api/debts/{id}/amount", post(change_amount))
}

/// A Debtor in the list: what it owes is counted, never summed.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DebtorRow {
    /// Its ID.
    pub id: Uuid,
    /// The client company's name.
    pub business_name: String,
    /// The Contact's name.
    pub contact_name: String,
    /// Where its reminders stand, e.g. `active` or `paused`.
    pub state: String,
    /// Until when a Promise holds, `YYYY-MM-DD`.
    pub promised_until: Option<String>,
    /// How many of its Debts are open.
    pub open_debts: i64,
    /// The earliest due date among its open Debts, `YYYY-MM-DD`.
    pub oldest_due: Option<String>,
}

async fn list(
    State(state): State<AppState>,
    ApiBusiness(business): ApiBusiness,
) -> Result<Json<Vec<DebtorRow>>, ApiError> {
    let rows = state.debtors.list(business).await.map_err(debtor_error)?;
    Ok(Json(
        rows.into_iter()
            .map(|row| DebtorRow {
                id: row.id.as_uuid(),
                business_name: row.business_name,
                contact_name: row.contact_name,
                state: row.state,
                promised_until: row.promised_until.map(iso),
                open_debts: row.open_debts,
                oldest_due: row.oldest_due.map(iso),
            })
            .collect(),
    ))
}

/// A new Debtor, as the form sends it.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct NewDebtor {
    /// The client company's name.
    pub business_name: String,
    /// The Contact's name.
    pub contact_name: String,
    /// The Contact's email address.
    pub contact_email: String,
    /// The Contact's mobile number; may be blank.
    pub contact_mobile: String,
    /// `taglish` or `english`.
    pub language: String,
}

/// The ID of something just created.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Created {
    /// Its ID.
    pub id: Uuid,
}

async fn create(
    State(state): State<AppState>,
    ApiBusiness(business): ApiBusiness,
    ApiJson(form): ApiJson<NewDebtor>,
) -> Result<(StatusCode, Json<Created>), ApiError> {
    let input = DebtorInput {
        business_name: form.business_name,
        contact_name: form.contact_name,
        contact_email: form.contact_email,
        contact_mobile: form.contact_mobile,
        language: form.language,
    };
    let id = state.debtors.create(business, &input).await.map_err(debtor_error)?;
    Ok((StatusCode::CREATED, Json(Created { id: id.as_uuid() })))
}

/// One Debtor's page: its Contact, its Debts, its recent reminders and
/// replies.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DebtorPage {
    /// Its ID.
    pub id: Uuid,
    /// The client company's name.
    pub business_name: String,
    /// The Contact's name.
    pub contact_name: String,
    /// The Contact's email address.
    pub contact_email: String,
    /// The Contact's mobile number, if given.
    pub contact_mobile: Option<String>,
    /// `taglish` or `english`.
    pub language: String,
    /// Where its reminders stand.
    pub state: String,
    /// Until when a Promise holds, `YYYY-MM-DD`.
    pub promised_until: Option<String>,
    /// Its Debts, earliest due first.
    pub debts: Vec<DebtRow>,
    /// Its reminders, newest first.
    pub reminders: Vec<ReminderRow>,
    /// Replies from its Contact, newest first.
    pub replies: Vec<ReplyRow>,
}

/// One Debt on a Debtor's page.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct DebtRow {
    /// Its ID.
    pub id: Uuid,
    /// The Business's invoice number.
    pub invoice_number: String,
    /// `YYYY-MM-DD`.
    pub invoice_date: String,
    /// Exactly as typed.
    pub amount: String,
    /// `YYYY-MM-DD`.
    pub due_date: String,
    /// The private note, if any.
    pub note: Option<String>,
    /// `open`, `payment_claimed`, `paid`, `disputed`, `written_off` or
    /// `cancelled`.
    pub status: &'static str,
}

/// One reminder on a Debtor's page.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ReminderRow {
    /// `heads_up`, `first`, `second` or `third`.
    pub step: String,
    /// The subject line.
    pub subject: String,
    /// The outbox status, e.g. `draft`, `sent` or `expired`.
    pub status: String,
    /// When it was drafted, RFC 3339.
    pub drafted_at: String,
    /// When it was sent, RFC 3339.
    pub sent_at: Option<String>,
}

/// One reply on a Debtor's page. Its text is untrusted and shown as text.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ReplyRow {
    /// Who sent it.
    pub from_address: String,
    /// Its subject.
    pub subject: String,
    /// Its text, until it is deleted after 90 days.
    pub text: Option<String>,
    /// Whether it was an automatic reply.
    pub automatic: bool,
    /// When it arrived, RFC 3339.
    pub received_at: String,
}

async fn page(
    State(state): State<AppState>,
    ApiBusiness(business): ApiBusiness,
    ApiPath(id): ApiPath<Uuid>,
) -> Result<Json<DebtorPage>, ApiError> {
    let debtor = state.debtors.get(business, id.into()).await.map_err(debtor_error)?;
    let debts = state.debts.for_debtor(business, debtor.id).await.map_err(debt_error)?;
    let reminders = sqlx::query!(
        "select r.step, m.subject, m.status, r.created_at, m.sent_at
         from reminders r join outbox_messages m on m.id = r.message_id and m.tenant_id = r.tenant_id
         where r.tenant_id = $1 and r.debtor_id = $2
         order by r.created_at desc limit 20",
        business.as_uuid(),
        id,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(ApiError::internal)?;
    let replies = sqlx::query!(
        "select from_address, subject, body_text, automatic, received_at from replies
         where tenant_id = $1 and debtor_id = $2
         order by received_at desc limit 20",
        business.as_uuid(),
        id,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(ApiError::internal)?;
    Ok(Json(DebtorPage {
        id,
        business_name: debtor.business_name,
        contact_name: debtor.contact_name,
        contact_email: debtor.contact_email,
        contact_mobile: debtor.contact_mobile,
        language: debtor.language,
        state: debtor.state,
        promised_until: debtor.promised_until.map(iso),
        debts: debts
            .into_iter()
            .map(|debt| DebtRow {
                id: debt.id.as_uuid(),
                invoice_number: debt.invoice_number,
                invoice_date: iso(debt.invoice_date),
                amount: debt.amount,
                due_date: iso(debt.due_date),
                note: debt.note,
                status: debt.status.as_str(),
            })
            .collect(),
        reminders: reminders
            .into_iter()
            .map(|row| ReminderRow {
                step: row.step,
                subject: row.subject,
                status: row.status,
                drafted_at: rfc3339(row.created_at),
                sent_at: row.sent_at.map(rfc3339),
            })
            .collect(),
        replies: replies
            .into_iter()
            .map(|row| ReplyRow {
                from_address: row.from_address,
                subject: row.subject,
                text: row.body_text,
                automatic: row.automatic,
                received_at: rfc3339(row.received_at),
            })
            .collect(),
    }))
}

/// A new Debt, as the form sends it.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct NewDebt {
    /// The Business's own invoice number.
    pub invoice_number: String,
    /// `YYYY-MM-DD`.
    pub invoice_date: String,
    /// As the Business writes it, e.g. `₱12,500.00`.
    pub amount: String,
    /// `YYYY-MM-DD`.
    pub due_date: String,
    /// A private note; may be blank.
    pub note: String,
}

async fn add_debt(
    State(state): State<AppState>,
    ApiBusiness(business): ApiBusiness,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(form): ApiJson<NewDebt>,
) -> Result<(StatusCode, Json<Created>), ApiError> {
    let input = DebtInput {
        invoice_number: form.invoice_number,
        invoice_date: form.invoice_date,
        amount: form.amount,
        due_date: form.due_date,
        note: form.note,
    };
    let debt = state
        .debts
        .create(business, id.into(), &input)
        .await
        .map_err(debt_error)?;
    Ok((StatusCode::CREATED, Json(Created { id: debt.as_uuid() })))
}

/// The amount still owed, after a partial payment or a correction.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct NewAmount {
    /// As typed, e.g. `₱5,000.00`.
    pub amount: String,
}

async fn change_amount(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(form): ApiJson<NewAmount>,
) -> Result<StatusCode, ApiError> {
    state
        .debts
        .change_amount(member.tenant, id.into(), &form.amount, &member.email)
        .await
        .map_err(debt_error)?;
    Ok(StatusCode::NO_CONTENT)
}

/// The Owner's decision about a Debtor (F3).
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct Outcome {
    /// `paid`, `promised`, `disputed` or `resume`.
    pub outcome: String,
    /// For `promised`: the date, `YYYY-MM-DD`.
    pub until: Option<String>,
}

async fn outcome(
    State(state): State<AppState>,
    ApiMember(member): ApiMember,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(form): ApiJson<Outcome>,
) -> Result<StatusCode, ApiError> {
    let (business, debtor, by) = (member.tenant, id.into(), member.email.as_str());
    let outcomes = &state.outcomes;
    let result = match form.outcome.as_str() {
        "paid" => outcomes.paid(business, debtor, by).await,
        "disputed" => outcomes.disputed(business, debtor, by).await,
        "resume" => outcomes.resume(business, debtor, by).await,
        "promised" => {
            let until = form
                .until
                .as_deref()
                .and_then(parse_date)
                .ok_or_else(|| ApiError::invalid("Give the date the Debtor promised to pay by."))?;
            outcomes.promised(business, debtor, until, by).await
        }
        _ => return Err(ApiError::invalid("Choose paid, promised, disputed or resume.")),
    };
    match result {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(OutcomeError::NotFound) => Err(ApiError::NotFound),
        Err(OutcomeError::PastDate) => Err(ApiError::invalid("The promised date must be today or later.")),
        Err(OutcomeError::Stopped) => Err(ApiError::Refused {
            status: StatusCode::CONFLICT,
            message: "Reminders to this Debtor stay off: they asked to stop, or their email bounced.".to_owned(),
            problems: Vec::new(),
        }),
        Err(err) => Err(ApiError::internal(err)),
    }
}

fn debtor_error(err: DebtorError) -> ApiError {
    match err {
        DebtorError::Invalid(problems) => ApiError::problems("Please check the Debtor's details.", problems),
        DebtorError::NotFound => ApiError::NotFound,
        DebtorError::Database(err) => ApiError::internal(err),
    }
}

fn debt_error(err: DebtError) -> ApiError {
    match err {
        DebtError::Invalid(problems) => ApiError::problems("Please check the Debt's details.", problems),
        DebtError::NotFound => ApiError::NotFound,
        DebtError::Database(err) => ApiError::internal(err),
        DebtError::Outbox(err) => ApiError::internal(err),
    }
}

fn iso(date: Date) -> String {
    date.to_string()
}

fn rfc3339(at: time::OffsetDateTime) -> String {
    at.format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_default()
}
