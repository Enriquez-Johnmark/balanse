//! B2 through the API: the Owner uploads a spreadsheet and sees what was
//! added and which rows weren't, with why.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{app, business, call, clock, log_in, post_files_as};
use sqlx::PgPool;

const FILE: &[u8] = b"Debtor,Contact name,Contact email,Invoice number,Invoice date,Amount,Due date
Santos Trading,Lito Santos,lito@santos.test,INV-1,2026-09-30,12500,2026-10-30
Cruz Studio,Mara Cruz,mara@cruz.test,INV-3,2026-10-01,twelve,2026-10-31
";

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b2_an_uploaded_file_is_imported_and_its_bad_rows_listed(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let ana = log_in(&app, &pool, "ana@reyes.test").await;

    let (status, body) = post_files_as(&app, "/api/import", &[("file", "debts.csv", FILE)], Some(&ana)).await;
    let template = call(&app, "GET", "/api/import/template", None, Some(&ana)).await;
    let (refused, _) = post_files_as(&app, "/api/import", &[("file", "debts.csv", FILE)], None).await;

    assert_eq!(status.as_u16(), 200, "{body}");
    let report: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(
        (report["added_debts"].as_i64(), report["added_debtors"].as_i64()),
        (Some(1), Some(1))
    );
    assert_eq!(report["invalid"][0]["row"], 3);
    assert!(
        report["invalid"][0]["problems"][0]
            .as_str()
            .unwrap()
            .starts_with("Amount")
    );
    assert_eq!(template.status.as_u16(), 200);
    assert_eq!(refused.as_u16(), 401);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b2_a_file_that_isnt_a_spreadsheet_is_refused_with_why(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let ana = log_in(&app, &pool, "ana@reyes.test").await;

    let (status, body) = post_files_as(&app, "/api/import", &[("file", "notes.csv", b"just,one\n")], Some(&ana)).await;

    assert_eq!(status.as_u16(), 422);
    assert!(body.contains("has no Debtor,"), "{body}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b2_an_upload_cut_short_says_so_rather_than_blaming_its_size(pool: PgPool) {
    use tower::ServiceExt;
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let ana = log_in(&app, &pool, "ana@reyes.test").await;
    // A part that never ends: the connection dropped mid-upload.
    let body = "--x\r\nContent-Disposition: form-data; name=\"file\"; filename=\"a.csv\"\r\n\r\nDebtor,Amo";
    let request = axum::http::Request::post("/api/import")
        .header("origin", common::BASE_URL)
        .header("cookie", &ana)
        .header("content-type", "multipart/form-data; boundary=x")
        .body(axum::body::Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status().as_u16(), 422);
    let bytes = axum::body::to_bytes(response.into_body(), 1 << 16).await.unwrap();
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("didn't arrive whole"), "{text}");
}
