//! The scheduler: each run drafts the reminders that are due (C1–C3, C7).
//! Every Draft waits for the Owner's approval; nothing here approves or
//! sends.
//!
//! - Drafts appear from 07:00 Manila time, and expire unapproved 3 days
//!   later (D3); each run also sweeps overdue Drafts to `expired`.
//! - Only active Debtors with open Debts get Drafts. A Promise that has run
//!   out starts a new cycle the day after it (F3).
//! - Each Draft is recorded in `reminders` in the same transaction that
//!   creates it; that table's constraints allow each step once per cycle.
//!
//! This is a system-wide worker: it reads every Business's Debtors, and
//! names the Business in every row it writes.

use std::sync::Arc;

use sqlx::PgPool;
use time::macros::{offset, time};
use time::{Date, Duration, OffsetDateTime, Time, UtcOffset};

use super::businesses::BusinessId;
use super::debtors::DebtorId;
use super::schedule::{Next, Plan, Step, plan};
use super::wording::{self, DebtLine, Reminder};
use crate::engine::clock::Clock;
use crate::engine::ids::id_type;
use crate::engine::mail::Email;
use crate::engine::outbox::{Outbox, OutboxError};
use crate::engine::signed::Signer;

id_type!(
    /// One reminder drafted for a Debtor.
    ReminderId
);

/// Manila's offset from UTC; the Philippines has no daylight saving.
pub const MANILA: UtcOffset = offset!(+8);
/// When each day's Drafts appear, Manila time (C1).
pub const DRAFTS_APPEAR_AT: Time = time!(07:00);
/// How long a Draft waits for approval before it expires (D3).
pub const DRAFT_LIFETIME: Duration = Duration::days(3);

/// Signed reply addresses, `reply+CODE@domain`, whose code names the
/// reminder replied to (F1).
#[derive(Clone)]
pub struct ReplyAddresses {
    signer: Arc<Signer>,
    domain: String,
}

impl ReplyAddresses {
    /// Addresses at `domain`, signed by `signer`.
    pub fn new(signer: Arc<Signer>, domain: &str) -> Self {
        Self {
            signer,
            domain: domain.trim().to_ascii_lowercase(),
        }
    }

    /// The reply address of `reminder`.
    pub fn address(&self, reminder: ReminderId) -> String {
        format!("reply+{}@{}", self.signer.token(reminder.as_uuid()), self.domain)
    }

    /// The reminder a reply address names, if its code verifies.
    pub fn reminder(&self, address: &str) -> Option<ReminderId> {
        let (local, domain) = address.trim().rsplit_once('@')?;
        if !domain.eq_ignore_ascii_case(&self.domain) {
            return None;
        }
        let code = local.strip_prefix("reply+")?;
        self.signer.verify(code).map(ReminderId::from)
    }
}

/// What one run did.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RunReport {
    /// Drafts created.
    pub drafted: usize,
    /// Debtors marked as needing the Owner's call.
    pub needs_call: usize,
    /// Debtors whose step waits for the cap.
    pub waiting: usize,
    /// Debtors that failed and will be tried again next run.
    pub failed: usize,
}

/// Scheduler errors.
#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    /// The outbox refused a Draft.
    #[error(transparent)]
    Outbox(#[from] OutboxError),
}

/// Creates the Drafts that are due.
pub struct Scheduler {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    outbox: Outbox,
    replies: ReplyAddresses,
}

/// An active Debtor with open Debts, as the run finds it.
struct Candidate {
    business: BusinessId,
    debtor: DebtorId,
    business_name: String,
    contact_name: String,
    contact_email: String,
    language: String,
    state: String,
    promised_until: Option<Date>,
    cycle_start: Option<Date>,
    heads_up: bool,
    due: Date,
    due_added: OffsetDateTime,
}

