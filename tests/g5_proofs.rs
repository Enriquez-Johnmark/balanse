//! G5: proof files are checked by type, size and count, virus-scanned
//! failing closed, kept encrypted and deleted on time.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use std::sync::Arc;

use balanse::balanse::businesses::BusinessId;
use balanse::balanse::contact_actions::{ClaimId, ContactActions};
use balanse::balanse::debtors::{DebtorInput, Debtors};
use balanse::balanse::proofs::{KEPT_AFTER_SETTLED_DAYS, KEPT_AT_MOST_DAYS, MAX_FILE_BYTES, ProofError, Proofs};
use balanse::engine::clock::{Clock, FixedClock};
use balanse::engine::scan::Clamd;
use common::clamd::{Behaviour, FakeClamd, nobody_listening};
use common::{business, clock};
use sqlx::PgPool;
use time::Duration;

const JPEG: &[u8] = b"\xFF\xD8\xFF\xE0 a photo of a GCash receipt";
const INFECTED_PDF: &[u8] = b"%PDF-1.4 EICAR";

struct World {
    clock: Arc<FixedClock>,
    business: BusinessId,
    claim: ClaimId,
    dir: tempfile::TempDir,
}

/// Santos Trading's claim that it paid Reyes Creative.
async fn world(pool: &PgPool) -> World {
    let clock = clock();
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
    let claim = ContactActions::new(pool.clone(), clock.clone(), common::links(pool, &clock))
        .claim_paid(business, debtor)
        .await
        .unwrap();
    World {
        clock,
        business,
        claim,
        dir: tempfile::tempdir().unwrap(),
    }
}

impl World {
    fn proofs(&self, pool: &PgPool) -> Proofs {
        Proofs::new(
            pool.clone(),
            self.clock.clone(),
            common::files(pool, &self.clock, self.dir.path()),
        )
    }
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g5_a_photo_is_kept_encrypted_and_read_back(pool: PgPool) {
    let world = world(&pool).await;
    let proofs = world.proofs(&pool);

    let id = proofs.add(world.business, world.claim, JPEG).await.unwrap();

    let listed = proofs.for_claim(world.business, world.claim).await.unwrap();
    assert_eq!(listed.len(), 1);
    let download = proofs.download(world.business, id, "Santos Trading").await.unwrap();
    assert_eq!(download.bytes, JPEG);
    assert!(
        download.name.starts_with("proof-Santos-Trading-") && download.name.ends_with(".jpg"),
        "{}",
        download.name
    );
    let on_disk: Vec<_> = walkdir(world.dir.path());
    assert!(
        on_disk.iter().all(|bytes| !bytes.windows(5).any(|w| w == b"GCash")),
        "stored in the clear"
    );
}

fn walkdir(dir: &std::path::Path) -> Vec<Vec<u8>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files.extend(walkdir(&path));
        } else {
            files.push(std::fs::read(path).unwrap());
        }
    }
    files
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g5_wrong_types_oversized_files_and_a_fourth_file_are_refused(pool: PgPool) {
    let world = world(&pool).await;
    let proofs = world.proofs(&pool);

    let text = proofs.add(world.business, world.claim, b"just text").await;
    let big = proofs
        .add(world.business, world.claim, &[JPEG, &vec![0; MAX_FILE_BYTES]].concat())
        .await;
    for _ in 0..3 {
        proofs.add(world.business, world.claim, JPEG).await.unwrap();
    }
    let fourth = proofs.add(world.business, world.claim, JPEG).await;

    assert!(matches!(text, Err(ProofError::NotAllowedType)), "{text:?}");
    assert!(matches!(big, Err(ProofError::TooBig)), "{big:?}");
    assert!(matches!(fourth, Err(ProofError::ClaimFull)), "{fourth:?}");
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g5_an_infected_or_unscannable_file_is_refused(pool: PgPool) {
    let world = world(&pool).await;
    let scanning = FakeClamd::start(Behaviour::Scans).await;
    let scanned = world.proofs(&pool).scanned_by(Clamd::new(scanning.addr()));
    let unscanned = world.proofs(&pool).scanned_by(Clamd::new(nobody_listening().await));

    let infected = scanned.add(world.business, world.claim, INFECTED_PDF).await;
    let clean = scanned.add(world.business, world.claim, JPEG).await;
    let cant_scan = unscanned.add(world.business, world.claim, JPEG).await;

    assert!(matches!(infected, Err(ProofError::Infected)), "{infected:?}");
    assert!(clean.is_ok(), "{clean:?}");
    assert!(matches!(cant_scan, Err(ProofError::NotScanned(_))), "{cant_scan:?}");
    assert_eq!(scanned.for_claim(world.business, world.claim).await.unwrap().len(), 1);
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g5_files_go_30_days_after_the_claim_is_settled_and_90_at_most(pool: PgPool) {
    let world = world(&pool).await;
    let proofs = world.proofs(&pool);
    let files = common::files(&pool, &world.clock, world.dir.path());
    let id = proofs.add(world.business, world.claim, JPEG).await.unwrap();
    let unsettled = delete_by(&pool, id).await;

    proofs.settle(world.business, world.claim).await.unwrap();
    let settled = delete_by(&pool, id).await;
    world.clock.advance(Duration::days(KEPT_AFTER_SETTLED_DAYS));
    files.sweep().await.unwrap();

    assert_eq!(
        unsettled,
        world.clock.now_plus(-Duration::days(KEPT_AFTER_SETTLED_DAYS)) + Duration::days(KEPT_AT_MOST_DAYS)
    );
    assert_eq!(settled, world.clock.now());
    assert!(
        proofs.for_claim(world.business, world.claim).await.unwrap().is_empty(),
        "deleted"
    );
    assert!(matches!(
        proofs.download(world.business, id, "Santos Trading").await,
        Err(ProofError::NotFound)
    ));
}

async fn delete_by(pool: &PgPool, id: balanse::engine::ids::FileId) -> time::OffsetDateTime {
    sqlx::query_scalar("select delete_by from stored_files where id = $1")
        .bind(id.as_uuid())
        .fetch_one(pool)
        .await
        .unwrap()
}
