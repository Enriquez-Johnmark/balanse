//! H4: "Copy for SMS" has no link and no phone number; "Open WhatsApp"
//! fills in the text for the Contact's mobile.

use balanse::balanse::copy::{for_sms, whatsapp_link};
use balanse::balanse::schedule::Step;
use balanse::balanse::wording::{DebtLine, Reminder, reminder};
use time::macros::date;

fn body() -> String {
    let wording = reminder(
        &Reminder {
            step: Step::First,
            language: "english",
            contact_name: "Lito Santos",
            business_name: "Reyes Creative",
        },
        &[DebtLine {
            invoice_number: "INV-0042".to_owned(),
            invoice_date: date!(2026 - 09 - 15),
            amount: "₱12,500.00".to_owned(),
            due_date: date!(2026 - 10 - 15),
            disputed: false,
        }],
        "https://balanseapp.com/d/0123abcd",
        None,
    );
    // As if the Owner had edited the Draft.
    wording.body.replace(
        "Reyes Creative\n\n--",
        "Call me on 0917 123 4567 or see reyes.ph/pay.\nReyes Creative\n\n--",
    )
}

#[test]
fn h4_sms_text_has_no_link_and_no_phone_number() {
    let invoice_lines = ["- Invoice INV-0042 (15 Sep 2026): ₱12,500.00, due 15 Oct 2026".to_owned()];

    let sms = for_sms(&body(), &invoice_lines);

    for banned in ["http", "balanseapp.com", "reyes.ph", "0917", "123 4567"] {
        assert!(!sms.contains(banned), "{banned:?} in {sms}");
    }
    assert!(
        sms.contains("- Invoice INV-0042 (15 Sep 2026): ₱12,500.00, due 15 Oct 2026"),
        "{sms}"
    );
    assert!(sms.trim_end().ends_with("Reyes Creative"), "{sms}");
}

#[test]
fn h4_whatsapp_opens_with_the_text_for_the_contacts_mobile() {
    assert_eq!(
        whatsapp_link("0917 123 4567", "Hi Lito, ₱1 & more").as_deref(),
        Some("https://wa.me/639171234567?text=Hi%20Lito%2C%20%E2%82%B11%20%26%20more")
    );
    assert_eq!(
        whatsapp_link("+63 917-123-4567", "x").as_deref(),
        Some("https://wa.me/639171234567?text=x")
    );
    assert_eq!(whatsapp_link("", "x"), None);
}
