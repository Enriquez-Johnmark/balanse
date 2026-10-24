# Research index

Research for Balanse, carried out 2026-10-24 by four parallel research agents plus synthesis, then verified the same day. All facts carry sources inside each file. **Nothing here is customer-validated, and nothing here is an owner-approved decision.** Recommendations are marked as recommendations.

**Method.** A first pass ran with the vendor, regulator and news sites blocked, so it rested on search-result excerpts (SR). A second pass the same day, with web access, re-checked the facts the recommendations rest on against the pages themselves; each file's method note lists what was read directly and what is still SR. Some sites still refuse automated readers (psa.gov.ph, sec.gov.ph, bsp.gov.ph, privacy.gov.ph, ntc.gov.ph), so a few figures stay SR.

## What the research says

- **The pain is real but thinly measured.** Freelancer bills in Congress name late payment as the problem, a House bill cites government payment delays of "months to years", and schools carry arrears. The "SMEs wait 30–90 days" piece is an op-ed by First Circle, a lender, so it counts as a vendor claim. PH listed companies collect in 49 days against an APAC average of 79 (Aon 2026), and 12.7% of PH firms use supplier or customer credit for working capital (World Bank 2023). No PH survey of SME late-payment rates exists ([market-and-pain.md](market-and-pain.md)).
- **"We send reminders" is a free checkbox.** PayMongo Invoices (beta) emails four reminders and stops when paid; Zoho Books' free plan, QuickBooks and Odoo remind automatically; Lista, a PH utang app with 1M+ Google Play downloads, sends free reminders by SMS, Viber or Messenger ([competitors.md](competitors.md)).
- **The gap is the conversation, not the reminder** *(hypothesis)*: no tool found lets a PH business without accounting software send owner-approved, relationship-aware Taglish or English messages that stop the moment the debtor replies. The approve-first tools that exist (Solido, Trove, Upflow) are English, email-only and Xero-based.
- **Most dangerous competitor: First Circle Invoice**, from a PH SME lender: free ("free forever for waitlist members"), reminders by email and SMS with Viber and WhatsApp "coming soon", pitched at awkward follow-ups. It is still waitlist-only and its reminder behaviour is unverified. **Mochi** (PH, live) confirms the price band at ₱1,899/month, but its reminders are automatic and email-only, with no AI drafting, approval or reply reading found.
- **Email fits business debtors, not consumers.** 37% of Filipino internet users used email in July 2026 (Pulse Asia), against 90.6% on Messenger monthly (GWI); business debtors have accounts-payable inboxes. Automated Viber (EUR 115/month per bot) and Messenger (utility messages only to people who wrote to the Page first) don't fit a ₱1,500 product. Semaphore SMS costs ₱0.56 a credit and may carry links, but debtors are trained to distrust links in texts ([channels-and-payment-signals.md](channels-and-payment-signals.md)).
- **Nothing found kills Balanse legally**, provided it stays a follow-up tool: refuse lending customers, contact only the debtor, never compute interest, never touch money, and never issue invoices, receipts or statements of account. Each debt follows an invoice the creditor already issued (BIR requires an invoice, not a statement of account, since 2024), and Balanse never becomes a receivables register: BIR's CAS rules (RMO 9-2021) cover software that records accounts receivable, which would put a joint sworn statement on every customer ([legal-limits.md](legal-limits.md)).
- **The engine already covers an email-only first version.** SMS through the engine would need Loopwright 0.2 ([engine-fit.md](engine-fit.md)).

## Recommendations (for the owner to decide)

1. **First segment: B2B service SMEs and agencies**, whose debtors are businesses with email inboxes: 45,621 formal firms, of which about 8,200 have 10–199 employees (my calculation from DTI's table of PSA 2023 data). Bookkeepers through Kompleto are a close second, starting with chasing their own fees.
2. **Channels: email through the engine, plus "copy and open"** for Viber, Messenger, WhatsApp and the owner's own SMS. No automated SMS, Viber or Messenger in a first version.
3. **Payment signals: the owner marks a debt paid; a debtor's reply stops the loop; a proof-of-payment upload only marks it "payment claimed"** until the owner confirms. No payment-gateway integration in a first version.
4. **Hard limits as product rules** (18 rules, R1–R18, in [legal-limits.md](legal-limits.md) §8): approval of every message, debtor-only contact, quiet hours 08:00–20:00, a frequency cap, stop on reply/payment/dispute, opt-out, no threats or shaming, no interest calculator, no invoices or ledgers, no lending customers.
5. **Kill-check the free tools first**: join First Circle's waitlist, try Lista's reminders and request PayMongo Invoices beta access, before building past the core loop.

## Kill-risks

| # | Risk | Where it's covered | How a pilot would show it |
|---|---|---|---|
| K1 | Free reminders (First Circle, Lista, PayMongo, HitPay) are "good enough"; nobody pays ₱1,500 for approval plus Taglish | competitors.md, market-and-pain.md | Pilots won't convert at ₱1,500 |
| K2 | Corporate clients pay on fixed payables cycles, whatever the reminder says | market-and-pain.md K2 | Days-to-pay unchanged before vs after |
| K3 | First Circle, Mochi or PayMongo adds AI drafts plus approval | competitors.md | Watch their releases; try First Circle and PayMongo Invoices first |
| K4 | Owners won't let any tool message their biggest client | market-and-pain.md K4, H1 | Drafts approved for small debtors only |
| K5 | Offering Balanse to bookkeepers undercuts Kompleto's ₱1,500 offer | market-and-pain.md | Bookkeepers ask for one price for both |
| K6 | BIR treats a reminder tool as a CAS component | legal-limits.md | Not a pilot signal: needs a BIR ruling or a lawyer's view before paid launch |
| K7 | The B2B core is small (about 8,200 firms with 10–199 employees) and unreachable without a network | market-and-pain.md | Outreach replies under the bar set in the brief |

## Files

| Area | File | Status | Main remaining gaps |
|---|---|---|---|
| Market and pain | [market-and-pain.md](market-and-pain.md) | **Verified** 2026-10-24 | PH late-payment rate; how each segment chases today; willingness to pay |
| Competitors and substitutes | [competitors.md](competitors.md) | **Verified** 2026-10-24 | First Circle Invoice launch and reminder behaviour; Zoho's PH price |
| Legal and regulatory limits | [legal-limits.md](legal-limits.md) | **Verified** 2026-10-24; not legal advice | BSP Circular 1160's own list of banned acts; full NPC Circular 2022-04; a BIR ruling on reminder tools vs CAS |
| Channels and payment signals | [channels-and-payment-signals.md](channels-and-payment-signals.md) | **Verified** 2026-10-24 | Which channel PH debtors answer; Messenger utility pricing; whether NTC's 2023 link memo covers A2P SMS |
| Engine fit | [engine-fit.md](engine-fit.md) | **Complete** | Whether logins, sign-up and import move into the engine (architecture decision) |
| Customer evidence | — | **Not started**: comes from outreach and pilots | Everything |

## Builds on

Kompleto's research ([docs/research](https://github.com/Enriquez-Johnmark/kompleto/tree/main/docs/research)), where payment follow-up was scored as candidate C (AR collections on Xero/QBO, 71.8) and set aside because it targeted Xero/QuickBooks users abroad, and where the PH sector sweep found post-sale collections to be "the under-served, low-liability pattern that recurs across sectors".
