//! Links to the Debtor's page (G1). A link is the record's ID followed by a
//! signature made with a key derived from the master key, so the database
//! holds no usable link, and a guessed or altered one is simply not found.
//! A link works for 60 days from the day it is issued; every reminder
//! carries the Debtor's current one.

use std::sync::Arc;

use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

use super::businesses::BusinessId;
use super::debtors::DebtorId;
use crate::engine::clock::Clock;
use crate::engine::files::MasterKey;
use crate::engine::ids::id_type;
use crate::engine::signed::Signer;

id_type!(
    /// One link to a Debtor's page.
    DebtorLinkId
);

/// How long a link works from the day it is issued (G1).
pub const WORKS_FOR: Duration = Duration::days(60);

/// A link due to expire within this long isn't put in a new reminder; a
/// fresh one is issued instead, so the reminder's link outlives the Draft.
const REISSUE_WITHIN: Duration = Duration::days(14);

/// Why a link doesn't open.
#[derive(Debug, thiserror::Error)]
pub enum LinkError {
    /// No such link, its signature is wrong, or it has expired: all the
    /// same to the person holding it (G1).
    #[error("debtor link not found")]
    NotFound,
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// A genuine, unexpired link, and whose it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenLink {
    /// The link.
    pub id: DebtorLinkId,
    /// The Business it belongs to.
    pub business: BusinessId,
    /// The Debtor it belongs to.
    pub debtor: DebtorId,
}

/// The links of every Business.
#[derive(Clone)]
pub struct DebtorLinks {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    signer: Arc<Signer>,
    base_url: String,
}

impl DebtorLinks {
    /// Links signed with a key derived from `master`, at `base_url`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, master: &MasterKey, base_url: &str) -> Self {
        Self {
            pool,
            clock,
            signer: Arc::new(Signer::new(master, "balanse debtor page v1")),
            base_url: base_url.trim_end_matches('/').to_owned(),
        }
    }

    /// The URL of link `id`.
    pub fn url(&self, id: DebtorLinkId) -> String {
        format!("{}/d/{}", self.base_url, self.signer.token(id.as_uuid()))
    }

    /// The URL to put in `debtor`'s next reminder: its current link, or a
    /// new one if there is none or it expires soon.
    pub async fn issue(&self, business: BusinessId, debtor: DebtorId) -> Result<String, sqlx::Error> {
        let now = self.clock.now();
        let current = sqlx::query_scalar!(
            "select id from debtor_links where tenant_id = $1 and debtor_id = $2 and expires_at > $3
             order by expires_at desc limit 1",
            business.as_uuid(),
            debtor.as_uuid(),
            now + REISSUE_WITHIN,
        )
        .fetch_optional(&self.pool)
        .await?;
        if let Some(id) = current {
            return Ok(self.url(id.into()));
        }
        let id = DebtorLinkId::new();
        sqlx::query!(
            "insert into debtor_links (id, tenant_id, debtor_id, created_at, expires_at) values ($1, $2, $3, $4, $5)",
            id.as_uuid(),
            business.as_uuid(),
            debtor.as_uuid(),
            now,
            now + WORKS_FOR,
        )
        .execute(&self.pool)
        .await?;
        Ok(self.url(id))
    }

    /// The URLs of `debtor`'s links that still work: the only links a
    /// reminder to it may carry (D4).
    pub async fn current_urls(&self, business: BusinessId, debtor: DebtorId) -> Result<Vec<String>, sqlx::Error> {
        let ids = sqlx::query_scalar!(
            "select id from debtor_links where tenant_id = $1 and debtor_id = $2 and expires_at > $3",
            business.as_uuid(),
            debtor.as_uuid(),
            self.clock.now(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(ids.into_iter().map(|id| self.url(id.into())).collect())
    }

    /// The link that `token` (the last part of its URL) names, if it's
    /// genuine and still works. The token itself is the credential, so this
    /// is the one lookup not scoped by a Business.
    pub async fn open(&self, token: &str) -> Result<OpenLink, LinkError> {
        let id = self.signer.verify(token).ok_or(LinkError::NotFound)?;
        let row = sqlx::query!(
            "select l.tenant_id, l.debtor_id from debtor_links l
             join accounts a on a.tenant_id = l.tenant_id and a.closed_at is null
             where l.id = $1 and l.expires_at > $2",
            id,
            self.clock.now(),
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(LinkError::NotFound)?;
        Ok(OpenLink {
            id: id.into(),
            business: row.tenant_id.into(),
            debtor: row.debtor_id.into(),
        })
    }
}
