//! G1: the Debtor's page opens only with a genuine, unexpired signed link.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use balanse::balanse::debtor_links::{DebtorLinks, LinkError, WORKS_FOR};
use balanse::balanse::debtors::{DebtorInput, Debtors};
use balanse::engine::files::MasterKey;
use common::{BASE_URL, business, clock};
use sqlx::PgPool;
use time::Duration;

fn token(url: &str) -> &str {
    url.rsplit('/').next().unwrap()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g1_a_genuine_link_opens_its_debtor_until_it_expires(pool: PgPool) {
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
    let master = MasterKey::from_hex(&"ab".repeat(32)).unwrap();
    let links = DebtorLinks::new(pool.clone(), clock.clone(), &master, BASE_URL);

    let url = links.issue(reyes, debtor).await.unwrap();
    let again = links.issue(reyes, debtor).await.unwrap();
    let opened = links.open(token(&url)).await.unwrap();
    let forged = links
        .open(&format!("{}00", &token(&url)[..token(&url).len() - 2]))
        .await;
    clock.advance(WORKS_FOR);
    let expired = links.open(token(&url)).await;

    assert!(url.starts_with("http://balanse.test/d/"), "{url}");
    assert_eq!(again, url, "one link per Debtor while it works");
    assert_eq!((opened.business, opened.debtor), (reyes, debtor));
    assert!(matches!(forged, Err(LinkError::NotFound)), "{forged:?}");
    assert!(matches!(expired, Err(LinkError::NotFound)), "{expired:?}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g1_a_link_about_to_expire_is_replaced_in_the_next_reminder(pool: PgPool) {
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
    let master = MasterKey::from_hex(&"ab".repeat(32)).unwrap();
    let links = DebtorLinks::new(pool.clone(), clock.clone(), &master, BASE_URL);
    let first = links.issue(reyes, debtor).await.unwrap();

    clock.advance(WORKS_FOR - Duration::days(7));
    let second = links.issue(reyes, debtor).await.unwrap();

    assert_ne!(first, second);
    assert!(links.open(token(&first)).await.is_ok(), "the old link still works");
    assert_eq!(links.current_urls(reyes, debtor).await.unwrap().len(), 2);
}

/// Reyes Creative, Santos Trading owing it INV-7, and the path of Santos's
/// page link.
async fn santos_page(pool: &PgPool, clock: &std::sync::Arc<balanse::engine::clock::FixedClock>) -> String {
    let reyes = business(pool, clock).await;
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
    balanse::balanse::debts::Debts::new(pool.clone(), clock.clone())
        .create(
            reyes,
            debtor,
            &balanse::balanse::debts::DebtInput {
                invoice_number: "INV-7".to_owned(),
                invoice_date: "2026-09-30".to_owned(),
                amount: "₱12,500.00".to_owned(),
                due_date: "2026-10-30".to_owned(),
                note: "never shown".to_owned(),
            },
        )
        .await
        .unwrap();
    sqlx::query("update businesses set payment_instructions = 'GCash 0917 123 4567 (Ana Reyes)'")
        .execute(pool)
        .await
        .unwrap();
    let url = common::links(pool, clock).issue(reyes, debtor).await.unwrap();
    url.trim_start_matches(BASE_URL).to_owned()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g1_the_page_shows_the_open_invoices_and_how_to_pay(pool: PgPool) {
    let clock = clock();
    let path = santos_page(&pool, &clock).await;
    let app = common::app(&pool, &clock);

    let (status, page) = common::get_page(&app, &path).await;

    assert_eq!(status.as_u16(), 200);
    for text in [
        "Reyes Creative",
        "Lito",
        "INV-7",
        "₱12,500.00",
        "due 30 Oct 2026",
        "GCash 0917 123 4567",
    ] {
        assert!(page.contains(text), "{text} missing from {page}");
    }
    assert!(!page.contains("never shown"), "the private note stays private");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g1_a_forged_or_expired_link_shows_the_same_not_found_page(pool: PgPool) {
    let clock = clock();
    let path = santos_page(&pool, &clock).await;
    let app = common::app(&pool, &clock);

    let (forged_status, forged) = common::get_page(&app, &format!("{}00", &path[..path.len() - 2])).await;
    clock.advance(WORKS_FOR);
    let (expired_status, expired) = common::get_page(&app, &path).await;

    assert_eq!((forged_status.as_u16(), expired_status.as_u16()), (404, 404));
    assert_eq!(forged, expired);
    assert!(forged.contains("This link doesn't work"), "{forged}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g2_pressing_i_have_paid_claims_payment_and_thanks_the_contact(pool: PgPool) {
    let clock = clock();
    let path = santos_page(&pool, &clock).await;
    let app = common::app(&pool, &clock);

    let (status, location) = common::post_page(&app, &format!("{path}/paid")).await;
    let (_, page) = common::get_page(&app, &path).await;

    assert_eq!((status.as_u16(), location.as_deref()), (303, Some(path.as_str())));
    assert!(page.contains("confirm it"), "{page}");
    assert!(!page.contains("/paid\""), "the button is gone: {page}");
    let state: String = sqlx::query_scalar("select state from debtors")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(state, "payment_claimed");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g1_a_forged_link_cant_claim_payment(pool: PgPool) {
    let clock = clock();
    let path = santos_page(&pool, &clock).await;
    let app = common::app(&pool, &clock);

    let (status, _) = common::post_page(&app, &format!("{}00/paid", &path[..path.len() - 2])).await;

    assert_eq!(status.as_u16(), 404);
    let state: String = sqlx::query_scalar("select state from debtors")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(state, "active");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g3_pressing_i_dispute_this_records_the_dispute(pool: PgPool) {
    let clock = clock();
    let path = santos_page(&pool, &clock).await;
    let app = common::app(&pool, &clock);

    let (status, _) = common::post_page(&app, &format!("{path}/dispute")).await;
    let (_, page) = common::get_page(&app, &path).await;

    assert_eq!(status.as_u16(), 303);
    assert!(page.contains("Your dispute is noted"), "{page}");
    let state: String = sqlx::query_scalar("select state from debtors")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(state, "disputed");
}
