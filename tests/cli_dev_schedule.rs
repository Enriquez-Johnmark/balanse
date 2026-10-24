//! `balanse dev-schedule --date` drafts the reminders due on a chosen date,
//! so the whole loop can be tried (and browser-tested) before real dates
//! come round. It refuses to run against a real mail provider.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::process::{Command, Output};

use balanse::balanse::debtors::{DebtorInput, Debtors};
use balanse::balanse::debts::{DebtInput, Debts};
use common::{business, clock, database_url};
use sqlx::PgPool;

async fn dev_schedule(pool: &PgPool, provider: &str) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_balanse"));
    cmd.args(["dev-schedule", "--date", "2026-11-04"])
        .env("DATABASE_URL", database_url(pool).await)
        .env("MASTER_KEY", "ab".repeat(32))
        .env("MAIL_PROVIDER", provider)
        .env("RESEND_API_KEY", "re_test")
        .env("RUST_LOG", "warn");
    tokio::task::spawn_blocking(move || cmd.output().unwrap())
        .await
        .unwrap()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn dev_schedule_drafts_the_reminders_due_on_a_chosen_date(pool: PgPool) {
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
    Debts::new(pool.clone(), clock.clone())
        .create(
            reyes,
            debtor,
            &DebtInput {
                invoice_number: "INV-7".to_owned(),
                invoice_date: "2026-10-01".to_owned(),
                amount: "₱12,500.00".to_owned(),
                due_date: "2026-10-30".to_owned(),
                note: String::new(),
            },
        )
        .await
        .unwrap();

    let out = dev_schedule(&pool, "mailpit").await;

    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let drafted: Vec<String> = sqlx::query_scalar("select to_address from outbox_messages where status = 'draft'")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(drafted, ["lito@santos.test"]);
    assert!(String::from_utf8_lossy(&out.stdout).contains("1 reminder draft"));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn dev_schedule_only_runs_against_the_local_test_mailbox(pool: PgPool) {
    let out = dev_schedule(&pool, "resend").await;

    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("development only"));
}
