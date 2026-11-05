# Balanse

*Filipino for "balance"*

**Polite payment follow-up for Philippine small businesses.**

A business lists who owes it money. Balanse drafts polite reminders in Taglish or English, the owner approves each one exactly as it will be sent, it goes out once, and follow-ups stop as soon as the customer replies or pays. It never touches the money and never threatens or shames anyone.

Balanse runs on [Nudgewright](https://github.com/Enriquez-Johnmark/nudgewright), the engine behind [Kompleto](https://github.com/Enriquez-Johnmark/kompleto).

## Status

**Build.** The research, [brief](docs/product/brief.md), [spec](docs/product/spec.md) and [architecture](docs/architecture.md) are approved. The core loop (M1) works end to end: sign-up and login, Debtors and their invoices, scheduled Drafts, approval, sending once within sending hours, and replies pausing reminders. M2–M6 follow ([build order](docs/architecture.md)). Decisions are in [docs/decisions.md](docs/decisions.md).

## Try it locally

You need Docker, Rust and pnpm.

```sh
docker compose up -d                 # Postgres and Mailpit
cp .env.example .env
pnpm --dir web install && pnpm --dir web build
cargo run -- serve                   # http://127.0.0.1:3000
```

1. Open http://127.0.0.1:3000/signup and request an account.
2. In another terminal, approve it: `cargo run -- signups`, then `cargo run -- approve-signup <id>`. The welcome email with your login link is in Mailpit, http://localhost:8025.
3. Add a Debtor, then an invoice whose due date has passed.
4. Draft its reminder as of a weekday: `cargo run -- dev-schedule --date YYYY-MM-DD` (a date after the due date).
5. Approve it on the Drafts page. It is sent on a weekday between 08:00 and 20:00 Manila time; outside those hours it waits. Sent mail appears in Mailpit.
6. Optional: put an `ANTHROPIC_API_KEY` in `.env` (an API key, never a Claude subscription) and run step 4 again: the model writes the wording, Balanse inserts the facts, and anything it gets wrong falls back to the template.
7. Open the link in the reminder: that is the Debtor's page. Press "I've paid" with a photo, then confirm or reject the claim on the Debtor's page in the app. Set your payment instructions under Settings (it asks for a login from the last 10 minutes).
8. On the Debtors page, choose "Import a spreadsheet", download the template, fill in a few rows (make one amount "lots" to see a row refused with its reason) and import it. Debtors needing your action are listed first, then active ones oldest overdue first.
9. On a Draft, open "Copy and open": copy it for Viber or Messenger, open WhatsApp (needs the Contact's mobile number), or copy the SMS version, which has no link and no phone number. On the Debtor's page, "I sent one myself" records a message you sent that way, so the next email waits.

## Licence

All rights reserved ([LICENSE](LICENSE)). The source is published to be read, not reused: copying, running or hosting it needs the author's written permission. The engine, [Nudgewright](https://github.com/Enriquez-Johnmark/nudgewright), is open source under AGPL-3.0-only.
