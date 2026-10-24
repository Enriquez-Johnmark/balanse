//! F1–F3: a Debtor's reply pauses its reminders and tells the Owner, who
//! then decides what happens next.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use balanse::balanse::businesses::BusinessId;
use balanse::balanse::debtors::{DebtorId, DebtorInput, Debtors};
use balanse::balanse::debts::{DebtInput, Debts};
use balanse::balanse::outcomes::Outcomes;
use balanse::balanse::reminders::{ReplyAddresses, Scheduler};
use balanse::balanse::replies::{Replies, ReplyOutcome};
use balanse::engine::clock::FixedClock;
use balanse::engine::files::MasterKey;
use balanse::engine::inbound::InboundEmail;
use balanse::engine::signed::Signer;
use common::{business, clock};
use sqlx::PgPool;
use time::Duration;
use time::macros::date;

struct World {
    clock: Arc<FixedClock>,
    business: BusinessId,
    debtor: DebtorId,
    replies: ReplyAddresses,
    scheduler: Scheduler,
    /// The first reminder's reply address.
    reply_to: String,
}

/// A Business with one Debtor, one overdue Debt and its first reminder
/// drafted (Monday 2026-11-02, 09:00 Manila).
async fn world(pool: &PgPool) -> World {
    let clock = clock();
    let business = business(pool, &clock).await;
    let debtor = Debtors::new(pool.clone(), clock.clone())
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
        .unwrap();
    Debts::new(pool.clone(), clock.clone())
        .create(
            business,
            debtor,
            &DebtInput {
                invoice_number: "INV-1".to_owned(),
                invoice_date: "2026-09-30".to_owned(),
                amount: "₱12,500.00".to_owned(),
                due_date: "2026-10-30".to_owned(),
                note: String::new(),
            },
        )
        .await
        .unwrap();
    let master = MasterKey::from_hex(&"ab".repeat(32)).unwrap();
    let replies = ReplyAddresses::new(Arc::new(Signer::new(&master, "test replies")), "reply.balanse.test");
    let scheduler = Scheduler::new(pool.clone(), clock.clone(), replies.clone());
    scheduler.run().await.unwrap();
    let reply_to: String = sqlx::query_scalar("select reply_to from outbox_messages")
        .fetch_one(pool)
        .await
        .unwrap();
    World {
        clock,
        business,
        debtor,
        replies,
        scheduler,
        reply_to,
    }
}

impl World {
    fn inbox(&self, pool: &PgPool) -> Replies {
        Replies::new(
            pool.clone(),
            self.clock.clone(),
            self.replies.clone(),
            "https://app.balanse.test",
        )
    }

    fn outcomes(&self, pool: &PgPool) -> Outcomes {
        Outcomes::new(pool.clone(), self.clock.clone())
    }
}

