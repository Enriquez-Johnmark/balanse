//! D1, D2, D4, D7, and C4, C5 and C7 at sending time: nothing goes out
//! unless the Owner approved that exact text and it still passes Balanse's
//! checks when it is sent.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use balanse::balanse::businesses::BusinessId;
use balanse::balanse::debtors::{DebtorId, DebtorInput, Debtors};
use balanse::balanse::debts::{DebtInput, Debts};
use balanse::balanse::drafts::approve_batch;
use balanse::balanse::presend::BalanseChecks;
use balanse::balanse::reminders::{ReplyAddresses, Scheduler};
use balanse::engine::clock::FixedClock;
use balanse::engine::files::MasterKey;
use balanse::engine::ids::MessageId;
use balanse::engine::mail::Email;
use balanse::engine::outbox::{MessageStatus, Outbox, OutboxError, Sender};
use balanse::engine::signed::Signer;
use common::{RecordingMailer, business, clock};
use sqlx::PgPool;
use time::Duration;

struct World {
    clock: Arc<FixedClock>,
    business: BusinessId,
    debtor: DebtorId,
    outbox: Outbox,
}

/// A Business with one Debtor, one overdue Debt and its first reminder
/// drafted (Monday 2026-11-02, 09:00 Manila).
async fn world(pool: &PgPool) -> (World, MessageId) {
    world_at(pool, Duration::ZERO).await
}

/// [`world`], with the reminder drafted `shift` from Monday 09:00.
async fn world_at(pool: &PgPool, shift: Duration) -> (World, MessageId) {
    let clock = clock();
    clock.advance(shift);
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
    Scheduler::new(pool.clone(), clock.clone(), replies)
        .run()
        .await
        .unwrap();
    let message: uuid::Uuid = sqlx::query_scalar("select message_id from reminders")
        .fetch_one(pool)
        .await
        .unwrap();
    let outbox = Outbox::new(pool.clone(), clock.clone());
    (
        World {
            clock,
            business,
            debtor,
            outbox,
        },
        message.into(),
    )
}

impl World {
    async fn approve(&self, pool: &PgPool, id: MessageId) -> Result<(), OutboxError> {
        let seen = self.outbox.get(self.business, id).await.unwrap().content_hash();
        self.outbox
            .approve(
                self.business,
                id,
                "ana@reyes.test",
                &seen,
                &BalanseChecks::for_approval(pool.clone()),
            )
            .await
    }

    async fn edit(&self, id: MessageId, change: impl FnOnce(&mut Email)) {
        let mut email = self.outbox.get(self.business, id).await.unwrap().email;
        change(&mut email);
        self.outbox
            .edit(self.business, id, &email, "ana@reyes.test")
            .await
            .unwrap();
    }

