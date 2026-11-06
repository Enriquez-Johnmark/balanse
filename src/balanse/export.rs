//! Everything a Business has in Balanse, as one JSON file for the Owner to
//! keep when leaving (H5): its settings, its Debtors with their Debts, the
//! messages sent to them, replies still held, messages sent by hand and
//! delivery problems. Amounts are as typed; there are no totals (H2).

use serde_json::{Value, json};
use sqlx::PgPool;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use super::businesses::BusinessId;

fn at(moment: OffsetDateTime) -> String {
    moment.format(&Rfc3339).unwrap_or_default()
}

/// All of `business`'s data.
pub async fn business_data(pool: &PgPool, business: BusinessId) -> Result<Value, sqlx::Error> {
    let tenant = business.as_uuid();
    let account = sqlx::query!(
        "select a.name, b.payment_instructions from accounts a join businesses b on b.tenant_id = a.tenant_id
         where a.tenant_id = $1",
        tenant,
    )
    .fetch_one(pool)
    .await?;
    let mut debtors = Vec::new();
    for debtor in sqlx::query!(
        "select id, business_name, contact_name, contact_email, contact_mobile, language, state, promised_until,
                created_at
         from debtors where tenant_id = $1 order by created_at, id",
        tenant,
    )
    .fetch_all(pool)
    .await?
    {
        let debts: Vec<Value> = sqlx::query!(
            "select invoice_number, invoice_date, amount, due_date, note, status, created_at, closed_at
             from debts where tenant_id = $1 and debtor_id = $2 order by created_at, id",
            tenant,
            debtor.id,
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|debt| {
            json!({
                "invoice_number": debt.invoice_number,
                "invoice_date": debt.invoice_date.to_string(),
                "amount": debt.amount,
                "due_date": debt.due_date.to_string(),
                "note": debt.note,
                "status": debt.status,
                "added_at": at(debt.created_at),
                "closed_at": debt.closed_at.map(at),
            })
        })
        .collect();
        let messages: Vec<Value> = sqlx::query!(
            "select r.step, m.status, m.to_address, m.subject, m.body_text, m.approved_at, m.approved_by, m.sent_at,
                    m.created_at
             from reminders r join outbox_messages m on m.tenant_id = r.tenant_id and m.id = r.message_id
             where r.tenant_id = $1 and r.debtor_id = $2 order by m.created_at, m.id",
            tenant,
            debtor.id,
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|message| {
            json!({
                "step": message.step,
                "status": message.status,
                "to": message.to_address,
                "subject": message.subject,
                "body": message.body_text,
                "drafted_at": at(message.created_at),
                "approved_at": message.approved_at.map(at),
                "approved_by": message.approved_by,
                "sent_at": message.sent_at.map(at),
            })
        })
        .collect();
        let replies: Vec<Value> = sqlx::query!(
            "select from_address, subject, body_text, automatic, received_at
             from replies where tenant_id = $1 and debtor_id = $2 order by received_at, id",
            tenant,
            debtor.id,
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|reply| {
            json!({
                "from": reply.from_address,
                "subject": reply.subject,
                "text": reply.body_text,
                "automatic": reply.automatic,
                "received_at": at(reply.received_at),
            })
        })
        .collect();
        let sent_by_hand: Vec<Value> = sqlx::query!(
            "select sent_at, logged_by from manual_sends where tenant_id = $1 and debtor_id = $2 order by sent_at, id",
            tenant,
            debtor.id,
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|sent| json!({ "sent_at": at(sent.sent_at), "logged_by": sent.logged_by }))
        .collect();
        let bounces: Vec<Value> = sqlx::query!(
            "select address, kind, reason, at from bounces where tenant_id = $1 and debtor_id = $2 order by at, id",
            tenant,
            debtor.id,
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|bounce| json!({ "address": bounce.address, "kind": bounce.kind, "reason": bounce.reason, "at": at(bounce.at) }))
        .collect();
        debtors.push(json!({
            "business_name": debtor.business_name,
            "contact_name": debtor.contact_name,
            "contact_email": debtor.contact_email,
            "contact_mobile": debtor.contact_mobile,
            "language": debtor.language,
            "state": debtor.state,
            "promised_until": debtor.promised_until.map(|date| date.to_string()),
            "added_at": at(debtor.created_at),
            "debts": debts,
            "messages": messages,
            "replies": replies,
            "sent_by_hand": sent_by_hand,
            "delivery_problems": bounces,
        }));
    }
    Ok(json!({
        "business": { "name": account.name, "payment_instructions": account.payment_instructions },
        "debtors": debtors,
    }))
}