fn reply(to: &str, automatic: bool) -> InboundEmail {
    InboundEmail {
        key: "<abc@santos.test>".to_owned(),
        from: "lito@santos.test".to_owned(),
        recipients: vec![to.to_owned()],
        subject: "Re: Reminder".to_owned(),
        text: "Pasensya na po, babayaran ko sa Friday. Click here: evil.example".to_owned(),
        automatic,
        attachments: Vec::new(),
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
    sqlx::query_scalar(
        "select count(*) from reminders r join outbox_messages m on m.id = r.message_id where m.status = 'draft'",
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f1_a_human_reply_pauses_the_debtor_and_cancels_the_waiting_draft(pool: PgPool) {
    let world = world(&pool).await;

    let outcome = world
        .inbox(&pool)
        .receive(&reply(&world.reply_to, false))
        .await
        .unwrap();

    assert_eq!(
        (outcome, state(&pool, world.debtor).await.as_str(), drafts(&pool).await),
        (ReplyOutcome::Paused, "paused", 0)
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f1_the_owner_is_told_without_the_replys_text(pool: PgPool) {
    let world = world(&pool).await;

    world
        .inbox(&pool)
        .receive(&reply(&world.reply_to, false))
        .await
        .unwrap();

    let (status, body): (String, String) =
        sqlx::query_as("select status, body_text from outbox_messages where to_address = 'ana@reyes.test'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(
        status == "approved" && body.contains("Santos Trading") && !body.contains("Pasensya") && !body.contains("evil"),
        "got {status}: {body}"
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f1_mail_to_a_forged_reply_address_is_ignored(pool: PgPool) {
    let world = world(&pool).await;

    let outcome = world
        .inbox(&pool)
        .receive(&reply(
            "reply+00000000000000000000000000000000@reply.balanse.test",
            false,
        ))
        .await
        .unwrap();

    assert_eq!(
        (outcome, state(&pool, world.debtor).await.as_str()),
        (ReplyOutcome::Unmatched, "active")
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f1_the_same_reply_delivered_twice_is_kept_once(pool: PgPool) {
    let world = world(&pool).await;
    let inbox = world.inbox(&pool);
    inbox.receive(&reply(&world.reply_to, false)).await.unwrap();

    let again = inbox.receive(&reply(&world.reply_to, false)).await.unwrap();

    let kept: i64 = sqlx::query_scalar("select count(*) from replies")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!((again, kept), (ReplyOutcome::Duplicate, 1));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f2_an_automatic_reply_doesnt_pause_reminders(pool: PgPool) {
    let world = world(&pool).await;

    let outcome = world.inbox(&pool).receive(&reply(&world.reply_to, true)).await.unwrap();

    assert_eq!(
        (outcome, state(&pool, world.debtor).await.as_str(), drafts(&pool).await),
        (ReplyOutcome::Automatic, "active", 1)
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f3_a_promise_resumes_reminders_the_day_after_it(pool: PgPool) {
    let world = world(&pool).await;
    world
        .inbox(&pool)
        .receive(&reply(&world.reply_to, false))
        .await
        .unwrap();
    world
        .outcomes(&pool)
        .promised(world.business, world.debtor, date!(2026 - 11 - 10), "ana@reyes.test")
        .await
        .unwrap();
    world.clock.advance(Duration::days(8)); // Tuesday 2026-11-10
    world.scheduler.run().await.unwrap();
    let on_the_day = drafts(&pool).await;
    world.clock.advance(Duration::days(1)); // Wednesday 2026-11-11

    world.scheduler.run().await.unwrap();

    assert_eq!((on_the_day, drafts(&pool).await), (0, 1));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f3_a_dispute_stops_reminders(pool: PgPool) {
    let world = world(&pool).await;
    world
        .inbox(&pool)
        .receive(&reply(&world.reply_to, false))
        .await
        .unwrap();
    world
        .outcomes(&pool)
        .disputed(world.business, world.debtor, "ana@reyes.test")
        .await
        .unwrap();
    world.clock.advance(Duration::days(9));

    world.scheduler.run().await.unwrap();

    assert_eq!(
        (state(&pool, world.debtor).await.as_str(), drafts(&pool).await),
        ("disputed", 0)
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f3_paid_settles_the_debtors_open_debts(pool: PgPool) {
    let world = world(&pool).await;
    world
        .inbox(&pool)
        .receive(&reply(&world.reply_to, false))
        .await
        .unwrap();
    world
        .outcomes(&pool)
        .paid(world.business, world.debtor, "ana@reyes.test")
        .await
        .unwrap();
    world.clock.advance(Duration::days(9));

    world.scheduler.run().await.unwrap();

    let open: i64 = sqlx::query_scalar("select count(*) from debts where status = 'open'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!((open, drafts(&pool).await), (0, 0));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn f3_resume_restarts_the_schedule_the_next_day(pool: PgPool) {
    let world = world(&pool).await;
    world
        .inbox(&pool)
        .receive(&reply(&world.reply_to, false))
        .await
        .unwrap();
    world.clock.advance(Duration::days(3)); // Thursday 2026-11-05
    world
        .outcomes(&pool)
        .resume(world.business, world.debtor, "ana@reyes.test")
        .await
        .unwrap();
    world.scheduler.run().await.unwrap();
    let same_day = drafts(&pool).await;
    world.clock.advance(Duration::days(1)); // Friday

    world.scheduler.run().await.unwrap();

    assert_eq!((same_day, drafts(&pool).await), (0, 1));
}
