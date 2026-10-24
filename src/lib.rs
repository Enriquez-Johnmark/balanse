//! Balanse: polite payment follow-up for Philippine small businesses. A
//! Business lists the Debts its clients haven't paid; Balanse drafts a
//! reminder for each Debtor, the Owner approves exactly what goes out, the
//! engine sends it once, and follow-ups stop when the Debtor replies, pays
//! or disputes.

use std::borrow::Cow;
use std::sync::LazyLock;

use sqlx::migrate::Migrator;

pub mod balanse;

/// The engine, Nudgewright, pinned to a release.
pub use nudgewright as engine;

/// The engine's migrations, then Balanse's, run as one set.
///
/// They share sqlx's migrations table, so two separate migrators would each
/// refuse the other's entries. sqlx marks `Migrator`'s fields as exempt from
/// semver; re-check this after upgrading sqlx.
pub static MIGRATOR: LazyLock<Migrator> = LazyLock::new(|| {
    static PRODUCT: Migrator = sqlx::migrate!("./migrations");
    let mut migrations: Vec<_> = engine::MIGRATOR.migrations.iter().cloned().collect();
    migrations.extend(PRODUCT.migrations.iter().cloned());
    migrations.sort_by_key(|migration| migration.version);
    Migrator {
        migrations: Cow::Owned(migrations),
        ..Migrator::DEFAULT
    }
});
