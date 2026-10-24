# Balanse on Loopwright: what the engine covers and what's missing

**Note 2026-10-24:** Loopwright was renamed **Nudgewright** (v0.2.0) after a trademark check, with behaviour unchanged; this note keeps the name it was written under.

Status: **research note, 2026-10-24. Nothing here is an owner-approved decision.** It compares what Balanse needs with what [Loopwright v0.1.0](https://github.com/Enriquez-Johnmark/loopwright) provides, from reading its README and source on 2026-10-24. Engine changes follow Loopwright's rule: anything only one product needs stays in that product until a second product needs it.

## Key conclusions

1. **The core loop is already built.** Draft → approval by content hash → exactly-once email → replies and bounces stop the loop → audit. Balanse needs no new engine work for an email-only first version.
2. **SMS would be the first engine change.** The outbox and `Mailer` only carry `Email`. Sending SMS through the engine, with the same approval and exactly-once guarantees, needs a channel abstraction in the outbox. That is a breaking change, so Loopwright 0.2. "Copy and open" (the owner sends from their own phone, as Kompleto's "copy for Viber" does) needs no engine change.
3. **Payment-provider webhooks stay in Balanse for now.** Verifying PayMongo/HitPay "paid" events is new, but no second product needs it yet. If one does (Kotasyon deposits, for example), the signature check moves into the engine's `webhooks` module.
4. **Logins, staff, approved sign-up and the Xero/QuickBooks import live in Kompleto's code, and Balanse needs them too.** That makes them the first real candidates to move into the engine, or into a shared crate. It's an architecture decision for Balanse's Phase 7, not a research question.

## What Balanse can use as it is

| Balanse need | Loopwright module | Notes |
|---|---|---|
| Owner approves each reminder exactly as shown | `outbox` (approval by content hash) | Same guarantee as Kompleto |
| Each reminder sent once, even across a crash | `outbox::Sender`, `mail` (Resend, Mailpit) | Email only |
| Stop following up when the debtor replies | `inbound` (reply parsing, auto-reply detection) | The product decides what a reply means |
| Stop when an address bounces | `webhooks` (signed delivery events) | |
| Debtor uploads proof of payment | `files` (per-file encryption, deletion deadlines), `scan` (ClamAV, fails closed) | A screenshot is only a claim: the owner confirms |
| A private page for the debtor (proof upload, "I've paid", dispute) | `signed` (signed links) | |
| Polite Taglish/English drafts | `llm`, `cost_log` | The model never sees more than the wording needs |
| Reminder schedules and follow-up steps | `jobs` (Postgres queue) | Schedules themselves are product code |
| Evidence of every send and reply | `audit` (append-only) | Useful to the creditor as a dated paper trail (see [legal-limits.md](legal-limits.md)) |
| Quiet hours, frequency cap, "never contact anyone but the debtor" | `outbox::PreSendCheck` (product rules run at approval and again at sending) | Rules are product code; the hook exists |
| Daily send budget under Resend's free cap | `outbox::Sender` daily budget | Resend Free is 100 emails a day for all customers together ([channels-and-payment-signals.md](channels-and-payment-signals.md)) |
| Closing an account erases its data | `tenants` | |
| Testable time (quiet hours, due dates) | `clock` | |

## What's missing

| Need | Where it belongs | When |
|---|---|---|
| SMS through the engine | Loopwright 0.2 (channel abstraction in `outbox`) | Only if the owner wants automated SMS in a first version |
| PayMongo/HitPay webhook verification | Balanse, later the engine's `webhooks` if a second product needs it | Deferred: no payment integration is recommended for v1 |
| Debts, debtors, amounts, schedules, templates | Balanse | Product code by Loopwright's rules |
| Logins, staff, approved sign-up | Today in Kompleto; candidate to share | Architecture decision |
| Xero/QuickBooks/spreadsheet import | Today in Kompleto (M11); candidate to share | Architecture decision |
| React app shell and API types | Today in Kompleto | Architecture decision |

## Sources

- I1. [Loopwright README](https://github.com/Enriquez-Johnmark/loopwright/blob/main/README.md) and `src/` (`outbox`, `mail/mod.rs`, `webhooks.rs`), read 2026-10-24. Internal; H.
- I2. [Loopwright AGENTS.md](https://github.com/Enriquez-Johnmark/loopwright/blob/main/AGENTS.md), "What belongs here", read 2026-10-24. Internal; H.
- I3. [Kompleto spec](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/product/p1-spec.md) ("copy for Viber", SMS without links) and [Kompleto AGENTS.md](https://github.com/Enriquez-Johnmark/kompleto/blob/main/AGENTS.md) (M9 staff logins, M10 approved sign-up, M11 import), read 2026-10-24. Internal; H.
