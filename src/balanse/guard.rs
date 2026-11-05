//! The content guard (D5, R8): words a reminder must never use, in English,
//! Tagalog and Taglish (`docs/research/legal-limits.md` §4). The Owner sees
//! why a message was refused.
//!
//! Two levels. Model output may contain none of it. The Owner's own words
//! may mention lawful legal action (a case, a lawyer, the barangay), which
//! is the Owner's decision to make (decision of 2026-10-24), but never a
//! threat of arrest or jail, an insult, shaming, impersonation or a request
//! for a one-time code or password.
//!
//! Facts Balanse inserts (names, the invoice list, the footer) are removed
//! before checking, so a Debtor called "Santos Law Office" is no problem.

/// Who wrote the text being checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Writer {
    /// The model: the strictest rules.
    Model,
    /// The Owner, editing or approving: lawful legal action is allowed.
    Owner,
}

/// A group of words never allowed, with the reason shown to the Owner.
struct Group {
    reason: &'static str,
    owner_may_write: bool,
    terms: &'static [&'static str],
}

const GROUPS: &[Group] = &[
    Group {
        reason: "it threatens arrest, jail or the police",
        owner_may_write: false,
        terms: &[
            "ipapakulong",
            "ipakulong",
            "makukulong",
            "kulong",
            "ikukulong",
            "jail",
            "arrest",
            "arrested",
            "aresto",
            "huhulihin",
            "police",
            "pulis",
            "nbi",
            "blotter",
            "blacklist",
            "blacklisted",
            "credit record",
            "criminal case",
            "kasong kriminal",
            "warrant",
        ],
    },
    Group {
        reason: "it mentions legal action, which only you may write",
        owner_may_write: true,
        terms: &[
            "kakasuhan",
            "kasuhan",
            "demanda",
            "idedemanda",
            "court",
            "korte",
            "lawyer",
            "abogado",
            "attorney",
            "legal action",
            "legal notice",
            "final notice",
            "small claims",
            "file a case",
            "magsasampa",
            "barangay",
        ],
    },
    Group {
        reason: "it insults the reader",
        owner_may_write: false,
        terms: &[
            "scammer",
            "you scam",
            "manloloko",
            "magnanakaw",
            "estafa",
            "estafador",
            "walang hiya",
            "walanghiya",
            "bwisit",
            "gago",
            "tanga",
            "bobo",
            "puta",
            "putang",
            "putangina",
            "shit",
            "fuck",
            "idiot",
            "stupid",
            "thief",
            "liar",
            "sinungaling",
            "deadbeat",
        ],
    },
    Group {
        reason: "it threatens to tell or shame them before others",
        owner_may_write: false,
        terms: &[
            "ipopost",
            "ipost",
            "post you",
            "post this on facebook",
            "post it on facebook",
            "sasabihan namin",
            "sabihin namin",
            "pamilya mo",
            "tell your family",
            "contact your family",
            "your employer",
            "your boss",
            "amo mo",
            "boss mo",
            "kapitbahay",
            "your neighbors",
            "your neighbours",
            "your relatives",
            "kamag-anak mo",
            "kamaganak mo",
            "group chat",
            "everyone will know",
        ],
    },
    Group {
        reason: "it poses as a collection agency, law office or government office",
        owner_may_write: false,
        terms: &[
            "collection agency",
            "collections department",
            "law office",
            "law firm",
            "legal department",
            "government office",
            "on behalf of the government",
            "bangko sentral",
            "securities and exchange commission",
            "bureau of internal revenue",
        ],
    },
    Group {
        reason: "it asks for a one-time code or password, as scams do",
        owner_may_write: false,
        terms: &[
            "send your otp",
            "send the otp",
            "send us your otp",
            "give your otp",
            "share your otp",
            "otp code",
            "ipadala ang otp",
            "ipadala po ang otp",
            "ibigay ang otp",
            "ibigay po ang otp",
            "send your pin",
            "give your pin",
            "your password",
            "ipadala ang password",
            "ibigay ang password",
            "passcode",
        ],
    },
];