    /// Runs the sender once and returns what it sent.
    async fn drain(&self, pool: &PgPool) -> Vec<Email> {
        let mailer = RecordingMailer::default();
        let checks = Arc::new(BalanseChecks::for_sending(pool.clone(), self.clock.clone()));
        Sender::new(pool.clone(), mailer.clone(), self.clock.clone(), checks)
            .drain_once(10)
            .await
            .unwrap();
        mailer.sent()
    }
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d1_nothing_is_sent_without_approval(pool: PgPool) {
    let (world, _) = world(&pool).await;

    assert!(world.drain(&pool).await.is_empty());
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d1_an_approved_reminder_is_sent(pool: PgPool) {
    let (world, id) = world(&pool).await;
    world.approve(&pool, id).await.unwrap();

    let sent = world.drain(&pool).await;

    assert_eq!(sent.len(), 1);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d1_editing_an_approved_draft_voids_its_approval(pool: PgPool) {
    let (world, id) = world(&pool).await;
    world.approve(&pool, id).await.unwrap();

    world.edit(id, |email| email.subject.push_str(" (edited)")).await;

    assert_eq!(
        world.outbox.get(world.business, id).await.unwrap().status,
        MessageStatus::Draft
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d2_batch_approval_skips_a_draft_that_changed_since_it_was_shown(pool: PgPool) {
    let (world, id) = world(&pool).await;
    let seen = world.outbox.get(world.business, id).await.unwrap().content_hash();
    world.edit(id, |email| email.subject.push_str(" (edited)")).await;

    let results = approve_batch(
        &world.outbox,
        world.business,
        &[(id, seen)],
        "ana@reyes.test",
        &BalanseChecks::for_approval(pool.clone()),
    )
    .await;

    assert!(
        matches!(results[0].1, Err(OutboxError::StaleApproval)),
        "got {results:?}"
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d4_a_changed_recipient_is_refused(pool: PgPool) {
    let (world, id) = world(&pool).await;
    world
        .edit(id, |email| email.to_address = "boss@santos.test".to_owned())
        .await;

    let err = world.approve(&pool, id).await.unwrap_err();

    assert!(matches!(err, OutboxError::CheckFailed(_)), "got {err:?}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d4_a_second_recipient_is_refused(pool: PgPool) {
    let (world, id) = world(&pool).await;
    world
        .edit(id, |email| {
            email.to_address = "lito@santos.test, boss@santos.test".to_owned()
        })
        .await;

    let err = world.approve(&pool, id).await.unwrap_err();

    assert!(
        matches!(err, OutboxError::CheckFailed(_) | OutboxError::InvalidMessage(_)),
        "got {err:?}"
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d4_a_link_is_refused(pool: PgPool) {
    let (world, id) = world(&pool).await;
    world
        .edit(id, |email| email.body_text.push_str("\nPay here: gcash-pay.ph/reyes"))
        .await;

    let err = world.approve(&pool, id).await.unwrap_err();

    assert!(matches!(err, OutboxError::CheckFailed(_)), "got {err:?}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d4_a_changed_amount_is_refused(pool: PgPool) {
    let (world, id) = world(&pool).await;
    world
        .edit(id, |email| {
            email.body_text = email.body_text.replace("₱12,500.00", "₱15,000.00")
        })
        .await;

    let err = world.approve(&pool, id).await.unwrap_err();

    assert!(matches!(err, OutboxError::CheckFailed(_)), "got {err:?}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn d7_a_reminder_without_its_footer_is_refused(pool: PgPool) {
    let (world, id) = world(&pool).await;
    world
        .edit(id, |email| {
            email.body_text = email.body_text.split("--\nSent via Balanse").next().unwrap().to_owned();
        })
        .await;

    let err = world.approve(&pool, id).await.unwrap_err();

    assert!(matches!(err, OutboxError::CheckFailed(_)), "got {err:?}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn c5_a_reminder_approved_in_the_evening_waits_for_the_next_weekday_morning(pool: PgPool) {
    let (world, id) = world(&pool).await;
    world.clock.advance(Duration::hours(12)); // Monday 21:00 Manila
    world.approve(&pool, id).await.unwrap();
    let at_night = world.drain(&pool).await.len();
    world.clock.advance(Duration::hours(11)); // Tuesday 08:00 Manila

    let in_the_morning = world.drain(&pool).await.len();

    assert_eq!((at_night, in_the_morning), (0, 1));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn c4_an_approved_reminder_waits_after_a_message_sent_by_hand(pool: PgPool) {
    let (world, id) = world(&pool).await;
    sqlx::query(
        "insert into manual_sends (id, tenant_id, debtor_id, sent_at, logged_by, created_at)
         values (gen_random_uuid(), $1, $2, $3, 'ana@reyes.test', $3)",
    )
    .bind(world.business.as_uuid())
    .bind(world.debtor.as_uuid())
    .bind(world.clock.now_plus(-Duration::hours(1)))
    .execute(&pool)
    .await
    .unwrap();
    world.approve(&pool, id).await.unwrap();

    assert!(world.drain(&pool).await.is_empty());
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn c7_an_approved_reminder_isnt_sent_once_the_debtor_is_paused(pool: PgPool) {
    let (world, id) = world(&pool).await;
    world.approve(&pool, id).await.unwrap();
    sqlx::query("update debtors set state = 'paused' where id = $1")
        .bind(world.debtor.as_uuid())
        .execute(&pool)
        .await
        .unwrap();

    assert!(world.drain(&pool).await.is_empty());
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn c5_an_approved_reminder_still_goes_out_after_its_draft_deadline(pool: PgPool) {
    // Drafted Monday 07:00, so unapproved it would expire Thursday 07:00,
    // an hour before Thursday's sending hours begin.
    let (world, id) = world_at(&pool, -Duration::hours(2)).await;
    world.clock.advance(Duration::hours(62)); // Wednesday 21:00 Manila
    world.approve(&pool, id).await.unwrap();
    let at_night = world.drain(&pool).await.len();
    world.clock.advance(Duration::hours(11)); // Thursday 08:00 Manila
    let master = MasterKey::from_hex(&"ab".repeat(32)).unwrap();
    let replies = ReplyAddresses::new(Arc::new(Signer::new(&master, "test replies")), "reply.balanse.test");
    Scheduler::new(pool.clone(), world.clock.clone(), replies)
        .run()
        .await
        .unwrap();

    let in_the_morning = world.drain(&pool).await.len();

    assert_eq!((at_night, in_the_morning), (0, 1));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn c7_a_reminder_held_for_sending_hours_is_cancelled_by_the_owners_decision(pool: PgPool) {
    let (world, id) = world(&pool).await;
    world.clock.advance(Duration::hours(12)); // Monday 21:00 Manila
    world.approve(&pool, id).await.unwrap();
    world.drain(&pool).await; // held until Tuesday 08:00
    let outcomes = balanse::balanse::outcomes::Outcomes::new(pool.clone(), world.clock.clone());
    outcomes
        .disputed(world.business, world.debtor, "ana@reyes.test")
        .await
        .unwrap();
    world.clock.advance(Duration::hours(10)); // Tuesday 07:00
    outcomes
        .resume(world.business, world.debtor, "ana@reyes.test")
        .await
        .unwrap();
    world.clock.advance(Duration::hours(1)); // Tuesday 08:00

    let sent = world.drain(&pool).await;

    assert!(
        sent.is_empty(),
        "the reminder approved before the dispute stays cancelled"
    );
    assert_eq!(
        world.outbox.get(world.business, id).await.unwrap().status,
        MessageStatus::Cancelled
    );
}
