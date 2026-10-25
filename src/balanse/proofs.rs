//! Proof of payment (G5): the photos or PDFs a Contact attaches to a claim.
//!
//! - Only photos (JPEG, PNG, HEIC/HEIF, WebP) and PDFs up to 10 MB,
//!   recognised by their contents, at most 3 per claim.
//! - Each file is scanned by ClamAV before it is kept, failing closed: a
//!   file that can't be scanned is refused.
//! - Kept encrypted by the engine; deleted 30 days after the Owner settles
//!   the claim, and never kept more than 90 days.
//! - Downloaded only as attachments under a name Balanse makes, never shown
//!   in the browser or named by the Contact.

use std::sync::Arc;

use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

use super::businesses::BusinessId;
use super::contact_actions::ClaimId;
use crate::engine::clock::Clock;
use crate::engine::files::{FileError, Files};
use crate::engine::ids::FileId;
use crate::engine::scan::{Clamd, ScanError, Verdict};

/// The largest file accepted, in bytes.
pub const MAX_FILE_BYTES: usize = 10 * 1024 * 1024;
/// How many files one claim may carry.
pub const MAX_FILES_PER_CLAIM: i64 = 3;
/// How long a file is kept at the most, from its upload.
pub const KEPT_AT_MOST_DAYS: i64 = 90;
/// How long a file is kept after its claim is settled.
pub const KEPT_AFTER_SETTLED_DAYS: i64 = 30;

/// The `ftyp` brands of HEIC/HEIF photos.
const HEIF_BRANDS: [&[u8]; 8] = [b"heic", b"heix", b"hevc", b"hevx", b"heim", b"heis", b"mif1", b"msf1"];

/// The kinds of file accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    /// A JPEG photo.
    Jpeg,
    /// A PNG image.
    Png,
    /// An iPhone photo (HEIC/HEIF).
    Heic,
    /// A WebP image.
    Webp,
    /// A PDF.
    Pdf,
}

impl FileKind {
    /// The kind `bytes` start like, if it's one we accept.
    pub fn sniff(bytes: &[u8]) -> Option<Self> {
        let starts = |prefix: &[u8]| bytes.starts_with(prefix);
        if starts(&[0xFF, 0xD8, 0xFF]) {
            Some(Self::Jpeg)
        } else if starts(b"\x89PNG\r\n\x1a\n") {
            Some(Self::Png)
        } else if starts(b"%PDF-") {
            Some(Self::Pdf)
        } else if starts(b"RIFF") && bytes.get(8..12) == Some(b"WEBP") {
            Some(Self::Webp)
        } else if bytes.get(4..8) == Some(b"ftyp".as_slice())
            && bytes.get(8..12).is_some_and(|brand| HEIF_BRANDS.contains(&brand))
        {
            Some(Self::Heic)
        } else {
            None
        }
    }

    /// The stored name, which is also the media type.
    pub fn media_type(self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Heic => "image/heic",
            Self::Webp => "image/webp",
            Self::Pdf => "application/pdf",
        }
    }

    /// The file name extension, for downloads.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Heic => "heic",
            Self::Webp => "webp",
            Self::Pdf => "pdf",
        }
    }

    /// Reads a stored media type.
    pub fn parse(media_type: &str) -> Option<Self> {
        [Self::Jpeg, Self::Png, Self::Heic, Self::Webp, Self::Pdf]
            .into_iter()
            .find(|kind| kind.media_type() == media_type)
    }
}