/// Why `text`, written by `writer`, must not be sent, if it mustn't.
/// `facts` are inserted by Balanse and left out of the check.
pub fn check(text: &str, facts: &[&str], writer: Writer) -> Option<&'static str> {
    // Whole words only: a name inside a blocked word ("Pol" in "police")
    // must not hide it.
    let mut tokens: Vec<Option<String>> = tokens(text).into_iter().map(Some).collect();
    for fact in facts
        .iter()
        .map(|fact| self::tokens(fact))
        .filter(|fact| !fact.is_empty())
    {
        let mut at = 0;
        while at + fact.len() <= tokens.len() {
            if tokens[at..at + fact.len()]
                .iter()
                .zip(&fact)
                .all(|(word, f)| word.as_ref() == Some(f))
            {
                tokens[at..at + fact.len()].iter_mut().for_each(|word| *word = None);
                at += fact.len();
            } else {
                at += 1;
            }
        }
    }
    // A removed fact still separates words, so no term spans it.
    let tokens: Vec<String> = tokens
        .into_iter()
        .map(|word| word.unwrap_or_else(|| "\u{0}".to_owned()))
        .collect();
    GROUPS
        .iter()
        .filter(|group| writer == Writer::Model || !group.owner_may_write)
        .find(|group| group.terms.iter().any(|term| contains(&tokens, &self::tokens(term))))
        .map(|group| group.reason)
}

/// Lower-case words, with hyphens joined ("i-post" is "ipost").
fn tokens(text: &str) -> Vec<String> {
    text.to_lowercase()
        .replace('-', "")
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_owned)
        .collect()
}

/// Whether `needle`'s words appear in `haystack`, together and in order.
fn contains(haystack: &[String], needle: &[String]) -> bool {
    !needle.is_empty() && haystack.windows(needle.len()).any(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d5_threats_insults_shaming_and_impersonation_are_blocked_in_three_languages() {
        for (text, why) in [
            ("Kung hindi ka magbabayad, makukulong ka.", "arrest"),
            ("We will report you to the police.", "arrest"),
            ("Manloloko ka talaga.", "insult"),
            ("You are a scammer.", "insult"),
            ("Ipo-post namin ito sa Facebook.", "shame"),
            ("Sasabihan namin ang pamilya mo.", "shame"),
            ("We'll tell your employer.", "shame"),
            ("This is the Legal Department of Reyes Creative.", "pose"),
            ("Ipadala po ang OTP para ma-verify.", "code"),
        ] {
            let reason = check(text, &[], Writer::Owner);
            assert!(
                reason.is_some_and(|r| r.contains(match why {
                    "arrest" => "arrest",
                    "insult" => "insults",
                    "shame" => "shame",
                    "pose" => "poses",
                    _ => "code",
                })),
                "{text}: {reason:?}"
            );
        }
    }

    #[test]
    fn d5_friendly_and_helpful_words_are_allowed() {
        for text in [
            "Hope your family is well!",
            "You can also message us on Facebook if that's easier.",
            "We never ask for your OTP. Beware of scams pretending to be us.",
            "Our employer ID is on the invoice.",
            "Thank you for the 2nd sec of your time.",
        ] {
            assert_eq!(check(text, &[], Writer::Owner), None, "{text}");
        }
    }

    #[test]
    fn d5_a_name_inside_a_blocked_word_doesnt_hide_it() {
        let text = "Hi Pol, we will report you to the police.";

        assert!(check(text, &["Pol"], Writer::Owner).is_some());
    }

    #[test]
    fn d5_a_name_that_is_a_blocked_word_is_only_a_name() {
        assert_eq!(check("Hi Pin, salamat po!", &["Pin"], Writer::Owner), None);
    }

    #[test]
    fn d5_only_the_owner_may_mention_lawful_legal_action() {
        let text = "If this stays unpaid, we may file a small claims case.";

        assert_eq!(check(text, &[], Writer::Owner), None);
        assert!(check(text, &[], Writer::Model).is_some());
    }

    #[test]
    fn d5_polite_reminders_and_inserted_names_pass() {
        let body = "Hi Lito,\n\nMagandang araw po! Friendly reminder lang po na lumampas na sa due date ang invoice na ito:\n\n\
                    - Invoice INV-7: ₱12,500.00\n\nPakisabi po sa amin kung kailan namin ito maaasahan. Salamat po!\n\n\
                    Santos Law Office\n--\nSent via Balanse. To stop these emails, reply STOP.";

        assert_eq!(check(body, &["Santos Law Office"], Writer::Model), None);
        assert!(
            check(body, &[], Writer::Model).is_some(),
            "without the fact removed, it would trip"
        );
        assert_eq!(
            check("Pinadala po namin ang kopya.", &[], Writer::Model),
            None,
            "not \"pin\""
        );
    }
}
