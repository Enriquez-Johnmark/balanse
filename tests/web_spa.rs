//! The built React app is served for every page address; its files from
//! `/assets/` are cached for good, since their names change with content.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::body::Body;
use axum::http::{HeaderMap, Request, StatusCode, header};
use balanse::balanse::web::{AppState, router};
use sqlx::PgPool;
use tower::ServiceExt as _;

fn built_app() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("index.html"), "<!doctype html><div id=root></div>").unwrap();
    std::fs::create_dir(dir.path().join("assets")).unwrap();
    std::fs::write(dir.path().join("assets/index-abc123.js"), "console.log(1)").unwrap();
    std::fs::write(dir.path().join("favicon.svg"), "<svg/>").unwrap();
    dir
}

async fn get(pool: &PgPool, web_dir: &std::path::Path, path: &str) -> (StatusCode, HeaderMap, String) {
    let state = AppState::new(pool.clone(), common::clock(), common::BASE_URL).with_web_dir(web_dir);
    let response = router(state)
        .oneshot(Request::get(path).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (status, headers, String::from_utf8_lossy(&body).into_owned())
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn an_app_address_gets_the_app_page_never_cached(pool: PgPool) {
    let dir = built_app();

    let (status, headers, body) = get(&pool, dir.path(), "/debtors/123").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<div id=root>"), "{body}");
    assert_eq!(headers[header::CACHE_CONTROL], "no-cache");
    assert!(headers.contains_key(header::CONTENT_SECURITY_POLICY));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn the_apps_files_are_cached_for_good(pool: PgPool) {
    let dir = built_app();

    let (status, headers, body) = get(&pool, dir.path(), "/assets/index-abc123.js").await;
    let (missing, _, _) = get(&pool, dir.path(), "/assets/gone.js").await;
    let (_, _, icon) = get(&pool, dir.path(), "/favicon.svg").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "console.log(1)");
    assert_eq!(headers[header::CACHE_CONTROL], "public, max-age=31536000, immutable");
    assert_eq!(missing, StatusCode::NOT_FOUND);
    assert_eq!(icon, "<svg/>");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn api_addresses_never_get_the_app_page(pool: PgPool) {
    let dir = built_app();

    let (status, _, body) = get(&pool, dir.path(), "/api/nothing-here").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(!body.contains("<div id=root>"), "{body}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn files_outside_the_app_folder_are_never_served(pool: PgPool) {
    let dir = built_app();

    let (_, _, body) = get(&pool, dir.path(), "/assets/../../etc/passwd").await;

    assert!(!body.contains("root:"), "{body}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn without_a_built_app_the_server_says_so(pool: PgPool) {
    let (status, _, body) = get(&pool, std::path::Path::new("/nonexistent/balanse-web"), "/debtors").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains("pnpm --dir web build"), "{body}");
}
