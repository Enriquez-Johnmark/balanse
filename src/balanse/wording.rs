//! The fixed wording of reminders, in Taglish or English. The model may
//! later write the opening and closing (M3); everything factual — the
//! invoices, amounts, dates and the Business's name — is always inserted
//! here, so nothing can garble or invent it (D4).

use time::Date;
use time::macros::format_description;

use super::schedule::Step;

/// One open Debt as a reminder lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebtLine {
    /// The Business's invoice number.
    pub invoice_number: String,
    /// The invoice's date.
    pub invoice_date: Date,
    /// The amount exactly as typed.
    pub amount: String,
    /// When it was due.
    pub due_date: Date,
}

/// A reminder's subject and body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wording {
    /// The subject line.
    pub subject: String,
    /// The plain-text body.
    pub body: String,
}

/// What a reminder is about.
#[derive(Debug, Clone, Copy)]
pub struct Reminder<'a> {
    /// Which step of the cycle.
    pub step: Step,
    /// `taglish` or `english`.
    pub language: &'a str,
    /// The Contact's name, for the greeting.
    pub contact_name: &'a str,
    /// The Business's name, which signs it.
    pub business_name: &'a str,
}

/// The reminder's wording, listing every Debt in `debts` (C2) and linking
/// to the Debtor's page at `page_url` (G1).
pub fn reminder(reminder: &Reminder<'_>, debts: &[DebtLine], page_url: &str) -> Wording {
    let taglish = reminder.language == "taglish";
    let invoices = match debts {
        [one] => format!("invoice {}", one.invoice_number),
        many => format!("{} invoices", many.len()),
    };
    let business = reminder.business_name;
    let subject = match (taglish, reminder.step) {
        (true, Step::HeadsUp) => format!("Paalala po: malapit nang mag-due ang {invoices} mula sa {business}"),
        (true, _) => format!("Paalala po: {invoices} mula sa {business}"),
        (false, Step::HeadsUp) => format!("Coming up: {invoices} from {business}"),
        (false, _) => format!("Reminder: {invoices} from {business}"),
    };
    let (opening, closing) = lines(taglish, reminder.step);
    let page = if taglish {
        "Makikita po dito ang mga detalye at kung paano magbayad:"
    } else {
        "You can see the details and how to pay here:"
    };
    let body = format!(
        "{greeting} {name},\n\n{opening}\n\n{list}\n\n{page} {page_url}\n\n{closing}\n\n{business}\n\n{footer}\n",
        greeting = "Hi",
        name = reminder.contact_name,
        list = debt_block(debts),
        footer = footer(business),
    );
    Wording { subject, body }
}

/// The list of Debts exactly as every reminder shows it; the pre-send check
/// requires a message to contain it unchanged (D4).
pub fn debt_block(debts: &[DebtLine]) -> String {
    let format = format_description!("[day padding:none] [month repr:short] [year]");
    debts
        .iter()
        .map(|debt| {
            format!(
                "- Invoice {} ({}): {}, due {}",
                debt.invoice_number,
                debt.invoice_date.format(format).unwrap_or_default(),
                debt.amount,
                debt.due_date.format(format).unwrap_or_default(),
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The footer every reminder carries (D7, R9, R15).
pub fn footer(business: &str) -> String {
    format!(
        "--\nSent via Balanse on behalf of {business}. {business} is responsible for your details in this \
         email, and Balanse handles them on its behalf. To stop these emails, reply STOP."
    )
}

fn lines(taglish: bool, step: Step) -> (&'static str, &'static str) {
    match (taglish, step) {
        (true, Step::HeadsUp) => (
            "Magandang araw po! Isang paalala lang po na malapit na ang due date ng invoice na ito:",
            "Kung naayos na po ninyo, salamat po at pakibalewala na lang ang mensaheng ito. May tanong po? I-reply \
             lang po sa email na ito.",
        ),
        (true, Step::First) => (
            "Magandang araw po! Friendly reminder lang po na lumampas na sa due date ang invoice na ito:",
            "Kung nabayaran na po ninyo, salamat po at pakibalewala na lang ang mensaheng ito. May tanong po? \
             I-reply lang po sa email na ito.",
        ),
        (true, Step::Second) => (
            "Magandang araw po! Follow-up lang po sa invoice na ito na hindi pa po namin natatanggap ang bayad:",
            "Kung may problema po o kailangan ninyo ng kopya ng invoice, i-reply lang po sa email na ito. Salamat po!",
        ),
        (true, Step::Third) => (
            "Magandang araw po. Hindi pa po namin natatanggap ang bayad sa invoice na ito:",
            "Pakisabi po sa amin kung kailan namin ito maaasahan, o kung may problema po, para maayos natin. \
             I-reply lang po sa email na ito. Salamat po.",
        ),
        (false, Step::HeadsUp) => (
            "I hope you're well. Just a quick note that the invoice below is due soon:",
            "If it's already taken care of, thank you, and please ignore this message. Any questions? Just reply \
             to this email.",
        ),
        (false, Step::First) => (
            "I hope you're well. This is a friendly reminder that the invoice below is now past its due date:",
            "If you've already paid, thank you, and please ignore this message. Any questions? Just reply to this \
             email.",
        ),
        (false, Step::Second) => (
            "I'm following up on the invoice below, which we haven't received payment for yet:",
            "If something's holding it up, or you need a copy of the invoice, just reply to this email. Thank you!",
        ),
        (false, Step::Third) => (
            "We still haven't received payment for the invoice below:",
            "Could you let us know when we can expect it, or tell us if there's a problem we can sort out? Just \
             reply to this email. Thank you.",
        ),
    }
}
