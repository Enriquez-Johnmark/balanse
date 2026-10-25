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
