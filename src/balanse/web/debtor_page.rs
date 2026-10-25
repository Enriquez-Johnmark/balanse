//! The Debtor's page (G1): what a Contact opens from a reminder, on a
//! phone, without logging in. The link itself is the credential; a forged,
//! altered or expired one shows the same "not found" page.
//!
//! The page shows the Debtor's open invoices exactly as the reminder listed
//! them and the Owner's own payment instructions (R12). Fixed wording in the
//! Debtor's language; the model writes none of it. Pages carry `noindex`
//! and a `same-origin` referrer policy, so the link never reaches another
//! site, and are never cached.

use askama::Template;
use askama_web::WebTemplate;
use axum::Router;
use axum::extract::{Path, Request, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use time::macros::format_description;

use super::AppState;
use crate::balanse::contact_actions::ActionError;
use crate::balanse::debtor_links::LinkError;
use crate::balanse::debtors::DebtorError;
use crate::balanse::reminders::open_debts;

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/d/{token}", get(page))
        .route("/d/{token}/paid", post(paid))
        .route("/d/{token}/dispute", post(dispute))
        .layer(middleware::from_fn(private))
}

/// Keeps Debtor pages out of search engines, caches and other sites' logs.
async fn private(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("x-robots-tag", HeaderValue::from_static("noindex, nofollow"));
    headers.insert(header::REFERRER_POLICY, HeaderValue::from_static("same-origin"));
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

/// The page's fixed wording, in one language.
pub(super) struct Words {
    pub lang: &'static str,
    pub title: &'static str,
    pub hello: &'static str,
    pub invoices: &'static str,
    pub invoice: &'static str,
    pub nothing_open: &'static str,
    pub how_to_pay: &'static str,
    pub ask_them: &'static str,
    pub paid: &'static str,
    pub dispute: &'static str,
    pub claimed: &'static str,
    pub disputed: &'static str,
    pub stopped: &'static str,
}

const TAGLISH: Words = Words {
    lang: "fil",
    title: "Mga invoice mula sa",
    hello: "Magandang araw po,",
    invoices: "Mga hindi pa nababayarang invoice",
    invoice: "Invoice",
    nothing_open: "Wala na pong natitirang balanse. Salamat po!",
    how_to_pay: "Paano magbayad",
    ask_them: "I-reply lang po sa email para sa detalye ng pagbabayad.",
    paid: "Nagbayad na po ako",
    dispute: "May dispute po ako dito",
    claimed: "Salamat po! Sinabi na namin na nagbayad na kayo; kukumpirmahin nila ito.",
    disputed: "Naitala na po na may dispute kayo; hindi na po kayo papadalhan ng paalala hangga't hindi ito naaayos.",
    stopped: "Hindi na po kayo makakatanggap ng email tungkol dito.",
};

const ENGLISH: Words = Words {
    lang: "en",
    title: "Invoices from",
    hello: "Hello",
    invoices: "Unpaid invoices",
    invoice: "Invoice",
    nothing_open: "Nothing is outstanding. Thank you!",
    how_to_pay: "How to pay",
    ask_them: "Reply to the email for payment details.",
    paid: "I've paid",
    dispute: "I dispute this",
    claimed: "Thank you! We've let them know you've paid; they'll confirm it.",
    disputed: "Your dispute is noted; you won't get reminders until it's sorted out.",
    stopped: "You won't get any more emails about this.",
};

pub(super) fn words(language: &str) -> &'static Words {
    if language == "taglish" { &TAGLISH } else { &ENGLISH }
}

/// One open Debt on the page.
pub(super) struct DebtItem {
    pub invoice_number: String,
    pub amount: String,
    /// "dated 30 Sep 2026, due 30 Oct 2026".
    pub dates: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "debtor_page.html")]