/// Why a file wasn't kept.
#[derive(Debug, thiserror::Error)]
pub enum ProofError {
    /// Not a photo or a PDF.
    #[error("only photos (JPEG, PNG, HEIC, WebP) and PDFs are accepted")]
    NotAllowedType,
    /// Over 10 MB.
    #[error("the file is larger than 10 MB")]
    TooBig,
    /// Nothing was sent.
    #[error("no file was sent")]
    NoFile,
    /// The claim already has 3 files.
    #[error("a claim can carry at most 3 files")]
    ClaimFull,
    /// ClamAV found a virus.
    #[error("the file failed the virus scan")]
    Infected,
    /// ClamAV couldn't check it, so it was refused.
    #[error("the file couldn't be scanned for viruses")]
    NotScanned(#[source] ScanError),
    /// No such claim or file in this Business.
    #[error("not found")]
    NotFound,
    /// Storing or reading the file failed.
    #[error(transparent)]
    File(#[from] FileError),
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// One proof file on a claim.
#[derive(Debug, Clone)]
pub struct Proof {
    /// The stored file's ID.
    pub id: FileId,
    /// Its kind.
    pub kind: FileKind,
    /// Its size in bytes.
    pub size_bytes: i32,
    /// When it arrived.
    pub uploaded_at: OffsetDateTime,
}

/// A file to download.
#[derive(Debug)]
pub struct Download {
    /// A name made by Balanse, never the Contact's own.
    pub name: String,
    /// Its kind.
    pub kind: FileKind,
    /// The decrypted contents.
    pub bytes: Vec<u8>,
}

/// Proof files, stored encrypted.
#[derive(Clone)]
pub struct Proofs {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    files: Files,
    scanner: Option<Clamd>,
}

impl Proofs {
    /// Proofs kept in `files`, not scanned for viruses.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, files: Files) -> Self {
        Self {
            pool,
            clock,
            files,
            scanner: None,
        }
    }

    /// The same Proofs, each file scanned by `clamd` before it is kept.
    #[must_use]
    pub fn scanned_by(mut self, clamd: Clamd) -> Self {
        self.scanner = Some(clamd);
        self
    }

    /// Checks `bytes` without keeping them: the type, the size and the
    /// virus scan. The page runs this before a claim is recorded, so a
    /// refused file never leaves a claim behind.
    pub async fn check(&self, business: BusinessId, bytes: &[u8]) -> Result<FileKind, ProofError> {
        if bytes.is_empty() {
            return Err(ProofError::NoFile);
        }
        if bytes.len() > MAX_FILE_BYTES {
            return Err(ProofError::TooBig);
        }
        let kind = FileKind::sniff(bytes).ok_or(ProofError::NotAllowedType)?;
        if let Some(scanner) = &self.scanner {
            match scanner.scan(bytes).await {
                Ok(Verdict::Clean) => {}
                Ok(Verdict::Infected(virus)) => {
                    tracing::warn!(%business, %virus, "refused an infected proof file");
                    return Err(ProofError::Infected);
                }
                Err(err) => {
                    tracing::error!(%business, %err, "a proof file couldn't be scanned, so it was refused");
                    return Err(ProofError::NotScanned(err));
                }
            }
        }
        Ok(kind)
    }

    /// Keeps `bytes` on `claim`, after [`Proofs::check`] passes them.
    pub async fn add(&self, business: BusinessId, claim: ClaimId, bytes: &[u8]) -> Result<FileId, ProofError> {
        let kind = self.check(business, bytes).await?;
        let size = i32::try_from(bytes.len()).map_err(|_| ProofError::TooBig)?;
        let mut tx = self.pool.begin().await?;
        // One upload at a time per claim, so two can't both squeeze under the limit.
        sqlx::query!(
            "select id from claims where id = $1 and tenant_id = $2 for update",
            claim.as_uuid(),
            business.as_uuid(),
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(ProofError::NotFound)?;
        let count = sqlx::query_scalar!(
            r#"select count(*) as "count!" from claim_files where claim_id = $1 and tenant_id = $2"#,
            claim.as_uuid(),
            business.as_uuid(),
        )
        .fetch_one(&mut *tx)
        .await?;
        if count >= MAX_FILES_PER_CLAIM {
            return Err(ProofError::ClaimFull);
        }
        let now = self.clock.now();
        let id = self
            .files
            .save(business, bytes, now + Duration::days(KEPT_AT_MOST_DAYS))
            .await?;
        let recorded = sqlx::query!(
            "insert into claim_files (id, tenant_id, claim_id, media_type, size_bytes, uploaded_at)
             values ($1, $2, $3, $4, $5, $6)",
            id.as_uuid(),
            business.as_uuid(),
            claim.as_uuid(),
            kind.media_type(),
            size,
            now,
        )
        .execute(&mut *tx)
        .await;
        match recorded {
            Ok(_) => tx.commit().await?,
            Err(err) => {
                // A file nobody can see goes now, not in 90 days.
                if let Err(discard) = self.files.discard(business, id).await {
                    tracing::error!(file = %id, %discard, "discarding an unrecorded proof file failed");
                }
                return Err(err.into());
            }
        }
        Ok(id)
    }

    /// `claim`'s files, oldest first.
    pub async fn for_claim(&self, business: BusinessId, claim: ClaimId) -> Result<Vec<Proof>, ProofError> {
        let rows = sqlx::query!(
            "select f.id, f.media_type, f.size_bytes, f.uploaded_at from claim_files f
             join stored_files s on s.id = f.id and s.deleted_at is null
             where f.tenant_id = $1 and f.claim_id = $2
             order by f.uploaded_at, f.id",
            business.as_uuid(),
            claim.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .filter_map(|row| {
                Some(Proof {
                    id: row.id.into(),
                    kind: FileKind::parse(&row.media_type)?,
                    size_bytes: row.size_bytes,
                    uploaded_at: row.uploaded_at,
                })
            })
            .collect())
    }

    /// File `id`, decrypted, named after `debtor_name` and its number on the
    /// claim.
    pub async fn download(&self, business: BusinessId, id: FileId, debtor_name: &str) -> Result<Download, ProofError> {
        let row = sqlx::query!(
            "select media_type, claim_id, uploaded_at from claim_files where id = $1 and tenant_id = $2",
            id.as_uuid(),
            business.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(ProofError::NotFound)?;
        let kind = FileKind::parse(&row.media_type).ok_or(ProofError::NotFound)?;
        let bytes = match self.files.read(business, id).await {
            Err(FileError::NotFound) => return Err(ProofError::NotFound),
            other => other?,
        };
        let date = row
            .uploaded_at
            .format(&time::format_description::well_known::Iso8601::DATE)
            .unwrap_or_default();
        let name: String = debtor_name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect();
        Ok(Download {
            name: format!(
                "proof-{name}-{date}-{}.{}",
                &id.as_uuid().simple().to_string()[..6],
                kind.extension()
            ),
            kind,
            bytes,
        })
    }

    /// Brings every file of `claim` forward to 30 days from now (G5), once
    /// the Owner settled it. A deadline only ever moves earlier.
    pub async fn settle(&self, business: BusinessId, claim: ClaimId) -> Result<(), ProofError> {
        let at = self.clock.now() + Duration::days(KEPT_AFTER_SETTLED_DAYS);
        for proof in self.for_claim(business, claim).await? {
            self.files.delete_no_later_than(business, proof.id, at).await?;
        }
        Ok(())
    }
}
