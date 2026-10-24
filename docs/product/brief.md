# Balanse brief

Status: **draft, 2026-10-24, awaiting the owner's "yes" (Phase 2 gate).** It turns the research ([../research/README.md](../research/README.md)) and the Round 1 decisions ([../decisions.md](../decisions.md)) into one page. Owner-approved decisions are marked **(decided)**; everything else is a recommendation or an open question for the spec.

## The product in one paragraph

A Philippine service business lists the invoices its clients haven't paid. Balanse drafts a polite reminder for each one, in Taglish or English, worded for the relationship. The owner reads it, edits it if they want, and approves exactly what will be sent. Balanse emails it once, follows up on a schedule the owner controls, and stops the moment the client replies, pays or disputes. For clients the owner talks to on Viber, Messenger or WhatsApp, Balanse prepares the same message to send from the owner's own phone. Balanse never touches the money and never threatens or shames anyone.

## Who it's for

- **First: B2B service businesses and agencies** (decided): marketing and creative agencies, IT and consulting firms, admin and support services, small professional practices. Their clients are companies with accounts-payable inboxes, which email reaches. The realistic core is about 8,200 firms with 10–199 employees (DTI's table of PSA 2023 data; my calculation), plus micro firms.
- **Second: bookkeepers through Kompleto** (decided as a close second), starting with chasing their own unpaid fees.
- **Not first** (decided): landlords, schools, clinics, distributors. **Never:** lenders, financing companies, online lending apps, pawnshops or informal lenders (rule R16).

## The problem

Owners give clients 30–90 days and then have to ask for the money, which is awkward with a client they want to keep. Reminders today are typed by hand in Viber or email, or sent automatically by a billing tool in one fixed wording. The research found no PH survey of how often this happens, so the size of the pain is a hypothesis the pilots must measure ([market-and-pain.md](../research/market-and-pain.md)).

## Why Balanse rather than a free reminder

Free tools already send reminders: First Circle Invoice (waitlist), PayMongo Invoices (beta), Lista, Zoho Books, HitPay. Balanse sells something else (hypothesis, to be proven in pilots):

1. **The words:** a polite, relationship-aware Taglish or English message for each debt, not the same template on day +3.
2. **The owner's approval of every message**, so nothing goes to a valued client that the owner didn't read.
3. **Stopping on reply**, not only on payment: once the client answers, Balanse steps back and the owner talks.
4. **A dated record** of every approved message, delivery and reply, which also helps if the debt ever goes to small claims.

## First version

- **Debts in:** typed in, or imported from a spreadsheet, each with the client's contact, amount, due date and **the owner's own invoice number and date** (decided, rule R11). Import from Xero/QuickBooks exports can follow Kompleto's import.
- **Messages:** drafted by the model or a template, checked by the content guard (R8), approved by the owner, sent by the engine exactly once (decided).
- **Channels:** email sent by Balanse; "copy and open" for Viber, Messenger, WhatsApp and SMS from the owner's phone (decided). No automated SMS, Viber or Messenger.
- **Follow-ups:** a schedule per debt, inside quiet hours (08:00–20:00 Manila) and a per-debtor frequency cap (decided, R4–R5), stopping on reply, payment, dispute or opt-out (R6–R7).
- **Paid status:** set by the owner; a debtor's reply stops reminders; a proof-of-payment upload marks the debt "payment claimed" until the owner confirms (decided).
- **A private page for the debtor** (signed link): "I've paid" with optional proof, "I dispute this", and the owner's own payment details or link.
- **Records:** per-debt log of approved text, sends, delivery events and replies, exportable by the owner (R17).

## What it never does (decided)

Never touches money or holds a merchant account; never contacts anyone but the debtor (no family, employer, group chats, CC or posting); never computes interest or penalties; never issues or titles anything as an invoice, receipt or statement of account; never keeps a receivables ledger, aging report or BIR-format export; never mentions legal action in a template (the owner may write it themselves). The full list with sources is R1–R18 in [legal-limits.md](../research/legal-limits.md).

## Kill-checks before building past the core loop

| Check | Who | Kill or rethink if |
|---|---|---|
| First Circle Invoice: join the waitlist; see what its reminders say, when, and whether a person approves them (decided) | Owner | It already sends owner-approved, relationship-aware messages for free |
| Lista: try its reminders (decided) | Owner | Same as above, for B2B debts |
| PayMongo Invoices: request beta access (decided) | Owner | It adds per-message approval and editable wording |
| Free Taglish reminder-draft page (no login, drafts only), shared in written outreach (recommendation) | Build after the core loop | Under 30 uses a week after 6 weeks, or B2B users under 25% |
| Pilot measures: overdue items never chased before; drafts softened before approval; days-to-pay before vs after (recommendation) | Pilots | Days-to-pay unchanged across pilots |

## Open questions for the spec

1. **Price and go-to-market.** Recommendation: the same path as Kompleto (free written outreach, 3–5 free 4-week pilots, then ₱1,500/month via GCash), which Mochi's ₱1,899 supports. Not yet decided for Balanse.
2. **Frequency cap numbers** (R5): proposed at most one message per 3 days and two per week per debtor, and a stop after N unanswered reminders until the owner acts.
3. **The model and Taglish quality:** reuse Kompleto's model choice, or run Balanse's own blind Taglish test.
4. **Bookkeepers:** offer "chase your own fees" inside Kompleto, or only through Balanse; and how to avoid undercutting Kompleto's price.
5. **What Balanse shares with Kompleto:** logins, staff, sign-up, import and the React shell live in Kompleto today ([engine-fit.md](../research/engine-fit.md)); moving them into the engine is an architecture decision.
6. **A lawyer's or BIR's view** on reminder tools versus CAS components before paid launch (kill-risk K6).
