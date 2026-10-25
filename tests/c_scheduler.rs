//! C1–C3, C7, D3, D7: the scheduler turns each active Debtor's plan into a
//! Draft waiting for the Owner's approval.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use balanse::balanse::businesses::BusinessId;
use balanse::balanse::debtors::{DebtorId, DebtorInput, Debtors};
use balanse::balanse::debts::{DebtInput, Debts};
use balanse::balanse::reminders::Scheduler;
use balanse::engine::clock::{Clock, FixedClock};
use common::{business, clock};
use sqlx::PgPool;
use time::Duration;
use time::macros::datetime;

fn scheduler(pool: &PgPool, clock: &Arc<FixedClock>) -> Scheduler {
    common::scheduler(pool, &clock)
}

async fn debtor(pool: &PgPool, clock: &Arc<FixedClock>, business: BusinessId) -> DebtorId {
    Debtors::new(pool.clone(), clock.clone())
        .create(
            business,
            &DebtorInput {
                business_name: "Santos Trading".to_owned(),
                contact_name: "Lito".to_owned(),
                contact_email: "lito@santos.test".to_owned(),
                contact_mobile: String::new(),
                language: "english".to_owned(),
            },
        )
        .await
        .unwrap()
}

/// Adds an open Debt `invoice`, due Friday 2026-10-30.
async fn debt(pool: &PgPool, clock: &Arc<FixedClock>, business: BusinessId, debtor: DebtorId, invoice: &str) {
    Debts::new(pool.clone(), clock.clone())
        .create(
            business,
            debtor,
            &DebtInput {
                invoice_number: invoice.to_owned(),
                invoice_date: "2026-09-30".to_owned(),
                amount: "₱12,500.00".to_owned(),
                due_date: "2026-10-30".to_owned(),
                note: "never in the email".to_owned(),
            },
        )
        .await
        .unwrap();
}

