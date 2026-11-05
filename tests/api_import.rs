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
