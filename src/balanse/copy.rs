//! "Copy and open" (H4): the texts the Owner pastes into Viber, Messenger,
//! WhatsApp or SMS. Producing them changes nothing: no approval, no send.
//!
//! An SMS may carry no link of any kind and no phone number (R13). The
//! Owner may have edited the message, so the SMS text is filtered strictly:
//! the invoice lines Balanse itself wrote are kept as they are, and every
//! other line with anything like a link or a phone number is dropped, as is
//! the email footer.

/// The SMS version of `body`: lines equal to one in `invoice_lines` are
/// kept; the footer and any other line with a link or a phone number are
/// dropped.
pub fn for_sms(body: &str, invoice_lines: &[String]) -> String {
    let mut kept: Vec<&str> = Vec::new();
    for line in body.lines() {
        if line.trim_end() == "--" {
            break;
        }
        let ours = invoice_lines.iter().any(|ours| ours == line);
        if ours || (!has_link(line) && !has_phone_number(line)) {
            kept.push(line);
        }
    }
    // Dropped lines may leave blank ones behind; keep single gaps only.
    let mut text = String::new();
    let mut blank = false;
    for line in kept {
        if line.trim().is_empty() {
            blank = !text.is_empty();
            continue;
        }
        if blank {
            text.push('\n');
            blank = false;
        }
        text.push_str(line);
        text.push('\n');
    }
    text.trim_end().to_owned()
}

/// A wa.me link that opens WhatsApp on `mobile` with `text` filled in, or
/// `None` without a usable mobile number.
pub fn whatsapp_link(mobile: &str, text: &str) -> Option<String> {
    let digits: String = mobile.chars().filter(char::is_ascii_digit).collect();
    let international = match digits.as_bytes() {
        [b'0', b'9', ..] if digits.len() == 11 => format!("63{}", &digits[1..]),
        [b'9', ..] if digits.len() == 10 => format!("63{digits}"),
        _ if (10..=15).contains(&digits.len()) => digits,
        _ => return None,
    };
    Some(format!("https://wa.me/{international}?text={}", percent_encoded(text)))
}

/// Whether any word of `line` looks like a web address, bare domains
/// included ("reyes.ph/pay").
fn has_link(line: &str) -> bool {
    line.split_whitespace().any(|word| {
        let word = word
            .trim_matches(|c: char| "()[]<>\"'.,;:!?".contains(c))
            .to_lowercase();
        if word.contains("://") || word.starts_with("www.") {
            return true;
        }
        let host = word.split('/').next().unwrap_or_default();
        let labels: Vec<&str> = host.split('.').collect();
        labels.len() >= 2
            && labels
                .iter()
                .all(|label| !label.is_empty() && label.chars().all(|c| c.is_alphanumeric() || c == '-'))
            && labels
                .last()
                .is_some_and(|tld| tld.chars().count() >= 2 && tld.chars().all(char::is_alphabetic))
    })
}

/// Whether `line` holds seven or more digits in one run, allowing single
/// spaces, dashes or dots between groups ("0917 123 4567", "+63-917…").
fn has_phone_number(line: &str) -> bool {
    let chars: Vec<char> = line.chars().collect();
    let mut digits = 0;
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_digit() {
            digits += 1;
            if digits >= 7 {
                return true;
            }
        } else {
            let joins = matches!(c, ' ' | '-' | '.')
                && i > 0
                && chars[i - 1].is_ascii_digit()
                && chars.get(i + 1).is_some_and(char::is_ascii_digit);
            if !joins {
                digits = 0;
            }
        }
    }
    false
}

fn percent_encoded(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.~".contains(&byte) {
            out.push(char::from(byte));
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}
