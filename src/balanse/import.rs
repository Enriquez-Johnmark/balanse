//! Importing Debts from a CSV or Excel file (B2). Each row is one Debt,
//! with its Debtor and Contact; a Debtor already on the list (the same
//! company and email) is reused. A row whose Debtor already has a Debt
//! with that invoice number is left alone, so importing a file twice adds
//! nothing new. Every invalid row is listed with its reasons; the valid
//! ones are added.
//!
//! The file is untrusted: the engine reads it within size limits, only the
//! columns below are read, and every value goes through the same checks as
//! the forms.

use std::collections::HashMap;
use std::sync::Arc;

use sqlx::PgPool;

use super::businesses::BusinessId;
use super::debtors::{self, DebtorError, DebtorId, DebtorInput, Debtors};
use super::debts::{self, DebtError, DebtInput, Debts};
use crate::engine::clock::Clock;
use crate::engine::sheets::{MAX_ROWS, squash};

/// The columns of Balanse's own template, in order.
pub const COLUMNS: [&str; 10] = [
    "Debtor",
    "Contact name",
    "Contact email",
    "Contact mobile",
    "Language",
    "Invoice number",
    "Invoice date",
    "Amount",
    "Due date",
    "Note",
];

/// Header names, squashed (lowercase letters and digits), that hold each
/// field, best first; the first is the template's.
const DEBTOR: &[&str] = &["debtor", "businessname", "company", "client", "customer"];
const CONTACT: &[&str] = &["contactname", "contact"];
const EMAIL: &[&str] = &["contactemail", "email", "emailaddress"];
const MOBILE: &[&str] = &["contactmobile", "mobile", "mobilenumber", "phone"];
const LANGUAGE: &[&str] = &["language"];
const INVOICE: &[&str] = &["invoicenumber", "invoiceno", "invoice"];
const INVOICE_DATE: &[&str] = &["invoicedate", "date"];
const AMOUNT: &[&str] = &["amount", "amountowed", "amountdue"];
const DUE_DATE: &[&str] = &["duedate", "due"];
const NOTE: &[&str] = &["note", "notes"];

/// Why a file couldn't be imported at all.
#[derive(Debug, thiserror::Error)]
pub enum ImportError {
    /// The file has no rows.
    #[error("the file is empty")]
    Empty,
    /// More than [`MAX_ROWS`] rows.
    #[error("the file has more than {MAX_ROWS} rows; split it into smaller files")]
    TooManyRows,
    /// Required columns are missing, by their template names.
    #[error("the file has no {} column", .0.join(", no "))]
    MissingColumns(Vec<&'static str>),
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// One row that wasn't added, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidRow {
    /// Its row number, as the spreadsheet shows it.
    pub row: usize,
    /// Every problem with it.
    pub problems: Vec<String>,
}

/// What an import did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportReport {
    /// New Debtors.
    pub added_debtors: usize,
    /// New Debts.
    pub added_debts: usize,
    /// Rows whose Debt was already there.
    pub already_there: usize,
    /// Rows not added.
    pub invalid: Vec<InvalidRow>,
}

/// Imports Debts for every Business.
pub struct Importer {
    pool: PgPool,
    debtors: Debtors,
    debts: Debts,
}

