//! H2 (R11): no page, API response or export holds a total owed across
//! Debtors, an aging breakdown, or anything titled as an invoice, receipt
//! or statement of account.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{app, business, call, clock, get_page, log_in};
use serde_json::{Value, json};
use sqlx::PgPool;

/// Words no page or answer may carry anywhere.
const NEVER: [&str; 6] = [
    "total",
    "aging",
    "ageing",
    "statement of account",
    "billing statement",
    "ledger",
];

/// What no title, subject or file name may be: the documents only a
/// registered invoicing system may issue.
const DOCUMENTS: [&str; 8] = [
    "invoice",
    "billing invoice",
    "receipt",
    "official receipt",
    "collection receipt",
    "statement",
    "statement of account",
    "billing statement",
];

fn every_key(value: &Value, keys: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, inner) in map {
                keys.push(key.clone());
                every_key(inner, keys);
            }
        }
        Value::Array(items) => items.iter().for_each(|item| every_key(item, keys)),
        _ => {}
    }
}

/// Whether `title` is a document's name: the name alone, or followed by a
/// number or colon ("Invoice #12", "Receipt: …").
fn is_document_title(title: &str) -> bool {
    let title = title.trim().to_lowercase();
    DOCUMENTS.iter().any(|document| {
        title.strip_prefix(document).is_some_and(|rest| {
            let rest = rest.trim_start();
            rest.is_empty() || rest.starts_with(['#', ':', '-']) || rest.starts_with("no")
        })
    })
}

/// The text of every `<tag>…</tag>` in `html`.
fn tag_texts(html: &str, tag: &str) -> Vec<String> {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let mut texts = Vec::new();
    let mut rest = html;
    while let Some(start) = rest.find(&open) {
        let after = &rest[start..];
        let Some(gt) = after.find('>') else { break };
        let Some(end) = after.find(&close) else { break };
        texts.push(after[gt + 1..end].trim().to_owned());
        rest = &after[end..];
    }
    texts
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h2_no_page_answer_or_export_is_an_accounting_record(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let cookie = log_in(&app, &pool, "ana@reyes.test").await;
    let mut ids = Vec::new();
    for (name, amount) in [("Santos Trading", "₱12,500.00"), ("Cruz Studio", "₱3,000")] {
        let debtor = call(
            &app,
            "POST",
            "/api/debtors",
            Some(json!({ "business_name": name, "contact_name": "Lito",
                         "contact_email": format!("lito@{}.test", name.len()),
                         "contact_mobile": "", "language": "english" })),
            Some(&cookie),
        )
        .await;
        let id = debtor.body["id"].as_str().unwrap().to_owned();
        for number in ["INV-1", "INV-2"] {
            call(
                &app,
                "POST",
                &format!("/api/debtors/{id}/debts"),
                Some(json!({ "invoice_number": number, "invoice_date": "2026-09-01",
                             "amount": amount, "due_date": "2026-10-01", "note": "" })),
                Some(&cookie),
            )
            .await;
        }
        ids.push(id);
    }
    common::scheduler(&pool, &clock).run().await.unwrap();

    let mut answers = Vec::new();
    for path in ["/api/me", "/api/debtors", "/api/drafts", "/api/settings"] {
        answers.push(call(&app, "GET", path, None, Some(&cookie)).await.body);
    }
    for id in &ids {
        answers.push(
            call(&app, "GET", &format!("/api/debtors/{id}"), None, Some(&cookie))
                .await
                .body,
        );
    }
    let drafts = answers[2].clone();
    let body = drafts[0]["body"].as_str().unwrap();
    let link = body.split_whitespace().find(|word| word.contains("/d/")).unwrap();
    let path = &link[link.find("/d/").unwrap()..];
    let (_, page) = get_page(&app, path).await;
    let template = template(&app, &cookie).await;
    let evidence = file(&app, &format!("/api/debtors/{}/evidence-log", ids[0]), &cookie).await;
    let data = file(&app, "/api/export", &cookie).await;
    let data_keys = {
        let mut keys = Vec::new();
        every_key(&serde_json::from_str(&data.1).unwrap(), &mut keys);
        keys
    };

    let mut keys = Vec::new();
    answers.iter().for_each(|answer| every_key(answer, &mut keys));
    let text = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}",
        keys.join(" "),
        data_keys.join(" "),
        evidence.1,
        data.1,
        serde_json::to_string(&answers).unwrap(),
        page,
        template
    )
    .to_lowercase();
    for word in NEVER {
        assert!(!text.contains(word), "{word:?} appears");
    }
    let mut titles: Vec<String> = ["title", "h1", "h2", "h3"]
        .iter()
        .flat_map(|tag| tag_texts(&page, tag))
        .collect();
    titles.extend(
        drafts
            .as_array()
            .unwrap()
            .iter()
            .map(|draft| draft["subject"].as_str().unwrap().to_owned()),
    );
    titles.push("balanse-debts.csv".to_owned());
    titles.push(evidence.0);
    titles.push(data.0);
    assert!(titles.len() > 3, "{titles:?}");
    for title in titles {
        assert!(!is_document_title(&title), "{title:?} is titled as a document");
    }
}

#[test]
fn h2_document_titles_are_recognised() {
    assert!(is_document_title("Invoice"));
    assert!(is_document_title("Statement of Account"));
    assert!(is_document_title("Receipt #12"));
    assert!(!is_document_title("Invoices from Reyes Creative"));
    assert!(!is_document_title("Reminder: invoice INV-1 from Reyes Creative"));
}

async fn template(app: &axum::Router, cookie: &str) -> String {
    use tower::ServiceExt;
    let request = axum::http::Request::get("/api/import/template")
        .header("cookie", cookie)
        .body(axum::body::Body::empty())
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert!(
        response
            .headers()
            .get("content-disposition")
            .unwrap()
            .to_str()
            .unwrap()
            .contains("balanse-debts.csv")
    );
    let bytes = axum::body::to_bytes(response.into_body(), 1 << 20).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}

/// `GET path` with `cookie`: the file name it is saved as, and its text.
async fn file(app: &axum::Router, path: &str, cookie: &str) -> (String, String) {
    let (status, disposition, body) = common::download(app, path, cookie).await;
    assert_eq!(status, 200, "{path}");
    let name = disposition
        .split("filename=\"")
        .nth(1)
        .unwrap()
        .trim_end_matches('"')
        .to_owned();
    (name, body)
}
