//! A Debtor's evidence log (H3): everything Balanse did and saw for it, in
//! time order: the Debts added, each reminder's approved text and who
//! approved it, when it was sent and to whom, messages sent by hand,
//! delivery problems, when replies came (never their text), and every
//! change of status. The Owner exports it as a CSV file; nothing in it is a
//! total, an aging breakdown or an invoice (H2).

use sqlx::PgPool;
use time::OffsetDateTime;
use time::macros::format_description;

use super::businesses::BusinessId;
use super::debtors::DebtorId;
use super::reminders::MANILA;

/// Evidence-log errors.
#[derive(Debug, thiserror::Error)]
pub enum EvidenceError {
    /// No such Debtor in this Business.
    #[error("debtor not found")]
    NotFound,
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// One line of the log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// When it happened.
    pub at: OffsetDateTime,
    /// What happened, in the Owner's words.
    pub what: String,
    /// Who did it: an Owner's address, "Balanse" or "The Contact"; blank
    /// when not recorded.
    pub by: String,
    /// The detail: an approved text, an address, an amount.
    pub details: String,
}

/// Builds evidence logs.
#[derive(Clone)]
pub struct Evidence {
    pool: PgPool,
}

impl Evidence {
    /// Evidence logs from `pool`.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// The evidence log of `debtor`, one of `business`'s Debtors, oldest
    /// first.
    pub async fn for_debtor(&self, business: BusinessId, debtor: DebtorId) -> Result<Vec<Entry>, EvidenceError> {
        let (tenant, id) = (business.as_uuid(), debtor.as_uuid());
        let exists = sqlx::query_scalar!(
            r#"select exists (select 1 from debtors where tenant_id = $1 and id = $2) as "exists!""#,
            tenant,
            id,
        )
        .fetch_one(&self.pool)
        .await?;
        if !exists {
            return Err(EvidenceError::NotFound);
        }
        // Each entry carries a rank, so things logged at the same moment
        // keep their natural order (a Debt before the reminder about it).
        let mut log: Vec<(OffsetDateTime, u8, Entry)> = Vec::new();
        let mut push = |at: OffsetDateTime, rank: u8, what: String, by: &str, details: String| {
            log.push((
                at,
                rank,
                Entry {
                    at,
                    what,
                    by: by_label(by),
                    details,
                },
            ));
        };
        let date = format_description!("[day padding:none] [month repr:short] [year]");
        for debt in sqlx::query!(
            "select invoice_number, amount, due_date, created_at from debts where tenant_id = $1 and debtor_id = $2",
            tenant,
            id,
        )
        .fetch_all(&self.pool)
        .await?
        {
            let due = debt.due_date.format(date).unwrap_or_default();
            push(
                debt.created_at,
                0,
                format!("Invoice {} added", debt.invoice_number),
                "",
                format!("{}, due {due}", debt.amount),
            );
        }
        for reminder in sqlx::query!(
            "select m.subject, m.body_text, m.to_address, m.approved_at, m.approved_by, m.sent_at
             from reminders r join outbox_messages m on m.tenant_id = r.tenant_id and m.id = r.message_id
             where r.tenant_id = $1 and r.debtor_id = $2",
            tenant,
            id,
        )
        .fetch_all(&self.pool)
        .await?
        {
            if let Some(at) = reminder.approved_at {
                push(
                    at,
                    1,
                    "Reminder approved".to_owned(),
                    reminder.approved_by.as_deref().unwrap_or_default(),
                    format!("Subject: {}\n\n{}", reminder.subject, reminder.body_text),
                );
            }
            if let Some(at) = reminder.sent_at {
                push(
                    at,
                    2,
                    "Reminder sent".to_owned(),
                    "system",
                    format!("To {}", reminder.to_address),
                );
            }
        }
        for sent in sqlx::query!(
            "select sent_at, logged_by from manual_sends where tenant_id = $1 and debtor_id = $2",
            tenant,
            id,
        )
        .fetch_all(&self.pool)
        .await?
        {
            push(
                sent.sent_at,
                2,
                "Sent a reminder by hand".to_owned(),
                &sent.logged_by,
                String::new(),
            );
        }
        for bounce in sqlx::query!(
            "select address, kind, reason, at from bounces where tenant_id = $1 and debtor_id = $2",
            tenant,
            id,
        )
        .fetch_all(&self.pool)
        .await?
        {
            let what = match bounce.kind.as_str() {
                "bounce" => "Email bounced",
                "soft_bounce" => "Email bounced for now",
                "complaint" => "Email marked as spam",
                other => other,
            };
            let details = match bounce.reason {
                Some(reason) if !reason.is_empty() => format!("{}: {reason}", bounce.address),
                _ => bounce.address,
            };
            push(bounce.at, 3, what.to_owned(), "system", details);
        }
        for reply in sqlx::query!(
            "select from_address, automatic, received_at from replies where tenant_id = $1 and debtor_id = $2",
            tenant,
            id,
        )
        .fetch_all(&self.pool)
        .await?
        {
            let what = if reply.automatic {
                "Automatic reply received"
            } else {
                "Reply received"
            };
            push(
                reply.received_at,
                3,
                what.to_owned(),
                "",
                format!("From {}", reply.from_address),
            );
        }
        for event in sqlx::query!(
            r#"select at, actor, action, details from audit_events
               where tenant_id = $1 and action <> 'debtor.replied'
                 and ((subject_type = 'debtor' and subject_id = $2)
                      or (subject_type = 'debt'
                          and subject_id in (select d.id from debts d where d.tenant_id = $1 and d.debtor_id = $2))
                      or (subject_type = 'claim' and details ->> 'debtor' = $2::text))"#,
            tenant,
            id,
        )
        .fetch_all(&self.pool)
        .await?
        {
            let field = |name: &str| {
                event
                    .details
                    .get(name)
                    .and_then(|value| value.as_str())
                    .unwrap_or_default()
                    .to_owned()
            };
            let (what, details) = match event.action.as_str() {
                "debtor.paid" => ("Marked as paid", String::new()),
                "debtor.promised" => ("Promised to pay", format!("Until {}", field("until"))),
                "debtor.disputed" => ("Marked as disputed", String::new()),
                "debtor.resumed" => ("Reminders resumed", String::new()),
                "debtor.payment_claimed" => ("Said they paid, on their page", String::new()),
                "debtor.opted_out" => ("Asked to stop reminders, on their page", String::new()),
                "debtor.bounced" => ("Reminders stopped after a bounce or complaint", String::new()),
                "debtor.changed" => ("Details changed", String::new()),
                "claim.confirmed" => ("Payment confirmed", String::new()),
                "claim.rejected" => ("Payment claim rejected", String::new()),
                "debt.amount_changed" => (
                    "Amount owed changed",
                    format!("From {} to {}", field("from"), field("to")),
                ),
                other => (other, String::new()),
            };
            push(event.at, 4, what.to_owned(), &event.actor, details);
        }
        log.sort_by_key(|(at, rank, _)| (*at, *rank));
        Ok(log.into_iter().map(|(_, _, entry)| entry).collect())
    }
}

/// Who did something, as the Owner reads it.
fn by_label(actor: &str) -> String {
    match actor {
        "system" => "Balanse".to_owned(),
        "contact" => "The Contact".to_owned(),
        other => other.to_owned(),
    }
}

/// `log` as a CSV file. A cell a spreadsheet would read as a formula gets a
/// leading `'`, since names and replies come from outside.
pub fn to_csv(log: &[Entry]) -> String {
    let when = format_description!("[year]-[month]-[day] [hour]:[minute]");
    let mut out = String::from("When (Manila time),What,By,Details\n");
    for entry in log {
        let at = entry.at.to_offset(MANILA).format(when).unwrap_or_default();
        let cells = [at.as_str(), &entry.what, &entry.by, &entry.details];
        let line: Vec<String> = cells.iter().map(|cell| csv_cell(cell)).collect();
        out.push_str(&line.join(","));
        out.push('\n');
    }
    out
}

fn csv_cell(text: &str) -> String {
    let text = if text.starts_with(['=', '+', '-', '@', '\t', '\r']) {
        format!("'{text}")
    } else {
        text.to_owned()
    };
    if text.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", text.replace('"', "\"\""))
    } else {
        text
    }
}
