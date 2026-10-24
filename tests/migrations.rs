//! The engine's tables and Balanse's are created together.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

use sqlx::PgPool;

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn one_migrator_creates_the_engine_and_product_tables(pool: PgPool) {
    let tables: Vec<String> = sqlx::query_scalar(
        "select table_name::text from information_schema.tables
         where table_schema = 'public' and table_name in ('outbox_messages', 'accounts', 'businesses')
         order by table_name",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(tables, ["accounts", "businesses", "outbox_messages"]);
}
