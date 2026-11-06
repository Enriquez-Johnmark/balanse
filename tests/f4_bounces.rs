//! F4: a permanent bounce or complaint stops all reminders to that address
//! until the Owner changes it; delivery events count only when they match a
//! message Balanse sent.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use balanse::balanse::bounces::{Bounces, Recorded};
use balanse::balanse::debtors::DebtorId;
use balanse::engine::webhooks::DeliveryEvent;
use common::{clock, scheduler, sent};
use sqlx::PgPool;
use time::Duration;

fn bounced(id: &str, permanent: bool) -> DeliveryEvent {
    DeliveryEvent::Bounced {
        provider_message_id: id.to_owned(),
        permanent,
        reason: "mailbox does not exist".to_owned(),
    }
}

async fn state(pool: &PgPool, debtor: DebtorId) -> String {
    sqlx::query_scalar("select state from debtors where id = $1")
        .bind(debtor.as_uuid())
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn drafts(pool: &PgPool) -> i64 {
    sqlx::query_scalar("select count(*) from outbox_messages where status = 'draft'")
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f4_a_permanent_bounce_stops_the_debtors_reminders(pool: PgPool) {
    let clock = clock();
    let (_, debtor) = sent(&pool, &clock).await;

    let recorded = Bounces::new(pool.clone(), clock.clone())
        .record(&bounced("re_1", true))
        .await
        .unwrap();
    // A week later, when the next reminder would be due.
    clock.advance(Duration::days(7));
    scheduler(&pool, &clock).run().await.unwrap();

    assert_eq!(recorded, Recorded::Stopped);
    assert_eq!(state(&pool, debtor).await, "bounced");
    assert_eq!(drafts(&pool).await, 0);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f4_a_complaint_stops_the_debtors_reminders(pool: PgPool) {
    let clock = clock();
    let (_, debtor) = sent(&pool, &clock).await;

    let recorded = Bounces::new(pool.clone(), clock.clone())
        .record(&DeliveryEvent::Complained {
            provider_message_id: "re_1".to_owned(),
        })
        .await
        .unwrap();

    assert_eq!(
        (recorded, state(&pool, debtor).await.as_str()),
        (Recorded::Stopped, "bounced")
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f4_a_temporary_bounce_is_only_noted(pool: PgPool) {
    let clock = clock();
    let (_, debtor) = sent(&pool, &clock).await;

    let recorded = Bounces::new(pool.clone(), clock.clone())
        .record(&bounced("re_1", false))
        .await
        .unwrap();

    assert_eq!(
        (recorded, state(&pool, debtor).await.as_str()),
        (Recorded::Noted, "active")
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f4_an_event_for_a_message_balanse_didnt_send_is_ignored(pool: PgPool) {
    let clock = clock();
    let (_, debtor) = sent(&pool, &clock).await;

    let recorded = Bounces::new(pool.clone(), clock.clone())
        .record(&bounced("re_someone_else", true))
        .await
        .unwrap();

    assert_eq!(
        (recorded, state(&pool, debtor).await.as_str()),
        (Recorded::Ignored, "active")
    );
    let kept: i64 = sqlx::query_scalar("select count(*) from bounces")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(kept, 0);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f4_a_late_bounce_for_a_replaced_address_stops_nothing(pool: PgPool) {
    let clock = clock();
    let (_, debtor) = sent(&pool, &clock).await;
    sqlx::query("update debtors set contact_email = 'lito@new.test' where id = $1")
        .bind(debtor.as_uuid())
        .execute(&pool)
        .await
        .unwrap();

    let recorded = Bounces::new(pool.clone(), clock.clone())
        .record(&bounced("re_1", true))
        .await
        .unwrap();

    assert_eq!(
        (recorded, state(&pool, debtor).await.as_str()),
        (Recorded::Noted, "active")
    );
}
