//! H5: the Owner can download all their data, and closing the account
//! deletes it within 30 days.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use balanse::balanse::businesses::create_business;
use common::{app, clock, log_in, sent};
use sqlx::PgPool;
use time::Duration;

/// `GET path` with `cookie`: the status, the content-disposition header and
/// the body.
async fn download(app: &axum::Router, path: &str, cookie: &str) -> (u16, String, String) {
    use tower::ServiceExt;
    let request = axum::http::Request::get(path)
        .header("cookie", cookie)
        .body(axum::body::Body::empty())
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status().as_u16();
    let disposition = response
        .headers()
        .get("content-disposition")
        .map(|value| value.to_str().unwrap().to_owned())
        .unwrap_or_default();
    let bytes = axum::body::to_bytes(response.into_body(), 1 << 20).await.unwrap();
    (status, disposition, String::from_utf8(bytes.to_vec()).unwrap())
}

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

    let (status, disposition, body) = download(&app, "/api/export", &ana).await;
    let (_, _, others) = download(&app, "/api/export", &cara).await;

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

    let (status, _, _) = download(&app, "/api/export", &ana).await;

    assert_eq!(status, 403);
}
