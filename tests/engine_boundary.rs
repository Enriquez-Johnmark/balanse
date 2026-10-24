//! Architecture rules that a grep can enforce.
//! - Only the engine's outbox sends email (AGENTS.md "Sending", D1, D6):
//!   calling `Mailer::send` needs the `Mailer` trait in scope, so no Balanse
//!   module may name it.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

use std::fs;
use std::path::{Path, PathBuf};

const SRC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src");

#[test]
fn only_the_outbox_can_send_email() {
    let violations = find(SRC, &[], names_the_mailer_trait);

    assert!(
        violations.is_empty(),
        "only engine::outbox may use the Mailer trait:\n{}",
        violations.join("\n")
    );
}

/// True if `code` contains `Mailer` as a whole word (not `ResendMailer`).
fn names_the_mailer_trait(code: &str) -> bool {
    code.match_indices("Mailer").any(|(at, word)| {
        let before = code[..at].chars().next_back();
        let after = code[at + word.len()..].chars().next();
        !before.is_some_and(|c| c.is_alphanumeric() || c == '_')
            && !after.is_some_and(|c| c.is_alphanumeric() || c == '_')
    })
}

/// Every non-comment line under `dir` (skipping `skip` dirs) for which
/// `bad` holds, as `path:line: text`.
fn find(dir: &str, skip: &[String], bad: impl Fn(&str) -> bool) -> Vec<String> {
    let mut files = Vec::new();
    collect(Path::new(dir), skip, &mut files);
    let mut violations = Vec::new();
    for path in files {
        let source = fs::read_to_string(&path).unwrap();
        for (n, line) in source.lines().enumerate() {
            let code = line.split("//").next().unwrap_or("");
            if bad(code) {
                violations.push(format!("{}:{}: {}", path.display(), n + 1, line.trim()));
            }
        }
    }
    violations
}

fn collect(dir: &Path, skip: &[String], files: &mut Vec<PathBuf>) {
    if skip.iter().any(|s| Path::new(s) == dir) {
        return;
    }
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect(&path, skip, files);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
}
