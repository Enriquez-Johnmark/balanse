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
use axum::extract::{DefaultBodyLimit, Multipart, Path, Request, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use time::macros::format_description;

use super::AppState;
use crate::balanse::contact_actions::ActionError;
use crate::balanse::debtor_links::LinkError;
use crate::balanse::debtors::DebtorError;
use crate::balanse::proofs::{MAX_FILE_BYTES, MAX_FILES_PER_CLAIM, ProofError};
use crate::balanse::reminders::open_debts;

/// How many proof uploads are read at once; each may hold up to 30 MB in
/// memory while it is checked.
pub const UPLOAD_SLOTS: usize = 4;

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/d/{token}", get(page))
        .route(
            "/d/{token}/paid",
            post(paid).layer(DefaultBodyLimit::max(
                MAX_FILE_BYTES * MAX_FILES_PER_CLAIM as usize + 64 * 1024,
            )),
        )
        .route("/d/{token}/dispute", post(dispute))
        .route("/d/{token}/stop", post(stop))
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
    pub proof: &'static str,
    pub too_many: &'static str,
    pub wrong_type: &'static str,
    pub too_big: &'static str,
    pub not_safe: &'static str,
    pub dispute: &'static str,
    pub stop: &'static str,
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
    proof: "Proof ng bayad, kung meron (photo o PDF, hanggang 3, 10 MB bawat isa)",
    too_many: "Hanggang 3 files lang po.",
    wrong_type: "Photo (JPEG, PNG, HEIC, WebP) o PDF lang po ang tinatanggap.",
    too_big: "Hanggang 10 MB lang po bawat file.",
    not_safe: "Hindi po namin matanggap ang file na ito ngayon. Pakisubukan ulit o magpadala ng iba.",
    dispute: "May dispute po ako dito",
    stop: "Huwag na po akong i-email",
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
    proof: "Proof of payment, if you have it (photo or PDF, up to 3, 10 MB each)",
    too_many: "Up to 3 files, please.",
    wrong_type: "Only photos (JPEG, PNG, HEIC, WebP) and PDFs are accepted.",
    too_big: "Each file can be up to 10 MB.",
    not_safe: "We can't accept this file right now. Please try again or send another one.",
    dispute: "I dispute this",
    stop: "Stop emailing me",
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
    /// Why a proof file was refused, in the Debtor's language.
    error: Option<&'static str>,
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
    Ok(render(&state, &token, None).await?.into_response())
}

/// The page for `token`, with `error` from a refused file, if any.
async fn render(
    state: &AppState,
    token: &str,
    error: Option<fn(&Words) -> &'static str>,
) -> Result<DebtorPage, PageError> {
    let link = state.links.open(token).await?;
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
        error: error.map(|message| message(w)),
    })
}

/// "I dispute this" (G3).
async fn dispute(State(state): State<AppState>, Path(token): Path<String>) -> Result<Response, PageError> {
    let link = state.links.open(&token).await?;
    state.actions.dispute(link.business, link.debtor).await?;
    Ok(Redirect::to(&format!("/d/{token}")).into_response())
}

/// "Stop emailing me" (G4).
async fn stop(State(state): State<AppState>, Path(token): Path<String>) -> Result<Response, PageError> {
    let link = state.links.open(&token).await?;
    state.actions.stop(link.business, link.debtor).await?;
    Ok(Redirect::to(&format!("/d/{token}")).into_response())
}

/// "I've paid" (G2), with up to 3 proof files (G5). Every file is checked
/// before the claim is recorded, so a refused file leaves nothing behind
/// and the Contact can try again. Back to the page afterwards, so a
/// refresh never repeats it.
async fn paid(
    State(state): State<AppState>,
    Path(token): Path<String>,
    multipart: Option<Multipart>,
) -> Result<Response, PageError> {
    let link = state.links.open(&token).await?;
    // The link is checked before taking a slot, so bad links can't take one.
    let _slot = state.upload_slots.acquire().await.map_err(PageError::internal)?;
    let files = match multipart {
        Some(multipart) => match read_files(multipart).await {
            Ok(files) => files,
            Err(message) => return refused(&state, &token, message).await,
        },
        None => Vec::new(),
    };
    let mut checked = Vec::with_capacity(files.len());
    if !files.is_empty() {
        let Some(proofs) = &state.proofs else {
            return refused(&state, &token, |w| w.not_safe).await;
        };
        for file in &files {
            match proofs.check(link.business, file).await {
                Ok(file) => checked.push(file),
                Err(err) => return refused(&state, &token, message(&err)).await,
            }
        }
    }
    let claim = state.actions.claim_paid(link.business, link.debtor).await?;
    if let Some(proofs) = &state.proofs {
        for file in checked {
            // A failure now is the store's (or the claim is full), and the claim stands.
            if let Err(err) = proofs.store(link.business, claim, file).await {
                tracing::error!(%err, "keeping a checked proof file failed");
            }
        }
    }
    Ok(Redirect::to(&format!("/d/{token}")).into_response())
}

/// The page again, with why a file was refused.
async fn refused(state: &AppState, token: &str, message: fn(&Words) -> &'static str) -> Result<Response, PageError> {
    let page = render(state, token, Some(message)).await?;
    Ok((StatusCode::UNPROCESSABLE_ENTITY, page).into_response())
}

/// The non-empty files sent as `file`, at most [`MAX_FILES_PER_CLAIM`].
async fn read_files(mut multipart: Multipart) -> Result<Vec<Vec<u8>>, fn(&Words) -> &'static str> {
    let mut files = Vec::new();
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| too_big as fn(&Words) -> &'static str)?
    {
        if field.name() != Some("file") {
            continue;
        }
        let bytes = field.bytes().await.map_err(|_| too_big as fn(&Words) -> &'static str)?;
        if bytes.is_empty() {
            continue;
        }
        if files.len() as i64 >= MAX_FILES_PER_CLAIM {
            return Err(|w| w.too_many);
        }
        files.push(bytes.to_vec());
    }
    Ok(files)
}

fn too_big(w: &Words) -> &'static str {
    w.too_big
}

/// The Contact's words for why a file was refused.
fn message(err: &ProofError) -> fn(&Words) -> &'static str {
    match err {
        ProofError::NotAllowedType | ProofError::NoFile => |w| w.wrong_type,
        ProofError::TooBig => |w| w.too_big,
        ProofError::ClaimFull => |w| w.too_many,
        _ => |w| w.not_safe,
    }
}
