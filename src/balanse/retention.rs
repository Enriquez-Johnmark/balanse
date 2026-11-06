//! Retention: what Balanse deletes, and when. A system-wide worker: it
//! sweeps every Business, naming each row's Business in every change.
//!
//! - Reply text (subject and body) 90 days after the reply arrived; that it
//!   came, and when, remains (F5).

use std::sync::Arc;

use sqlx::PgPool;
use time::Duration;

use crate::engine::clock::Clock;

/// How long a reply's text is kept.
pub const REPLY_TEXT_KEPT: Duration = Duration::days(90);

/// What one sweep deleted.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Swept {
    /// Replies whose text was deleted.
    pub reply_texts: u64,
}

/// Deletes what is past its time.
#[derive(Clone)]
pub struct Retention {
    pool: PgPool,
    clock: Arc<dyn Clock>,
}

impl Retention {
    /// Retention over `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self { pool, clock }
    }

    /// Deletes everything past its time, for every Business.
    pub async fn sweep(&self) -> Result<Swept, sqlx::Error> {
        let now = self.clock.now();
        let reply_texts = sqlx::query!(
            "update replies set subject = '', body_text = null
             where received_at <= $1 and (body_text is not null or subject <> '')",
            now - REPLY_TEXT_KEPT,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        Ok(Swept { reply_texts })
    }
}
