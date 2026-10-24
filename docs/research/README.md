# Research index

Research for Balanse, carried out 2026-10-24 by four parallel research agents plus synthesis. All facts carry sources inside each file. **Nothing here is customer-validated, and nothing here is an owner-approved decision.** Recommendations are marked as recommendations.

**Method limit (read first).** The build environment's network policy blocked every vendor, regulator and news site, so almost every new external fact rests on a search-result excerpt (evidence type SR), and confidence is capped at M. Facts carried over from Kompleto's research keep their own ratings. A verification pass with those sites reachable would raise the key facts to H; until then, treat prices and legal readings as indicative.

## What the research says

- **The pain is real but thinly measured.** PH SMEs routinely give larger clients 30–90-day terms, freelancer bills name late payment as the problem, and schools carry arrears, but no PH survey of late-payment rates exists ([market-and-pain.md](market-and-pain.md)).
- **"We send reminders" is a free checkbox.** PayMongo Invoices, Xendit links, Zoho Books' free plan, QuickBooks and Odoo all remind automatically; First Circle advertises free unlimited reminders ([competitors.md](competitors.md)).
- **The gap is the conversation, not the reminder** *(hypothesis)*: no tool found lets a PH business without accounting software send owner-approved, relationship-aware Taglish or English messages that stop the moment the debtor replies. The approve-first tools that exist (Solido, Trove, Upflow) are English, email-only and Xero-based.
- **Most dangerous competitor: Mochi**, a PH accounts-receivable startup (Makati, VC-backed) with invoices, payment links and automated reminders at a reported ₱1,899/month (price: low confidence). AI drafts plus approval would be a feature for them, not a rebuild.
- **Email fits business debtors, not consumers.** Only about 20% of Filipino internet users read email (Pulse Asia 2021), against ~91% on Messenger; but business debtors have accounts-payable inboxes. Automated Viber, Messenger and WhatsApp cost more than a ₱1,500 product can carry; SMS can't carry links ([channels-and-payment-signals.md](channels-and-payment-signals.md)).
- **Nothing found kills Balanse legally**, provided it stays a follow-up tool: refuse lending customers, contact only the debtor, never compute interest, never issue invoices, receipts or statements of account, never become a receivables ledger (BIR's CAS rules), and never touch money ([legal-limits.md](legal-limits.md)).
- **The engine already covers an email-only first version.** SMS through the engine would need Loopwright 0.2 ([engine-fit.md](engine-fit.md)).

## Recommendations (for the owner to decide)

1. **First segment: B2B service SMEs and agencies** (about 45,600 formal firms, my calculation from PSA sectors), whose debtors are businesses with email inboxes. Bookkeepers through Kompleto second, starting with chasing their own fees.
2. **Channels: email through the engine, plus "copy and open"** for Viber, Messenger, WhatsApp and the owner's own SMS. No automated SMS, Viber or Messenger in a first version.
3. **Payment signals: the owner marks a debt paid; a debtor's reply stops the loop; a proof-of-payment upload only marks it "payment claimed"** until the owner confirms. No payment-gateway integration in a first version.
4. **Hard limits as product rules** (18 rules, R1–R18, in [legal-limits.md](legal-limits.md) §8): approval of every message, debtor-only contact, quiet hours 08:00–20:00, a frequency cap, stop on reply/payment/dispute, opt-out, no threats or shaming, no interest calculator, no invoices or ledgers, no lending customers.
5. **Kill-check Mochi first**: see what it actually does and costs before building past the core loop.

## Kill-risks

| # | Risk | Where it's covered | How a pilot would show it |
|---|---|---|---|
| K1 | Free reminders (First Circle, PayMongo, HitPay) are "good enough"; nobody pays ₱1,500 for approval plus Taglish | competitors.md §5, market-and-pain.md | Pilots won't convert at ₱1,500 |
| K2 | Corporate clients pay on fixed payables cycles, whatever the reminder says | market-and-pain.md K2 | Days-to-pay unchanged before vs after |
| K3 | Mochi (or PayMongo) adds AI drafts plus approval | competitors.md §6 | Watch their releases; check Mochi first |
| K4 | Owners won't let any tool message their biggest client | market-and-pain.md K4, H1 | Drafts approved for small debtors only |
| K5 | Offering Balanse to bookkeepers undercuts Kompleto's ₱1,500 offer | market-and-pain.md | Bookkeepers ask for one price for both |
| K6 | BIR treats a reminder tool as a CAS component | legal-limits.md §5 | Not a pilot signal: needs a BIR ruling or a lawyer's view before paid launch |

## Files

| Area | File | Status | Main remaining gaps |
|---|---|---|---|
| Market and pain | [market-and-pain.md](market-and-pain.md) | **Complete** (SR-capped) | PH late-payment rate; how each segment chases today; willingness to pay |
| Competitors and substitutes | [competitors.md](competitors.md) | **Complete** (SR-capped) | Mochi's real price, users, channels and whether it reads replies; First Circle Invoice status |
| Legal and regulatory limits | [legal-limits.md](legal-limits.md) | **Complete** (SR-capped); not legal advice | Exact text of SEC MC 18 and BSP Circular 1160; email as extrajudicial demand (RA 8792); BIR view on reminder tools vs CAS |
| Channels and payment signals | [channels-and-payment-signals.md](channels-and-payment-signals.md) | **Complete** (SR-capped) | Which channel PH debtors answer; Messenger utility pricing; SMS sender-ID link rules |
| Engine fit | [engine-fit.md](engine-fit.md) | **Complete** | Whether logins, sign-up and import move into the engine (architecture decision) |
| Customer evidence | — | **Not started**: comes from outreach and pilots | Everything |

## Builds on

Kompleto's research ([docs/research](https://github.com/Enriquez-Johnmark/kompleto/tree/main/docs/research)), where payment follow-up was scored as candidate C (AR collections on Xero/QBO, 71.8) and set aside because it targeted Xero/QuickBooks users abroad, and where the PH sector sweep found post-sale collections to be "the under-served, low-liability pattern that recurs across sectors".