async fn drafts(pool: &PgPool) -> Vec<(String, String, String)> {
    sqlx::query_as(
        "select to_address, subject, body_text from outbox_messages where status = 'draft' order by created_at",
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn c1_a_draft_appears_at_seven_in_the_morning_manila_time(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = debtor(&pool, &clock, business).await;
    debt(&pool, &clock, business, debtor, "INV-1").await;
    let scheduler = scheduler(&pool, &clock);
    // From Monday 09:00 to Tuesday 2026-11-03, 06:59 and then 07:00 in Manila.
    clock.advance(Duration::hours(21) + Duration::minutes(59));
    assert_eq!(clock.now(), datetime!(2026-11-02 22:59 UTC));
    scheduler.run().await.unwrap();
    let before = drafts(&pool).await.len();
    clock.advance(Duration::minutes(1));

    scheduler.run().await.unwrap();

    assert_eq!((before, drafts(&pool).await.len()), (0, 1));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn c2_one_draft_lists_every_overdue_debt(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = debtor(&pool, &clock, business).await;
    debt(&pool, &clock, business, debtor, "INV-1").await;
    debt(&pool, &clock, business, debtor, "INV-2").await;

    scheduler(&pool, &clock).run().await.unwrap();

    let drafts = drafts(&pool).await;
    assert!(
        drafts.len() == 1 && drafts[0].2.contains("INV-1") && drafts[0].2.contains("INV-2"),
        "got {drafts:?}"
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn c2_a_draft_goes_to_the_debtors_contact_and_leaves_out_the_note(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = debtor(&pool, &clock, business).await;
    debt(&pool, &clock, business, debtor, "INV-1").await;

    scheduler(&pool, &clock).run().await.unwrap();

    let (to, _, body) = drafts(&pool).await.remove(0);
    assert!(
        to == "lito@santos.test" && body.contains("₱12,500.00") && !body.contains("never in the email"),
        "got {to} / {body}"
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn c2_running_again_creates_no_second_draft(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = debtor(&pool, &clock, business).await;
    debt(&pool, &clock, business, debtor, "INV-1").await;
    let scheduler = scheduler(&pool, &clock);
    scheduler.run().await.unwrap();

    scheduler.run().await.unwrap();

    assert_eq!(drafts(&pool).await.len(), 1);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn c3_after_the_third_reminder_the_debtor_needs_the_owners_call(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = debtor(&pool, &clock, business).await;
    debt(&pool, &clock, business, debtor, "INV-1").await;
    let scheduler = scheduler(&pool, &clock);
    // Each Draft expires unapproved; the step counts as done (D3).
    for _ in 0..30 {
        scheduler.run().await.unwrap();
        clock.advance(Duration::days(1));
    }

    let state: String = sqlx::query_scalar("select state from debtors where id = $1")
        .bind(debtor.as_uuid())
        .fetch_one(&pool)
        .await
        .unwrap();
    let steps: i64 = sqlx::query_scalar("select count(*) from reminders")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!((state.as_str(), steps), ("needs_call", 3));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn c7_no_draft_for_a_debtor_that_isnt_active(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let scheduler = scheduler(&pool, &clock);
    for state in [
        "paused",
        "needs_call",
        "payment_claimed",
        "disputed",
        "opted_out",
        "bounced",
    ] {
        let debtor = debtor(&pool, &clock, business).await;
        debt(&pool, &clock, business, debtor, "INV-1").await;
        sqlx::query("update debtors set state = $2 where id = $1")
            .bind(debtor.as_uuid())
            .bind(state)
            .execute(&pool)
            .await
            .unwrap();
    }
    let promised = debtor(&pool, &clock, business).await;
    debt(&pool, &clock, business, promised, "INV-1").await;
    sqlx::query("update debtors set state = 'promised', promised_until = '2026-11-20' where id = $1")
        .bind(promised.as_uuid())
        .execute(&pool)
        .await
        .unwrap();
    debtor(&pool, &clock, business).await; // settled: no open Debts

    scheduler.run().await.unwrap();

    assert_eq!(drafts(&pool).await.len(), 0);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d3_a_draft_expires_three_days_after_it_appears(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = debtor(&pool, &clock, business).await;
    debt(&pool, &clock, business, debtor, "INV-1").await;

    scheduler(&pool, &clock).run().await.unwrap();

    let expires: time::OffsetDateTime = sqlx::query_scalar("select expires_at from outbox_messages")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(expires, clock.now_plus(Duration::days(3)));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d7_a_draft_names_the_business_and_says_it_was_sent_via_balanse(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = debtor(&pool, &clock, business).await;
    debt(&pool, &clock, business, debtor, "INV-1").await;

    scheduler(&pool, &clock).run().await.unwrap();

    let (from, reply_to, body): (String, Option<String>, String) =
        sqlx::query_as("select from_name, reply_to, body_text from outbox_messages")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(
        from == "Reyes Creative"
            && body.contains("Sent via Balanse on behalf of Reyes Creative")
            && reply_to.is_some_and(|r| r.starts_with("reply+") && r.ends_with("@reply.balanse.test")),
        "got {from} / {body}"
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn c1_a_debt_added_after_paid_starts_a_cycle_of_its_own(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = debtor(&pool, &clock, business).await;
    debt(&pool, &clock, business, debtor, "INV-1").await;
    let scheduler = scheduler(&pool, &clock);
    scheduler.run().await.unwrap(); // INV-1's first reminder
    balanse::balanse::outcomes::Outcomes::new(pool.clone(), clock.clone())
        .paid(business, debtor, "ana@reyes.test")
        .await
        .unwrap();
    // A new overdue invoice the same day, as overdue as the last one was.
    debt(&pool, &clock, business, debtor, "INV-2").await;
    scheduler.run().await.unwrap();
    clock.advance(Duration::days(1));

    scheduler.run().await.unwrap();

    let steps: Vec<String> = sqlx::query_scalar(
        "select r.step from reminders r join outbox_messages m on m.id = r.message_id
         where m.status = 'draft'",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(steps, ["first"], "INV-2 gets its own friendly first reminder");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b3_a_changed_amount_redrafts_the_waiting_reminder_with_it(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = debtor(&pool, &clock, business).await;
    debt(&pool, &clock, business, debtor, "INV-1").await;
    let scheduler = scheduler(&pool, &clock);
    scheduler.run().await.unwrap(); // the first reminder, for ₱12,500.00
    let debts = Debts::new(pool.clone(), clock.clone());
    let debt = debts.for_debtor(business, debtor).await.unwrap()[0].id;

    debts
        .change_amount(business, debt, "₱5,000.00", "ana@reyes.test")
        .await
        .unwrap();
    scheduler.run().await.unwrap();

    let drafts = drafts(&pool).await;
    assert!(
        drafts.len() == 1 && drafts[0].2.contains("₱5,000.00") && !drafts[0].2.contains("₱12,500.00"),
        "got {drafts:?}"
    );
    let step: String = sqlx::query_scalar(
        "select r.step from reminders r join outbox_messages m on m.id = r.message_id where m.status = 'draft'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(step, "first", "still the friendly first reminder");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g1_every_reminder_carries_the_debtors_page_link(pool: PgPool) {
    let clock = clock();
    let business = business(&pool, &clock).await;
    let debtor = debtor(&pool, &clock, business).await;
    debt(&pool, &clock, business, debtor, "INV-1").await;

    scheduler(&pool, &clock).run().await.unwrap();

    let urls = common::links(&pool, &clock)
        .current_urls(business, debtor)
        .await
        .unwrap();
    let drafts = drafts(&pool).await;
    assert!(
        urls.len() == 1 && drafts[0].2.contains(&urls[0]),
        "{urls:?} in {drafts:?}"
    );
}
