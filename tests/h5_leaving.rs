//! H5: the Owner can download all their data, and closing the account
//! deletes it within 30 days.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use balanse::balanse::businesses::create_business;
use common::{app, clock, log_in, sent};
use sqlx::PgPool;
use time::Duration;

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h5_the_owner_downloads_all_their_data(pool: PgPool) {
    let clock = clock();
    sent(&pool, &clock).await;
    create_business(&pool, clock.as_ref(), "Other Studio", "cara@other.test")
        .await
        .unwrap();
    let app = app(&pool, &clock);
    let ana = log_in(&app, &pool, "ana@reyes.test").await;
    let cara = log_in(&app, &pool, "cara@other.test").await;

    let (status, disposition, body) = common::download(&app, "/api/export", &ana).await;
    let (_, _, others) = common::download(&app, "/api/export", &cara).await;

    assert_eq!(status, 200);
    assert!(
        disposition.starts_with("attachment;") && disposition.contains(".json"),
        "{disposition}"
    );
    let data: serde_json::Value = serde_json::from_str(&body).unwrap();
    let debtor = &data["debtors"][0];
    assert_eq!(data["business"]["name"], "Reyes Creative");
    assert_eq!(debtor["business_name"], "Santos Trading");
    assert_eq!(debtor["debts"][0]["amount"], "₱1,000");
    assert!(
        debtor["messages"][0]["body"].as_str().unwrap().contains("Hi Lito"),
        "{debtor}"
    );
    let others: serde_json::Value = serde_json::from_str(&others).unwrap();
    assert_eq!(others["debtors"], serde_json::json!([]), "only their own data (A2)");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h5_downloading_everything_needs_a_recent_login(pool: PgPool) {
    let clock = clock();
    common::business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let ana = log_in(&app, &pool, "ana@reyes.test").await;
    clock.advance(Duration::minutes(11));

    let (status, _, _) = common::download(&app, "/api/export", &ana).await;

    assert_eq!(status, 403);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn h5_closing_the_account_deletes_everything_within_30_days(pool: PgPool) {
    let clock = clock();
    sent(&pool, &clock).await;
    let app = app(&pool, &clock);
    let ana = log_in(&app, &pool, "ana@reyes.test").await;
    let dir = tempfile::tempdir().unwrap();
    let accounts = balanse::engine::accounts::Accounts::new(pool.clone(), clock.clone());

    let wrong = common::call(
        &app,
        "POST",
        "/api/account/close",
        Some(serde_json::json!({ "typed_name": "Reyes" })),
        Some(&ana),
    )
    .await;
    let closed = common::call(
        &app,
        "POST",
        "/api/account/close",
        Some(serde_json::json!({ "typed_name": "reyes creative" })),
        Some(&ana),
    )
    .await;
    let after = common::call(&app, "GET", "/api/me", None, Some(&ana)).await;
    clock.advance(Duration::days(30));
    accounts
        .erase_due(&common::files(&pool, &clock, dir.path()))
        .await
        .unwrap();

    assert_eq!(
        (wrong.status.as_u16(), closed.status.as_u16(), after.status.as_u16()),
        (422, 200, 401)
    );
    assert!(closed.body["erase_after"].is_string(), "{}", closed.body);
    for table in ["accounts", "businesses", "debtors", "debts", "outbox_messages"] {
        let left: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!("select count(*) from {table}")))
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(left, 0, "{table} still has rows");
    }
}
