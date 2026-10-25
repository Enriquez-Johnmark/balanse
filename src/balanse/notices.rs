//! Notices Balanse sends the Owner itself: a Debtor replied, claimed to
//! have paid, disputed, or asked to stop. Each is a plain email to the
//! Owner's address, drafted and approved by Balanse (D1 allows it: the
//! Owner is the recipient, not the Debtor), and never copies a Contact's
//! words.

use sqlx::{PgPool, Postgres, Transaction};

use super::businesses::BusinessId;
use super::debtors::DebtorId;
use crate::engine::ids::MessageId;
use crate::engine::mail::Email;
use crate::engine::outbox::{Outbox, OutboxError};

/// The approver recorded for notices Balanse sends the Owner itself.
pub const NOTICE_APPROVER: &str = "system:notice";

/// Who a notice is about.
pub struct About {
    /// The Contact's name.
    pub contact_name: String,
    /// The Debtor's company name.
    pub debtor_name: String,
    /// The Owner's address.
    pub owner_email: String,
}

/// The Contact, Debtor and Owner a notice about `debtor` concerns.
pub async fn about(pool: &PgPool, business: BusinessId, debtor: DebtorId) -> Result<Option<About>, sqlx::Error> {
    let row = sqlx::query!(
        "select d.contact_name, d.business_name as debtor_name, m.email as owner_email
         from debtors d
         join accounts a on a.tenant_id = d.tenant_id and a.closed_at is null
         join account_members m on m.tenant_id = d.tenant_id and m.role = 'owner'
         where d.tenant_id = $1 and d.id = $2",
        business.as_uuid(),
        debtor.as_uuid(),
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|row| About {
        contact_name: row.contact_name,
        debtor_name: row.debtor_name,
        owner_email: row.owner_email,
    }))
}

/// A notice's wording.
pub struct Notice<'a> {
    /// The subject line.
    pub subject: &'a str,
    /// What happened, a sentence ending with a full stop.
    pub what: &'a str,
    /// What follows.
    pub then: &'a str,
}

/// Drafts `notice` to the Owner in `tx`, with a link to `debtor`'s page in
/// the app at `base_url`. The Draft still needs approving with
/// [`approve_notice`] once `tx` commits.
pub async fn draft_notice(
    outbox: &Outbox,
    tx: &mut Transaction<'_, Postgres>,
    (business, debtor): (BusinessId, DebtorId),
    about: &About,
    base_url: &str,
    notice: &Notice<'_>,
) -> Result<MessageId, OutboxError> {
    let email = Email {
        to_address: about.owner_email.clone(),
        from_name: "Balanse".to_owned(),
        reply_to: None,
        subject: notice.subject.to_owned(),
        body_text: format!(
            "Hi,\n\n{what} See it in Balanse:\n\n{base_url}/debtors/{id}\n\n{then}\n",
            what = notice.what,
            then = notice.then,
            id = debtor.as_uuid(),
        ),
    };
    outbox.create_draft_in(tx, business, &email, None).await
}

/// Approves a notice drafted with [`draft_notice`], once its transaction
/// committed. Safe to repeat: an already approved notice is left alone.
pub async fn approve_notice(
    outbox: &Outbox,
    checks: &super::presend::BalanseChecks,
    business: BusinessId,
    notice: MessageId,
) -> Result<(), OutboxError> {
    let message = outbox.get(business, notice).await?;
    if message.status != crate::engine::outbox::MessageStatus::Draft {
        return Ok(());
    }
    match outbox
        .approve(business, notice, NOTICE_APPROVER, &message.content_hash(), checks)
        .await
    {
        Ok(()) | Err(OutboxError::WrongStatus(_)) => Ok(()),
        Err(err) => Err(err),
    }
}
