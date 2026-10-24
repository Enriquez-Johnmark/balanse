//! Balanse's pre-send checks. The engine runs them when the Owner approves a
//! message and again just before sending it; a message that fails at
//! approval isn't approved, and one that fails at sending is blocked.
//!
//! A reminder must (D4, D7, R2–R3, R9):
//! - go to its Debtor's stored Contact, one address only;
//! - be for a Debtor that is still active (C7);
//! - carry no link;
//! - list the Debtor's open Debts exactly as Balanse writes them;
//! - come from the Business's name and keep Balanse's footer.
//!
//! At sending time only, it must also fall on a weekday between 08:00 and
//! 20:00 Manila time (C5) and within the cap (C4). Those make it wait, not
//! fail: the engine tries again shortly.
//!
//! Any other message must be an account email (a login link) to a member of
//! the Business.

use std::sync::Arc;

use sqlx::PgPool;
use time::macros::time;
use time::{Duration, Weekday};

use super::reminders::{MANILA, open_debts, sent_since};
use super::schedule::cap_allows;
use super::wording::{debt_block, footer};
use crate::engine::accounts::is_email_address;
use crate::engine::clock::Clock;
use crate::engine::ids::{MessageId, TenantId};
use crate::engine::mail::Email;
use crate::engine::outbox::{BoxFuture, CheckFailed, PreSendCheck, links};

/// Balanse's rules for one message.
#[derive(Clone)]
pub struct BalanseChecks {
    pool: PgPool,
    /// Set when checking at sending time, for the time rules.
    sending: Option<Arc<dyn Clock>>,
}

impl BalanseChecks {
    /// The rules at approval: everything but the time of day and the cap,
    /// so an Owner can approve in the evening (C5).
    pub fn for_approval(pool: PgPool) -> Self {
        Self { pool, sending: None }
    }

    /// The rules at sending: everything, including the time of day and the
    /// cap.
    pub fn for_sending(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self {
            pool,
            sending: Some(clock),
        }
    }

    async fn run(&self, tenant: TenantId, id: MessageId, email: &Email) -> Result<(), CheckFailed> {
        let unavailable = |err: sqlx::Error| CheckFailed::Unavailable(err.to_string());
        let refuse = |reason: String| Err(CheckFailed::Refused(reason));
        let reminder = sqlx::query!(
            "select r.debtor_id, d.state, d.contact_email, a.name as business_name
             from reminders r
             join debtors d on d.tenant_id = r.tenant_id and d.id = r.debtor_id
             join accounts a on a.tenant_id = r.tenant_id
             where r.tenant_id = $1 and r.message_id = $2",
            tenant.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(unavailable)?;
        let Some(reminder) = reminder else {
            return self.account_email(tenant, email).await;
        };
        let to = email.to_address.trim();
        if !is_email_address(to) {
            return refuse(format!("{to:?} isn't one email address"));
        }
        if !to.eq_ignore_ascii_case(&reminder.contact_email) {
            return refuse(format!("it goes to {to}, not to the Debtor's Contact"));
        }
        if reminder.state != "active" {
            return refuse(format!("the Debtor is {}", reminder.state.replace('_', " ")));
        }
        if !links(&email.subject).is_empty() || !links(&email.body_text).is_empty() {
            return refuse("reminders can't contain links".to_owned());
        }
        if email.from_name != reminder.business_name {
            return refuse("it must come from the Business's name".to_owned());
        }
        let debtor = reminder.debtor_id.into();
        let debts = open_debts(&self.pool, tenant, debtor).await.map_err(unavailable)?;
        if debts.is_empty() {
            return refuse("the Debtor has no open Debts".to_owned());
        }
        if !email.body_text.contains(&debt_block(&debts)) {
            return refuse("the list of invoices or amounts differs from the Debts on record".to_owned());
        }
        if !email.body_text.contains(&footer(&reminder.business_name)) {
            return refuse("Balanse's footer is missing".to_owned());
        }
        let Some(clock) = &self.sending else {
            return Ok(());
        };
        let now = clock.now();
        let local = now.to_offset(MANILA);
        let weekday = !matches!(local.weekday(), Weekday::Saturday | Weekday::Sunday);
        if !weekday || local.time() < time!(08:00) || local.time() >= time!(20:00) {
            return Err(CheckFailed::Unavailable(
                "reminders go out on weekdays, 08:00 to 20:00 Manila time".to_owned(),
            ));
        }
        let sent_on: Vec<_> = sent_since(&self.pool, tenant, debtor, now - Duration::days(7))
            .await
            .map_err(unavailable)?
            .into_iter()
            .map(|at| at.to_offset(MANILA).date())
            .collect();
        if !cap_allows(local.date(), &sent_on) {
            return Err(CheckFailed::Unavailable(
                "the Debtor had a message recently; this one waits".to_owned(),
            ));
        }
        Ok(())
    }

    /// A message that isn't a reminder: only an account email to one of the
    /// Business's members.
    async fn account_email(&self, tenant: TenantId, email: &Email) -> Result<(), CheckFailed> {
        let member = sqlx::query_scalar!(
            r#"select exists (
                   select 1 from account_members where tenant_id = $1 and lower(email) = lower($2)
               ) as "member!""#,
            tenant.as_uuid(),
            email.to_address.trim(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| CheckFailed::Unavailable(err.to_string()))?;
        if member {
            Ok(())
        } else {
            Err(CheckFailed::Refused(
                "only reminders and account emails go out".to_owned(),
            ))
        }
    }
}

impl PreSendCheck for BalanseChecks {
    fn check<'a>(
        &'a self,
        tenant: TenantId,
        id: MessageId,
        email: &'a Email,
    ) -> BoxFuture<'a, Result<(), CheckFailed>> {
        Box::pin(self.run(tenant, id, email))
    }
}
