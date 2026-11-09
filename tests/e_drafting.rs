//! E1, E2: the model writes only a reminder's wording, from six permitted
//! facts; anything it gets wrong falls back to the template at once.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::{Arc, Mutex};

use balanse::balanse::debtors::{DebtorInput, Debtors};
use balanse::balanse::debts::{DebtInput, Debts};
use balanse::balanse::drafting::ModelWording;
use balanse::engine::llm::{LanguageModel, LlmError, LlmReply, LlmRequest};
use common::{business, clock};
use sqlx::PgPool;

/// A model that answers with `answer` and keeps every request.
#[derive(Clone)]
struct FakeModel {
    answer: Result<String, LlmError>,
    asked: Arc<Mutex<Vec<LlmRequest>>>,
}

impl FakeModel {
    fn answering(answer: Result<&str, LlmError>) -> Self {
        Self {
            answer: answer.map(str::to_owned),
            asked: Arc::default(),
        }
    }
}

impl LanguageModel for FakeModel {
    fn model(&self) -> &str {
        "claude-haiku-5-5"
    }

    async fn complete(&self, request: &LlmRequest) -> Result<LlmReply, LlmError> {
        self.asked.lock().unwrap().push(request.clone());
        self.answer.clone().map(|text| LlmReply {
            text,
            model: "claude-haiku-5-5".to_owned(),
            request_id: Some("req_1".to_owned()),
            input_tokens: 120,
            output_tokens: 60,
        })
    }
}

const GOOD: &str = r#"{"subject":"A gentle note from us","opening":"We hope business is going well. Just a gentle note about the invoice below.","closing":"Thank you so much for your help."}"#;

/// Runs the scheduler for Santos Trading (one overdue Debt) with `model`,
/// and returns the reminder drafted.
async fn draft_with(pool: &PgPool, model: &FakeModel) -> (String, String) {
    let clock = clock();
    let business = business(pool, &clock).await;
    let debtor = Debtors::new(pool.clone(), clock.clone())
        .create(
            business,
            &DebtorInput {
                business_name: "Santos Trading".to_owned(),
                contact_name: "Lito Santos".to_owned(),
                contact_email: "lito@santos.test".to_owned(),
                contact_mobile: String::new(),
                language: "english".to_owned(),
            },
        )
        .await
        .unwrap();
    Debts::new(pool.clone(), clock.clone())
        .create(
            business,
            debtor,
            &DebtInput {
                invoice_number: "INV-7".to_owned(),
                invoice_date: "2026-09-30".to_owned(),
                amount: "₱12,500.00".to_owned(),
                due_date: "2026-10-30".to_owned(),
                note: "private note".to_owned(),
            },
        )
        .await
        .unwrap();
    let wording = ModelWording::new(pool.clone(), clock.clone(), model.clone());
    common::scheduler(pool, &clock)
        .with_wording(Arc::new(wording))
        .run()
        .await
        .unwrap();
    sqlx::query_as("select subject, body_text from outbox_messages")
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn e1_the_model_sees_only_the_six_permitted_facts(pool: PgPool) {
    let model = FakeModel::answering(Ok(GOOD));

    draft_with(&pool, &model).await;

    let asked = model.asked.lock().unwrap();
    let facts: serde_json::Value = serde_json::from_str(&asked[0].user).unwrap();
    let mut keys: Vec<_> = facts.as_object().unwrap().keys().cloned().collect();
    keys.sort();
    assert_eq!(
        keys,
        [
            "contact_first_name",
            "debtor_name",
            "language",
            "open_invoices",
            "overdue",
            "step"
        ]
    );
    assert_eq!(facts["contact_first_name"], "Lito");
    assert_eq!(facts["open_invoices"], 1);
    let everything = format!("{}{}", asked[0].system, asked[0].user);
    for secret in ["12,500", "INV-7", "private note", "lito@santos.test", "Reyes"] {
        assert!(!everything.contains(secret), "{secret} reached the model");
    }
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn e1_the_model_is_told_never_to_say_how_long_an_invoice_is_overdue(pool: PgPool) {
    // It is told only a rough range (up to a week, a month, three months…),
    // so a length it states ("overdue for several months" for 37 days) can
    // overstate it.
    let model = FakeModel::answering(Ok(GOOD));

    draft_with(&pool, &model).await;

    let asked = model.asked.lock().unwrap();
    assert!(
        asked[0]
            .system
            .contains("Never say how long an invoice has been unpaid or overdue"),
        "{}",
        asked[0].system
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn e2_valid_wording_is_used_with_the_facts_inserted_and_the_call_logged(pool: PgPool) {
    let model = FakeModel::answering(Ok(GOOD));

    let (subject, body) = draft_with(&pool, &model).await;

    assert_eq!(subject, "A gentle note from us");
    for part in [
        "Hi Lito,",
        "Just a gentle note",
        "- Invoice INV-7",
        "₱12,500.00",
        "/d/",
        "Thank you so much",
        "Sent via Balanse",
    ] {
        assert!(body.contains(part), "{part} missing from {body}");
    }
    let (outcome, tokens): (String, i32) = sqlx::query_as("select outcome, output_tokens from llm_calls")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!((outcome.as_str(), tokens), ("ok", 60));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn e2_a_failed_or_invalid_answer_falls_back_to_the_template(pool: PgPool) {
    let bad = [
        Err(LlmError::Unavailable("timed out".to_owned())),
        Ok("not json"),
        Ok(r#"{"subject":"Hi","opening":"Hello","closing":"Thanks","extra":"x"}"#),
        Ok(r#"{"subject":"Pay now","opening":"Pay at https://pay.example.com today.","closing":"Thanks"}"#),
        Ok(r#"{"subject":"Reminder","opening":"You owe ₱12,500 to us.","closing":"Thanks"}"#),
        Ok(r#"{"subject":"Reminder","opening":"Write to billing@reyes.test about this.","closing":"Thanks"}"#),
        Ok(r#"{"subject":"Reminder","opening":"Call 09171234567 today.","closing":"Thanks"}"#),
        Ok(r#"{"subject":"Reminder","opening":"Kung hindi, kakasuhan namin kayo.","closing":"Salamat"}"#),
        Ok(r#"{"subject":"","opening":"Hello","closing":"Thanks"}"#),
    ];
    for answer in bad {
        let label = format!("{answer:?}");
        sqlx::query("truncate accounts, outbox_messages, llm_calls cascade")
            .execute(&pool)
            .await
            .unwrap();
        let model = FakeModel::answering(answer);

        let (subject, body) = draft_with(&pool, &model).await;

        assert_eq!(subject, "Reminder: invoice INV-7 from Reyes Creative", "{label}");
        assert!(body.contains("This is a friendly reminder"), "{label}: {body}");
    }
}