struct DebtorPage {
    w: &'static Words,
    /// The page's own path, where its buttons post.
    path: String,
    business: String,
    contact: String,
    debts: Vec<DebtItem>,
    instructions: String,
    /// What the Contact already did, if anything.
    status: Option<&'static str>,
    /// Whether the buttons are shown: not after the Contact acted.
    can_act: bool,
}

#[derive(Template, WebTemplate)]
#[template(path = "not_found.html")]
pub(super) struct NotFoundPage;

/// Why a page couldn't be shown.
pub(super) enum PageError {
    /// No such link: forged, altered or expired, all alike (G1).
    NotFound,
    /// Logged; the visitor sees a generic error.
    Internal,
}

impl PageError {
    pub(super) fn internal(err: impl std::fmt::Display) -> Self {
        tracing::error!(%err, "debtor page failed");
        Self::Internal
    }
}

impl From<LinkError> for PageError {
    fn from(err: LinkError) -> Self {
        match err {
            LinkError::NotFound => Self::NotFound,
            LinkError::Database(err) => Self::internal(err),
        }
    }
}

impl From<DebtorError> for PageError {
    fn from(err: DebtorError) -> Self {
        match err {
            DebtorError::NotFound => Self::NotFound,
            other => Self::internal(other),
        }
    }
}

impl From<ActionError> for PageError {
    fn from(err: ActionError) -> Self {
        match err {
            ActionError::NotFound => Self::NotFound,
            other => Self::internal(other),
        }
    }
}

impl From<sqlx::Error> for PageError {
    fn from(err: sqlx::Error) -> Self {
        Self::internal(err)
    }
}

impl IntoResponse for PageError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, NotFoundPage).into_response(),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong. Please try again.",
            )
                .into_response(),
        }
    }
}

async fn page(State(state): State<AppState>, Path(token): Path<String>) -> Result<Response, PageError> {
    let link = state.links.open(&token).await?;
    let debtor = state.debtors.get(link.business, link.debtor).await?;
    let settings = sqlx::query!(
        "select a.name, b.payment_instructions from businesses b
         join accounts a on a.tenant_id = b.tenant_id
         where b.tenant_id = $1",
        link.business.as_uuid(),
    )
    .fetch_one(&state.pool)
    .await?;
    let date = format_description!("[day padding:none] [month repr:short] [year]");
    let debts = open_debts(&state.pool, link.business, link.debtor)
        .await?
        .into_iter()
        .map(|debt| DebtItem {
            invoice_number: debt.invoice_number,
            amount: debt.amount,
            dates: format!(
                "dated {}, due {}",
                debt.invoice_date.format(date).unwrap_or_default(),
                debt.due_date.format(date).unwrap_or_default()
            ),
        })
        .collect();
    let w = words(&debtor.language);
    let status = match debtor.state.as_str() {
        "payment_claimed" => Some(w.claimed),
        "disputed" => Some(w.disputed),
        "opted_out" => Some(w.stopped),
        _ => None,
    };
    Ok(DebtorPage {
        w,
        path: format!("/d/{token}"),
        business: settings.name,
        contact: debtor.contact_name,
        debts,
        instructions: settings.payment_instructions,
        status,
        can_act: status.is_none() && debtor.state != "bounced",
    }
    .into_response())
}

/// "I dispute this" (G3).
async fn dispute(State(state): State<AppState>, Path(token): Path<String>) -> Result<Response, PageError> {
    let link = state.links.open(&token).await?;
    state.actions.dispute(link.business, link.debtor).await?;
    Ok(Redirect::to(&format!("/d/{token}")).into_response())
}

/// "I've paid" (G2). Back to the page afterwards, so a refresh never
/// repeats it.
async fn paid(State(state): State<AppState>, Path(token): Path<String>) -> Result<Response, PageError> {
    let link = state.links.open(&token).await?;
    state.actions.claim_paid(link.business, link.debtor).await?;
    Ok(Redirect::to(&format!("/d/{token}")).into_response())
}
