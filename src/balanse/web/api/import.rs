//! Importing Debts from a spreadsheet (B2): the Owner uploads a CSV or
//! Excel file and sees what was added and which rows weren't, with why.
//! Scoped by the logged-in Business (A2). Excel files are parsed by the
//! engine in a sandboxed child process.

use axum::extract::multipart::MultipartError;
use axum::extract::{DefaultBodyLimit, Multipart, State};
use axum::http::StatusCode;
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;

use super::{ApiBusiness, ApiError};
use crate::balanse::import::{COLUMNS, ImportError, Importer};
use crate::balanse::web::AppState;
use crate::engine::sheets::{MAX_BYTES, SheetError, numbered, read_csv};

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/import",
            post(import).layer(DefaultBodyLimit::max(MAX_BYTES + 64 * 1024)),
        )
        .route("/api/import/template", get(template))
}

/// What an import did.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ImportResult {
    /// New Debtors.
    pub added_debtors: usize,
    /// New Debts.
    pub added_debts: usize,
    /// Rows whose Debt was already there.
    pub already_there: usize,
    /// Rows not added.
    pub invalid: Vec<ImportProblem>,
}

/// One row not added.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS), ts(export))]
pub struct ImportProblem {
    /// Its row number, as the spreadsheet shows it.
    pub row: usize,
    /// Why.
    pub problems: Vec<String>,
}

async fn import(
    State(state): State<AppState>,
    ApiBusiness(business): ApiBusiness,
    mut multipart: Multipart,
) -> Result<Json<ImportResult>, ApiError> {
    let mut file = None;
    while let Some(field) = multipart.next_field().await.map_err(upload_error)? {
        if field.name() == Some("file") {
            file = Some(field.bytes().await.map_err(upload_error)?);
        }
    }
    let bytes = file.ok_or_else(|| ApiError::invalid("Choose a CSV or Excel file."))?;
    // An Excel file is a zip archive; anything else is read as CSV.
    let rows = if bytes.starts_with(b"PK\x03\x04") {
        let reader = state
            .xlsx
            .as_ref()
            .ok_or_else(|| ApiError::invalid("Excel files can't be read here; save it as CSV and try again."))?;
        reader.cells(&bytes).await.map(numbered)
    } else {
        read_csv(&bytes)
    }
    .map_err(|err| match err {
        SheetError::TooLarge => ApiError::invalid("The file is larger than 2 MB."),
        SheetError::Unreadable(_) => {
            ApiError::invalid("That file couldn't be read. Save it as CSV or Excel and try again.")
        }
    })?;
    let report = Importer::new(state.pool.clone(), state.clock.clone())
        .import(business, &rows)
        .await
        .map_err(|err| match err {
            ImportError::Database(err) => ApiError::internal(err),
            other => ApiError::invalid(format!("{}.", capitalised(&other.to_string()))),
        })?;
    Ok(Json(ImportResult {
        added_debtors: report.added_debtors,
        added_debts: report.added_debts,
        already_there: report.already_there,
        invalid: report
            .invalid
            .into_iter()
            .map(|row| ImportProblem {
                row: row.row,
                problems: row.problems,
            })
            .collect(),
    }))
}

/// A CSV with the template's columns and one example row, to fill in.
async fn template(ApiBusiness(_): ApiBusiness) -> impl IntoResponse {
    let example = "Santos Trading,Lito Santos,lito@example.com,0917 123 4567,taglish,INV-001,2026-10-01,\"₱12,500.00\",2026-10-31,";
    (
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"balanse-debts.csv\"",
            ),
        ],
        format!("{}\n{example}\n", COLUMNS.join(",")),
    )
}

/// Too large, or cut short on the way.
fn upload_error(err: MultipartError) -> ApiError {
    if err.status() == StatusCode::PAYLOAD_TOO_LARGE {
        ApiError::invalid("The file is larger than 2 MB.")
    } else {
        ApiError::invalid("The file didn't arrive whole; try again.")
    }
}

fn capitalised(text: &str) -> String {
    let mut chars = text.chars();
    chars
        .next()
        .map(|first| first.to_uppercase().chain(chars).collect())
        .unwrap_or_default()
}
