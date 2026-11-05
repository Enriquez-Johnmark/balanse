//! G2, G5 through the API: the Owner sees an open claim with its proof
//! files, downloads them only as attachments, and answers it once (A2: only
//! their own).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use axum::body::Body;
use axum::http::Request;
use balanse::balanse::contact_actions::ContactActions;
use balanse::balanse::debtors::{DebtorInput, Debtors};
use balanse::balanse::proofs::Proofs;
use common::{business, call, clock, log_in};
use sqlx::PgPool;
use tower::ServiceExt as _;

const JPEG: &[u8] = b"\xFF\xD8\xFF\xE0 receipt";

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g2_the_owner_sees_the_claim_downloads_its_proof_and_confirms_it_once(pool: PgPool) {
    let clock = clock();
    let reyes = business(&pool, &clock).await;
    let debtor = Debtors::new(pool.clone(), clock.clone())
        .create(
            reyes,
            &DebtorInput {
                business_name: "Santos Trading".to_owned(),
                contact_name: "Lito".to_owned(),
                contact_email: "lito@santos.test".to_owned(),
                contact_mobile: String::new(),
                language: "english".to_owned(),
            },
        )
        .await
        .unwrap();
    let claim = ContactActions::new(pool.clone(), clock.clone(), common::links(&pool, &clock))
        .claim_paid(reyes, debtor)
        .await
        .unwrap();
    let dir = tempfile::tempdir().unwrap();
    Proofs::new(pool.clone(), clock.clone(), common::files(&pool, &clock, dir.path()))
        .add(reyes, claim, JPEG)
        .await
        .unwrap();
    let app = common::app_with_proofs(&pool, &clock, dir.path());
    let ana = log_in(&app, &pool, "ana@reyes.test").await;
    balanse::balanse::businesses::create_business(&pool, clock.as_ref(), "Other Co", "bo@other.test")
        .await
        .unwrap();
    let bo = log_in(&app, &pool, "bo@other.test").await;

    let page = call(
        &app,
        "GET",
        &format!("/api/debtors/{}", debtor.as_uuid()),
        None,
        Some(&ana),
    )
    .await;
    let file = page.body["claim"]["files"][0]["id"].as_str().unwrap().to_owned();
    let path = format!("/api/claims/{}/files/{file}", claim.as_uuid());
    let download = app
        .clone()
        .oneshot(Request::get(&path).header("cookie", &ana).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let theirs = call(&app, "GET", &path, None, Some(&bo)).await;
    let not_theirs = call(
        &app,
        "POST",
        &format!("/api/claims/{}/confirm", claim.as_uuid()),
        None,
        Some(&bo),
    )
    .await;
    let confirmed = call(
        &app,
        "POST",
        &format!("/api/claims/{}/confirm", claim.as_uuid()),
        None,
        Some(&ana),
    )
    .await;
    let again = call(
        &app,
        "POST",
        &format!("/api/claims/{}/reject", claim.as_uuid()),
        None,
        Some(&ana),
    )
    .await;

    assert_eq!(page.body["claim"]["files"].as_array().unwrap().len(), 1);
    assert_eq!(download.status(), 200);
    let disposition = download.headers()["content-disposition"].to_str().unwrap().to_owned();
    assert!(
        disposition.starts_with("attachment; filename=\"proof-Santos-Trading-"),
        "{disposition}"
    );
    let bytes = axum::body::to_bytes(download.into_body(), usize::MAX).await.unwrap();
    assert_eq!(&bytes[..], JPEG);
    assert_eq!(
        (
            theirs.status.as_u16(),
            not_theirs.status.as_u16(),
            confirmed.status.as_u16(),
            again.status.as_u16()
        ),
        (404, 404, 204, 409)
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g5_the_owners_paid_decision_closes_an_open_claim_and_its_files_go_in_30_days(pool: PgPool) {
    use balanse::engine::clock::Clock as _;
    let clock = clock();
    let reyes = business(&pool, &clock).await;
    let debtor = Debtors::new(pool.clone(), clock.clone())
        .create(
            reyes,
            &DebtorInput {
                business_name: "Santos Trading".to_owned(),
                contact_name: "Lito".to_owned(),
                contact_email: "lito@santos.test".to_owned(),
                contact_mobile: String::new(),
                language: "english".to_owned(),
            },
        )
        .await
        .unwrap();
    let claim = ContactActions::new(pool.clone(), clock.clone(), common::links(&pool, &clock))
        .claim_paid(reyes, debtor)
        .await
        .unwrap();
    let dir = tempfile::tempdir().unwrap();
    Proofs::new(pool.clone(), clock.clone(), common::files(&pool, &clock, dir.path()))
        .add(reyes, claim, JPEG)
        .await
        .unwrap();
    let app = common::app_with_proofs(&pool, &clock, dir.path());
    let ana = log_in(&app, &pool, "ana@reyes.test").await;

    let paid = call(
        &app,
        "POST",
        &format!("/api/debtors/{}/outcome", debtor.as_uuid()),
        Some(serde_json::json!({ "outcome": "paid", "until": null })),
        Some(&ana),
    )
    .await;
    let page = call(
        &app,
        "GET",
        &format!("/api/debtors/{}", debtor.as_uuid()),
        None,
        Some(&ana),
    )
    .await;

    assert_eq!(paid.status.as_u16(), 204);
    assert!(page.body["claim"].is_null(), "{}", page.body);
    let (outcome, delete_by): (Option<String>, time::OffsetDateTime) = sqlx::query_as(
        "select c.outcome, s.delete_by from claims c join claim_files f on f.claim_id = c.id join stored_files s on s.id = f.id",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(outcome.as_deref(), Some("confirmed"));
    assert_eq!(delete_by, clock.now() + time::Duration::days(30));
}
