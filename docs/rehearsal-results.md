# Rehearsal and measurements

Run on 2026-11-09 in a Claude Code cloud session, as Kompleto's was: the **release build**, Postgres 17, a **real ClamAV** with live signatures, **Claude Sonnet wording the reminders** through the owner's API key, and email caught by Mailpit. Balanse's deployment (M6) waits for its domain, so there are no deploy or backup scripts to rehearse yet.

**Result: everything that ran passed.**

## Build

| Measure | Result |
|---|---|
| Release build (`cargo build --release`, 4 cores, cold) | 2 min 40 s |
| Program size | 24 MB |
| Web app build | 9 s; 1.1 MB |
| `/health` | `ok`: app, database and ClamAV all answer |

## Tests

| Suite | Result |
|---|---|
| Rust tests, one per acceptance criterion and more (`cargo test --all-features`) | 224 passed |
| Web app tests (`pnpm --dir web test`) | 13 passed |
| Browser journeys against the release build (`pnpm --dir web e2e`, `BALANSE_BIN` set to it) | 12 of 12 passed: login, Debtors by form and spreadsheet, a batch reviewed in full before approval, a promise pausing reminders, a lending business refused, a Contact saying they paid with proof (scanned by ClamAV) and the Owner confirming it, copy for SMS, no totals or accounting documents, changing a Contact, downloading all data, and closing the account |
| The showcase (`pnpm --dir web showcase`) against the release build | screenshots and walkthrough made; all 7 reminders worded by Claude Sonnet (US$0.019 in all) |

## The AI model (the Taglish test)

Haiku and Sonnet each wrote 10 made-up reminders, covering all four steps and every overdue range, with Balanse's exact request. A blind judge (Claude Opus, told to judge as a small Manila business chasing its clients) compared each pair twice with the order swapped: **Sonnet 9, Haiku 1, no ties**, every verdict the same in both orders. Haiku often greeted twice ("Hi Lito," then "Hi Lito, magandang araw po!") and overused "po". All 20 drafts kept Balanse's rules (no digits, amounts, addresses, links or overdue lengths). Cost per reminder: Haiku US$0.0001, Sonnet US$0.003. The owner chose Sonnet (`docs/decisions.md`).

## Speed

The release build, logged in as the showcase Business (16 Debtors after the browser tests), 400 requests per page, 10 at a time. No request failed.

| Page | Requests a second | Median | 95th percentile |
|---|---|---|---|
| `/health` | 1,694 | 5.7 ms | 9.2 ms |
| Debtors | 1,581 | 5.4 ms | 11.7 ms |
| One Debtor | 1,060 | 8.7 ms | 16.0 ms |
| A Debtor's evidence log | 1,004 | 9.3 ms | 15.4 ms |
| Drafts | 2,566 | 3.0 ms | 8.4 ms |
| The Debtor's own page (`/d/…`) | 1,478 | 6.2 ms | 11.4 ms |
| The app's page (`/`) | 266 | 44.0 ms | 48.0 ms |

Memory: Balanse 23 MB. ClamAV needed this sandbox's proxy certificate to fetch its signatures, which a real server doesn't.

## Not covered yet

Real email through Resend, replies by email, the public address, starting at boot and backups all wait for M6 and the domain `balanseapp.com`.