impl Importer {
    /// An importer over `pool`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self {
            debtors: Debtors::new(pool.clone(), clock.clone()),
            debts: Debts::new(pool.clone(), clock),
            pool,
        }
    }

    /// Imports `rows` (header first, each with its row number) for `business`.
    pub async fn import(
        &self,
        business: BusinessId,
        rows: &[(usize, Vec<String>)],
    ) -> Result<ImportReport, ImportError> {
        let Some(((_, header), rows)) = rows.split_first() else {
            return Err(ImportError::Empty);
        };
        // Rows of empty cells, as Excel saves formatted ones, don't count.
        let filled = rows.iter().filter(|(_, cells)| !is_blank(cells)).count();
        if filled > MAX_ROWS {
            return Err(ImportError::TooManyRows);
        }
        let columns = Columns::find(header)?;
        let mut report = ImportReport::default();
        for (number, cells) in rows {
            if is_blank(cells) {
                continue;
            }
            let (debtor, debt) = columns.read(cells);
            let mut problems = debtors::problems(&debtor);
            problems.extend(debts::problems(&debt));
            if !problems.is_empty() {
                report.invalid.push(InvalidRow { row: *number, problems });
                continue;
            }
            match self.add(business, &debtor, &debt, &mut report).await {
                Ok(()) => {}
                Err(problems) => report.invalid.push(InvalidRow { row: *number, problems }),
            }
        }
        Ok(report)
    }

    /// Adds one checked row; a database failure is reported on the row.
    async fn add(
        &self,
        business: BusinessId,
        debtor: &DebtorInput,
        debt: &DebtInput,
        report: &mut ImportReport,
    ) -> Result<(), Vec<String>> {
        let failed = |err: &dyn std::fmt::Display| {
            tracing::error!(%err, "importing a row failed");
            vec!["Couldn't be saved; try again".to_owned()]
        };
        let id = match self
            .existing_debtor(business, debtor)
            .await
            .map_err(|err| failed(&err))?
        {
            Some(id) => id,
            None => {
                let id = self.debtors.create(business, debtor).await.map_err(|err| match err {
                    DebtorError::Invalid(problems) => problems,
                    other => failed(&other),
                })?;
                report.added_debtors += 1;
                id
            }
        };
        let exists = sqlx::query_scalar!(
            r#"select exists (select 1 from debts where tenant_id = $1 and debtor_id = $2
                              and lower(invoice_number) = lower($3)) as "exists!""#,
            business.as_uuid(),
            id.as_uuid(),
            debt.invoice_number.trim(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| failed(&err))?;
        if exists {
            report.already_there += 1;
            return Ok(());
        }
        self.debts.create(business, id, debt).await.map_err(|err| match err {
            DebtError::Invalid(problems) => problems,
            other => failed(&other),
        })?;
        report.added_debts += 1;
        Ok(())
    }

    /// The Debtor with the same company name and Contact email, if any.
    async fn existing_debtor(
        &self,
        business: BusinessId,
        debtor: &DebtorInput,
    ) -> Result<Option<DebtorId>, sqlx::Error> {
        let id = sqlx::query_scalar!(
            "select id from debtors where tenant_id = $1 and lower(business_name) = lower($2)
             and lower(contact_email) = lower($3) order by created_at limit 1",
            business.as_uuid(),
            debtor.business_name.trim(),
            debtor.contact_email.trim(),
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(id.map(Into::into))
    }
}

/// Whether every cell of a row is empty.
fn is_blank(cells: &[String]) -> bool {
    cells.iter().all(|cell| cell.trim().is_empty())
}

/// Where each field is in a file, by column index.
struct Columns {
    at: HashMap<&'static str, usize>,
}

impl Columns {
    /// Finds the columns in `header`; the optional ones may be missing.
    fn find(header: &[String]) -> Result<Self, ImportError> {
        let names: Vec<String> = header.iter().map(|name| squash(name)).collect();
        let mut at = HashMap::new();
        let mut missing = Vec::new();
        for (field, aliases, required) in [
            ("Debtor", DEBTOR, true),
            ("Contact name", CONTACT, true),
            ("Contact email", EMAIL, true),
            ("Contact mobile", MOBILE, false),
            ("Language", LANGUAGE, false),
            ("Invoice number", INVOICE, true),
            ("Invoice date", INVOICE_DATE, true),
            ("Amount", AMOUNT, true),
            ("Due date", DUE_DATE, true),
            ("Note", NOTE, false),
        ] {
            match aliases
                .iter()
                .find_map(|alias| names.iter().position(|name| name == alias))
            {
                Some(index) => {
                    at.insert(field, index);
                }
                None if required => missing.push(field),
                None => {}
            }
        }
        if missing.is_empty() {
            Ok(Self { at })
        } else {
            Err(ImportError::MissingColumns(missing))
        }
    }

    /// The text of `field` in `cells`, or empty.
    fn get(&self, cells: &[String], field: &str) -> String {
        self.at
            .get(field)
            .and_then(|index| cells.get(*index))
            .map(|cell| cell.trim().to_owned())
            .unwrap_or_default()
    }

    /// One row as the forms would send it. A blank language is Taglish, as
    /// in the form.
    fn read(&self, cells: &[String]) -> (DebtorInput, DebtInput) {
        let language = match self.get(cells, "Language") {
            blank if blank.is_empty() => "taglish".to_owned(),
            language => language,
        };
        (
            DebtorInput {
                business_name: self.get(cells, "Debtor"),
                contact_name: self.get(cells, "Contact name"),
                contact_email: self.get(cells, "Contact email"),
                contact_mobile: self.get(cells, "Contact mobile"),
                language,
            },
            DebtInput {
                invoice_number: self.get(cells, "Invoice number"),
                invoice_date: self.get(cells, "Invoice date"),
                amount: self.get(cells, "Amount"),
                due_date: self.get(cells, "Due date"),
                note: self.get(cells, "Note"),
            },
        )
    }
}
