//! The operator's sign-up commands, run as the real binary.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::process::Command;

use balanse::engine::accounts::Signups;
use common::{clock, database_url};
use sqlx::PgPool;

async fn balanse(pool: &PgPool, args: &[&str]) -> String {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_balanse"));
    cmd.args(args)
        .env("DATABASE_URL", database_url(pool).await)
        .env("MASTER_KEY", "ab".repeat(32))
        .env("RUST_LOG", "warn");
    let output = tokio::task::spawn_blocking(move || cmd.output().unwrap())
        .await
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn signups_lists_waiting_requests(pool: PgPool) {
    Signups::new(pool.clone(), clock())
        .request("Reyes Creative", "ana@reyes.test", true)
        .await
        .unwrap();

    let listed = balanse(&pool, &["signups"]).await;

    assert!(
        listed.contains("Reyes Creative") && listed.contains("ana@reyes.test"),
        "{listed}"
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn approving_a_request_creates_the_business_and_emails_a_welcome_link(pool: PgPool) {
    let signups = Signups::new(pool.clone(), clock());
    signups.request("Reyes Creative", "ana@reyes.test", true).await.unwrap();
    let id = signups.pending().await.unwrap()[0].id;

    balanse(&pool, &["approve-signup", &id.to_string()]).await;

    let (status, subject): (String, String) =
        sqlx::query_as("select status, subject from outbox_messages where to_address = 'ana@reyes.test'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        (status.as_str(), subject.as_str()),
        ("approved", "Your Balanse account is ready")
    );
}
