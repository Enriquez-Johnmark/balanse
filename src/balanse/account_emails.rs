//! The wording of account emails: login links and the welcome for an
//! approved Business. The engine sends each one to the member's own address
//! whatever is set here.

use crate::engine::accounts::LoginLink;
use crate::engine::mail::Email;

/// A login link the person asked for (A1).
pub fn login_email(base_url: &str, link: &LoginLink<'_>) -> Email {
    email(
        link,
        "Your Balanse login link".to_owned(),
        format!("Open this link to log in to {} on Balanse:", link.account_name),
        base_url,
    )
}

/// The first login link, once the operator approved the account.
pub fn welcome_email(base_url: &str, link: &LoginLink<'_>) -> Email {
    email(
        link,
        "Your Balanse account is ready".to_owned(),
        format!(
            "Welcome to Balanse! The account for {} is ready. Open this link to log in:",
            link.account_name
        ),
        base_url,
    )
}

fn email(link: &LoginLink<'_>, subject: String, opening: String, base_url: &str) -> Email {
    Email {
        to_address: link.to_address.to_owned(),
        from_name: "Balanse".to_owned(),
        reply_to: None,
        subject,
        body_text: format!(
            "Hi,\n\n{opening}\n\n{base_url}/login/{}\n\nIt works once, for the next 15 minutes. After that, ask for \
             a new one on Balanse's login page. If you didn't expect this email, ignore it.\n",
            link.token,
        ),
    }
}
