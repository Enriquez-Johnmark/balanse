//! F5: reply text is deleted 90 days after it arrives; only "replied on
//! <date>" remains.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use balanse::balanse::retention::Retention;
use balanse::engine::clock::Clock;
use common::{clock, sent};
use sqlx::PgPool;
use time::Duration;

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f5_reply_text_is_deleted_90_days_after_it_arrives(pool: PgPool) {
    let clock = clock();
    let (business, debtor) = sent(&pool, &clock).await;
    let reminder: uuid::Uuid = sqlx::query_scalar("select id from reminders")
        .fetch_one(&pool)
        .await
        .unwrap();
    for (key, age) in [("old", 90), ("recent", 89)] {
        sqlx::query(
            "insert into replies (id, tenant_id, debtor_id, reminder_id, from_address, subject, body_text,
                                  automatic, received_at, message_key)
             values (gen_random_uuid(), $1, $2, $3, 'lito@santos.test', 'Re: invoice', 'Bayad na po',
                     false, $4, $5)",
        )
        .bind(business.as_uuid())
        .bind(debtor.as_uuid())
        .bind(reminder)
        .bind(clock.now() - Duration::days(age))
        .bind(key)
        .execute(&pool)
        .await
        .unwrap();
    }

    Retention::new(pool.clone(), clock.clone()).sweep().await.unwrap();

    let kept: Vec<(String, String, Option<String>)> =
        sqlx::query_as("select message_key, subject, body_text from replies order by message_key")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(
        kept,
        [
            ("old".to_owned(), String::new(), None),
            (
                "recent".to_owned(),
                "Re: invoice".to_owned(),
                Some("Bayad na po".to_owned())
            ),
        ]
    );
}