impl Scheduler {
    /// A scheduler over `pool`, giving each reminder a reply address from
    /// `replies`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, replies: ReplyAddresses) -> Self {
        Self {
            outbox: Outbox::new(pool.clone(), clock.clone()),
            pool,
            clock,
            replies,
        }
    }

    /// Expires overdue Drafts, then drafts every reminder due now.
    pub async fn run(&self) -> Result<RunReport, SchedulerError> {
        self.outbox.expire_overdue().await?;
        let mut report = RunReport::default();
        let now = self.clock.now();
        let local = now.to_offset(MANILA);
        if local.time() < DRAFTS_APPEAR_AT {
            return Ok(report);
        }
        for candidate in self.candidates().await? {
            match self.one(&candidate, now, local.date()).await {
                Ok(Next::Draft(_)) => report.drafted += 1,
                Ok(Next::NeedsCall) => report.needs_call += 1,
                Ok(Next::Wait) => report.waiting += 1,
                Ok(Next::Nothing) => {}
                Err(err) => {
                    report.failed += 1;
                    tracing::error!(business = %candidate.business, %err, "drafting a reminder failed");
                }
            }
        }
        Ok(report)
    }

    async fn candidates(&self) -> Result<Vec<Candidate>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"select d.id, d.tenant_id, a.name as business_name, d.contact_name, d.contact_email, d.language,
                      d.state, d.promised_until, d.cycle_start, b.heads_up,
                      min(t.due_date) as "due!",
                      (array_agg(t.created_at order by t.due_date, t.created_at))[1] as "due_added!"
               from debtors d
               join businesses b on b.tenant_id = d.tenant_id
               join accounts a on a.tenant_id = d.tenant_id and a.closed_at is null
               join debts t on t.tenant_id = d.tenant_id and t.debtor_id = d.id and t.status = 'open'
               where d.state in ('active', 'promised')
               group by d.id, a.name, b.heads_up
               order by d.tenant_id, d.id"#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| Candidate {
                business: row.tenant_id.into(),
                debtor: row.id.into(),
                business_name: row.business_name,
                contact_name: row.contact_name,
                contact_email: row.contact_email,
                language: row.language,
                state: row.state,
                promised_until: row.promised_until,
                cycle_start: row.cycle_start,
                heads_up: row.heads_up,
                due: row.due,
                due_added: row.due_added,
            })
            .collect())
    }

    async fn one(&self, c: &Candidate, now: OffsetDateTime, today: Date) -> Result<Next, SchedulerError> {
        let (business, debtor) = (c.business.as_uuid(), c.debtor.as_uuid());
        let mut cycle_start = c.cycle_start;
        if c.state == "promised" {
            let Some(until) = c.promised_until.filter(|until| today > *until) else {
                return Ok(Next::Nothing);
            };
            // The Promise ran out: a new cycle starts the day after it (F3).
            sqlx::query!(
                "update debtors set state = 'active', promised_until = null, cycle_start = $3, updated_at = $4
                 where tenant_id = $1 and id = $2 and state = 'promised'",
                business,
                debtor,
                until,
                now,
            )
            .execute(&self.pool)
            .await?;
            cycle_start = Some(until);
        }
        let cycle_start = match cycle_start {
            Some(start) => start,
            None => {
                // A Debt entered already overdue starts its cycle the day
                // before it was added, so its first reminder is the friendly
                // one, at once; but always after any earlier cycle, whose
                // steps would otherwise count as done.
                let fresh = c.due.max(c.due_added.to_offset(MANILA).date() - Duration::days(1));
                let last = sqlx::query_scalar!(
                    "select max(cycle_start) from reminders where tenant_id = $1 and debtor_id = $2",
                    business,
                    debtor,
                )
                .fetch_one(&self.pool)
                .await?;
                last.map_or(fresh, |last| fresh.max(last + Duration::days(1)))
            }
        };
        let done = sqlx::query_scalar!(
            "select step from reminders where tenant_id = $1 and debtor_id = $2 and cycle_start = $3",
            business,
            debtor,
            cycle_start,
        )
        .fetch_all(&self.pool)
        .await?
        .iter()
        .filter_map(|step| Step::parse(step))
        .collect();
        let waiting = sqlx::query_scalar!(
            r#"select exists (
                   select 1 from reminders r join outbox_messages m on m.id = r.message_id and m.tenant_id = r.tenant_id
                   where r.tenant_id = $1 and r.debtor_id = $2 and m.status in ('draft', 'approved', 'sending', 'failed')
               ) as "waiting!""#,
            business,
            debtor,
        )
        .fetch_one(&self.pool)
        .await?;
        let sent_on = sent_dates(&self.pool, c.business, c.debtor, now).await?;
        let next = plan(&Plan {
            today,
            cycle_start,
            due: c.due,
            heads_up: c.heads_up,
            done,
            waiting,
            sent_on,
        });
        match next {
            Next::Draft(step) => self.draft(c, step, cycle_start, now).await?,
            Next::NeedsCall => {
                sqlx::query!(
                    "update debtors set state = 'needs_call', updated_at = $3
                     where tenant_id = $1 and id = $2 and state = 'active'",
                    business,
                    debtor,
                    now,
                )
                .execute(&self.pool)
                .await?;
            }
            Next::Wait | Next::Nothing => {}
        }
        Ok(next)
    }

    async fn draft(
        &self,
        c: &Candidate,
        step: Step,
        cycle_start: Date,
        now: OffsetDateTime,
    ) -> Result<(), SchedulerError> {
        let debts = open_debts(&self.pool, c.business, c.debtor).await?;
        let wording = wording::reminder(
            &Reminder {
                step,
                language: &c.language,
                contact_name: &c.contact_name,
                business_name: &c.business_name,
            },
            &debts,
        );
        let id = ReminderId::new();
        let email = Email {
            to_address: c.contact_email.clone(),
            from_name: c.business_name.clone(),
            reply_to: Some(self.replies.address(id)),
            subject: wording.subject,
            body_text: wording.body,
        };
        let mut tx = self.pool.begin().await?;
        let message = self
            .outbox
            .create_draft_to_approve_by_in(&mut tx, c.business, &email, now + DRAFT_LIFETIME)
            .await?;
        sqlx::query!(
            "insert into reminders (id, tenant_id, debtor_id, cycle_start, step, message_id, created_at)
             values ($1, $2, $3, $4, $5, $6, $7)",
            id.as_uuid(),
            c.business.as_uuid(),
            c.debtor.as_uuid(),
            cycle_start,
            step.as_str(),
            message.as_uuid(),
            now,
        )
        .execute(&mut *tx)
        .await?;
        sqlx::query!(
            "update debtors set cycle_start = $3 where tenant_id = $1 and id = $2 and cycle_start is null",
            c.business.as_uuid(),
            c.debtor.as_uuid(),
            cycle_start,
        )
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }
}

