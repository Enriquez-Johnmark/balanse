//! The model's part in a reminder (M3): it writes only the wording, a
//! subject, an opening and a closing, from six facts (E1). Balanse inserts
//! everything factual itself: the greeting, the invoices and amounts, the
//! Debtor's page link, the Business's name and the footer, so the model
//! can't garble or invent them.
//!
//! Its answer is untrusted. Anything wrong with it (no answer, the wrong
//! shape, too long, a link, an amount, an address, a digit, or a word the
//! content guard refuses) means the template is used at once (E2). Every
//! call is logged with its cost.

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use time::Date;

use super::guard;
use super::schedule::Step;
use super::wording::Lines;
use crate::engine::clock::Clock;
use crate::engine::cost_log::{self, LlmCall, Outcome};
use crate::engine::ids::TenantId;
use crate::engine::llm::{LanguageModel, LlmError, LlmRequest, Usage};
use crate::engine::outbox::{BoxFuture, links};

/// How long the oldest open Debt has been overdue, as a range (E1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Overdue {
    /// Not due yet (a heads-up).
    NotYetDue,
    /// Up to a week.
    UpToAWeek,
    /// Up to a month.
    UpToAMonth,
    /// Up to three months.
    UpToThreeMonths,
    /// Longer.
    OverThreeMonths,
}

impl Overdue {
    /// The range for a Debt due on `due`, as of `today`.
    pub fn since(due: Date, today: Date) -> Self {
        match (today - due).whole_days() {
            ..=0 => Self::NotYetDue,
            1..=7 => Self::UpToAWeek,
            8..=31 => Self::UpToAMonth,
            32..=92 => Self::UpToThreeMonths,
            _ => Self::OverThreeMonths,
        }
    }
}

/// Everything the model may know about a reminder, and nothing else (E1):
/// never an amount, an invoice number, a note or a reply.
#[derive(Debug, Clone, Serialize)]
pub struct Brief {
    /// The Debtor's business name.
    pub debtor_name: String,
    /// The Contact's first name.
    pub contact_first_name: String,
    /// How many open invoices.
    pub open_invoices: usize,
    /// How long the oldest has been overdue, as a range.
    pub overdue: Overdue,
    /// `taglish` or `english`.
    pub language: String,
    /// Which reminder of the cycle.
    pub step: &'static str,
}

impl Brief {
    /// The brief for `step` of a reminder.
    pub fn new(
        debtor_name: &str,
        contact_first_name: &str,
        open_invoices: usize,
        overdue: Overdue,
        language: &str,
        step: Step,
    ) -> Self {
        Self {
            debtor_name: debtor_name.to_owned(),
            contact_first_name: contact_first_name.to_owned(),
            open_invoices,
            overdue,
            language: language.to_owned(),
            step: step.as_str(),
        }
    }
}

/// Something that may write a reminder's wording; `None` means use the
/// template.
pub trait WordingSource: Send + Sync {
    /// The wording for `brief`, written for `tenant`.
    fn write<'a>(&'a self, tenant: TenantId, brief: &'a Brief) -> BoxFuture<'a, Option<Lines>>;
}

const SYSTEM: &str = "You write the wording of a short, polite payment reminder email from a Philippine small \
business to a client company. The email itself greets the contact, lists the unpaid invoices with their amounts, \
links to a page with payment details, and signs off; you write only:
- subject: one line, at most 100 characters;
- opening: one to three sentences, after the greeting, saying an invoice is due or unpaid;
- closing: one short sentence of thanks.
Write in the requested language. Taglish means natural Tagalog-English as Filipino businesses write to clients, \
with \"po\" where respectful. The step sets the tone: heads_up and first are friendly, second is polite, third is \
firm but polite and asks when payment can be expected. Never threaten, blame, shame or mention legal action, \
lawyers, courts, the police or anyone else. Never include links, email addresses, numbers, amounts or dates. \
Never say how long an invoice has been unpaid or overdue: the overdue range you are given is only rough.";

/// The shape the model must answer in.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Answer {
    subject: String,
    opening: String,
    closing: String,
}

/// Wording written by a model.
pub struct ModelWording<M> {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    model: M,
}

impl<M: LanguageModel> ModelWording<M> {
    /// Wording by `model`; its calls are logged in `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, model: M) -> Self {
        Self { pool, clock, model }
    }

    async fn ask(&self, tenant: TenantId, brief: &Brief) -> Option<Lines> {
        let request = LlmRequest {
            system: SYSTEM.to_owned(),
            user: serde_json::to_string(brief).ok()?,
            schema: json!({
                "type": "object",
                "properties": {"subject": {"type": "string"}, "opening": {"type": "string"}, "closing": {"type": "string"}},
                "required": ["subject", "opening", "closing"],
                "additionalProperties": false,
            }),
            max_tokens: 1_000,
        };
        let (lines, usage, model, request_id, outcome) = match self.model.complete(&request).await {
            Ok(reply) => {
                let usage = Usage {
                    input_tokens: reply.input_tokens,
                    output_tokens: reply.output_tokens,
                };
                let lines = serde_json::from_str::<Answer>(&reply.text)
                    .ok()
                    .and_then(|answer| valid(answer, brief));
                let outcome = if lines.is_some() { Outcome::Ok } else { Outcome::Invalid };
                (lines, usage, reply.model, reply.request_id, outcome)
            }
            Err(err) => {
                tracing::warn!(%tenant, %err, "the model gave no wording");
                let (usage, request_id) = match err {
                    LlmError::Incomplete { usage, request_id, .. } => (usage, request_id),
                    LlmError::Api { request_id, .. } => (Usage::default(), request_id),
                    LlmError::Unavailable(_) | LlmError::Malformed(_) => (Usage::default(), None),
                };
                (None, usage, self.model.model().to_owned(), request_id, Outcome::Failed)
            }
        };
        let call = LlmCall {
            tenant,
            at: self.clock.now(),
            model,
            usage,
            request_id,
            outcome,
        };
        if let Err(err) = cost_log::record(&self.pool, &call).await {
            tracing::error!(%tenant, %err, "logging a model call failed");
        }
        if lines.is_none() {
            tracing::info!(%tenant, "using the template for a reminder");
        }
        lines
    }
}

impl<M: LanguageModel> WordingSource for ModelWording<M> {
    fn write<'a>(&'a self, tenant: TenantId, brief: &'a Brief) -> BoxFuture<'a, Option<Lines>> {
        Box::pin(self.ask(tenant, brief))
    }
}

/// E2: the answer is used only if every part is short, plain text that
/// passes the content guard at its strictest.
fn valid(answer: Answer, brief: &Brief) -> Option<Lines> {
    let subject = answer.subject.trim();
    let (opening, closing) = (answer.opening.trim(), answer.closing.trim());
    let plain = |text: &str| {
        links(text).is_empty()
            && !text.contains('@')
            && !text.contains('₱')
            && !text.chars().any(|c| c.is_ascii_digit())
    };
    let ok = !subject.is_empty()
        && subject.chars().count() <= 120
        && !subject.contains(['\n', '\r'])
        && [opening, closing]
            .iter()
            .all(|part| !part.is_empty() && part.chars().count() <= 600)
        && [subject, opening, closing].into_iter().all(plain)
        && guard::check(
            &format!("{subject}\n{opening}\n{closing}"),
            &[&brief.debtor_name, &brief.contact_first_name],
            guard::Writer::Model,
        )
        .is_none();
    ok.then(|| Lines {
        subject: subject.to_owned(),
        opening: opening.to_owned(),
        closing: closing.to_owned(),
    })
}
