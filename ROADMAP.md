# Roadmap

Where Balanse stands and what comes next, in one page. Dated facts come from `docs/decisions.md`; the milestones from `docs/architecture.md`. Updated 2026-11-05.

## Where we are

**M1, M2 and M3 done, M4 next.** Research, brief, spec (38 acceptance criteria) and architecture are approved; the core loop works end to end and was code-reviewed, with its findings fixed.

| Step | What it delivers | Status |
|---|---|---|
| Engine | Nudgewright v0.3.0 (accounts, sheets), v0.4.0 (held messages don't expire), v0.4.1 (when a session started) | Done |
| M1 | Sign-up and login, Debtors and invoices by form, the schedule, template Drafts, approval, sending once within sending hours, replies pausing reminders; the React app | Done (2026-10-24) |
| M2 | The Debtor's page: signed 60-day link, open invoices and payment instructions, "I've paid" with proof upload, "I dispute this", "Stop emailing me"; the Owner confirms or rejects claims; payment instructions need a fresh login | Done (2026-11-05), code-reviewed |
| M3 | AI-worded Drafts (Claude Haiku) with template fallback, and the content guard | Done (2026-11-05), code-reviewed; AI wording turns on with an API key |
| M4 | Spreadsheet import, the dashboard, "copy and open" for Viber, Messenger, WhatsApp and SMS | |
| M5 | Bounces and complaints, the evidence log and its export, leaving, retention | |
| M6 | Deployment, once `balanseapp.com` is bought | |

## Next

1. M4: spreadsheet import, the dashboard, copy and open.
2. M5, each ending with a code review.
3. **Owner, before pilots:** try First Circle Invoice (waitlist), Lista and PayMongo Invoices (beta) and report what each does: the brief's kill-checks (moved from before M2 on 2026-10-25).
4. **Owner, before M6:** buy `balanseapp.com`.
5. Pilots: B2B service businesses and agencies first, bookkeepers through Kompleto second; ₱0 until then.

## Limits that never move

Balanse never touches money, never contacts anyone but the Debtor, never issues invoices, statements of account or a receivables ledger, and never adds interest or fees; messages are approved one by one and go out only on weekdays 08:00–20:00 Manila time (R1–R18, `docs/research/legal-limits.md`).