/// `debtor`'s open Debts, earliest due first, as reminders list them.
pub async fn open_debts(pool: &PgPool, business: BusinessId, debtor: DebtorId) -> Result<Vec<DebtLine>, sqlx::Error> {
    let rows = sqlx::query!(
        "select invoice_number, invoice_date, amount, due_date from debts
         where tenant_id = $1 and debtor_id = $2 and status = 'open'
         order by due_date, invoice_number, id",
        business.as_uuid(),
        debtor.as_uuid(),
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| DebtLine {
            invoice_number: row.invoice_number,
            invoice_date: row.invoice_date,
            amount: row.amount,
            due_date: row.due_date,
        })
        .collect())
}

/// The Manila dates of every message to `debtor` in the 7 days before
/// `now`, sent by the engine or logged as sent by hand: what the cap counts
/// (C4).
pub async fn sent_dates(
    pool: &PgPool,
    business: BusinessId,
    debtor: DebtorId,
    now: OffsetDateTime,
) -> Result<Vec<Date>, sqlx::Error> {
    let sent = sqlx::query_scalar!(
        r#"select m.sent_at as "sent_at!" from reminders r
               join outbox_messages m on m.id = r.message_id and m.tenant_id = r.tenant_id
           where r.tenant_id = $1 and r.debtor_id = $2 and m.sent_at > $3
           union all
           select sent_at from manual_sends where tenant_id = $1 and debtor_id = $2 and sent_at > $3"#,
        business.as_uuid(),
        debtor.as_uuid(),
        now - Duration::days(7),
    )
    .fetch_all(pool)
    .await?;
    Ok(sent.into_iter().map(|at| at.to_offset(MANILA).date()).collect())
}

/// Cancels `debtor`'s reminders that still wait to be approved or sent,
/// after a reply or an Owner's decision, and returns which. One already
/// being sent is too late to stop. `actor` names who caused it in the audit
/// log.
pub async fn cancel_waiting(
    outbox: &Outbox,
    pool: &PgPool,
    business: BusinessId,
    debtor: DebtorId,
    actor: &str,
) -> Result<Vec<ReminderId>, OutboxError> {
    let waiting = sqlx::query!(
        "select r.id, r.message_id from reminders r
         join outbox_messages m on m.id = r.message_id and m.tenant_id = r.tenant_id
         where r.tenant_id = $1 and r.debtor_id = $2 and m.status in ('draft', 'approved')",
        business.as_uuid(),
        debtor.as_uuid(),
    )
    .fetch_all(pool)
    .await?;
    let mut cancelled = Vec::with_capacity(waiting.len());
    for row in waiting {
        match outbox.cancel(business, row.message_id.into(), actor).await {
            Ok(()) => cancelled.push(row.id.into()),
            Err(OutboxError::WrongStatus(_)) => {}
            Err(err) => return Err(err),
        }
    }
    Ok(cancelled)
}

/// After `debtor`'s Debts changed: cancels its waiting reminders, which no
/// longer list the Debts as they are, and forgets their steps, so the
/// scheduler drafts them again with the current Debts (B3). A reminder
/// never sent can have no replies, so nothing else goes with it.
pub async fn redraft_waiting(
    outbox: &Outbox,
    pool: &PgPool,
    business: BusinessId,
    debtor: DebtorId,
    actor: &str,
) -> Result<(), OutboxError> {
    let cancelled: Vec<_> = cancel_waiting(outbox, pool, business, debtor, actor)
        .await?
        .iter()
        .map(|id| id.as_uuid())
        .collect();
    sqlx::query!(
        "delete from reminders where tenant_id = $1 and id = any($2)",
        business.as_uuid(),
        &cancelled,
    )
    .execute(pool)
    .await?;
    Ok(())
}
