//! B2: a spreadsheet import adds every valid row, lists each invalid row
//! with its reason, and adds nothing new the second time.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use balanse::balanse::import::{ImportError, Importer};
use balanse::engine::sheets::read_csv;
use common::{business, clock};
use sqlx::PgPool;

const FILE: &str = "Debtor,Contact name,Contact email,Invoice number,Invoice date,Amount,Due date,Note
Santos Trading,Lito Santos,lito@santos.test,INV-1,2026-09-30,\"₱12,500.00\",2026-10-30,
Santos Trading,Lito Santos,LITO@santos.test,INV-2,2026-10-05,PHP 3000,2026-11-04,second one
Cruz Studio,Mara Cruz,mara@cruz.test,INV-3,2026-10-01,twelve thousand,2026-10-31,
Reyes Foods,,not-an-email,INV-4,2026-10-01,500,2026-10-31,
";

/// How many Debtors and Debts there are.
async fn counts(pool: &PgPool) -> (i64, i64) {
    let debtors = sqlx::query_scalar("select count(*) from debtors")
        .fetch_one(pool)
        .await
        .unwrap();
    let debts = sqlx::query_scalar("select count(*) from debts")
        .fetch_one(pool)
        .await
        .unwrap();
    (debtors, debts)
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b2_valid_rows_are_added_and_each_invalid_row_says_why(pool: PgPool) {
    let clock = clock();
    let reyes = business(&pool, &clock).await;
    let importer = Importer::new(pool.clone(), clock.clone());

    let report = importer
        .import(reyes, &read_csv(FILE.as_bytes()).unwrap())
        .await
        .unwrap();

    assert_eq!(
        (report.added_debtors, report.added_debts, report.already_there),
        (1, 2, 0)
    );
    let invalid: Vec<_> = report.invalid.iter().map(|row| row.row).collect();
    assert_eq!(invalid, [4, 5]);
    assert!(
        report.invalid[0].problems.iter().any(|p| p.starts_with("Amount")),
        "{:?}",
        report.invalid[0]
    );
    assert!(
        report.invalid[1].problems.iter().any(|p| p.starts_with("Email")),
        "{:?}",
        report.invalid[1]
    );
    assert_eq!(counts(&pool).await, (1, 2));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b2_importing_the_same_file_twice_adds_nothing_new(pool: PgPool) {
    let clock = clock();
    let reyes = business(&pool, &clock).await;
    let importer = Importer::new(pool.clone(), clock.clone());
    let rows = read_csv(FILE.as_bytes()).unwrap();
    importer.import(reyes, &rows).await.unwrap();

    let again = importer.import(reyes, &rows).await.unwrap();

    assert_eq!((again.added_debtors, again.added_debts, again.already_there), (0, 0, 2));
    assert_eq!(counts(&pool).await, (1, 2));
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b2_a_file_without_a_required_column_is_refused(pool: PgPool) {
    let clock = clock();
    let reyes = business(&pool, &clock).await;
    let file = "Debtor,Contact email,Amount\nSantos Trading,lito@santos.test,500\n";

    let result = Importer::new(pool.clone(), clock.clone())
        .import(reyes, &read_csv(file.as_bytes()).unwrap())
        .await;

    assert!(
        matches!(&result, Err(ImportError::MissingColumns(missing)) if missing.contains(&"Invoice number")),
        "{result:?}"
    );
}

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn b2_rows_of_empty_cells_dont_count_toward_the_row_limit(pool: PgPool) {
    let clock = clock();
    let reyes = business(&pool, &clock).await;
    let row = |n: usize| format!("Santos Trading,Lito,lito@santos.test,INV-{n},2026-09-30,500,2026-10-30,\n");
    let mut file = "Debtor,Contact name,Contact email,Invoice number,Invoice date,Amount,Due date,Note\n".to_owned();
    (1..=300).for_each(|n| file.push_str(&row(n)));
    // As Excel saves formatted but empty rows.
    file.push_str(&",,,,,,,\n".repeat(100));
    (301..=450).for_each(|n| file.push_str(&row(n)));

    let report = Importer::new(pool.clone(), clock.clone())
        .import(reyes, &read_csv(file.as_bytes()).unwrap())
        .await
        .unwrap();

    assert_eq!((report.added_debts, report.invalid.len()), (450, 0));
}
