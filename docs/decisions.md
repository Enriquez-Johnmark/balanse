# Decision log

Every owner-approved decision, newest first, with the date it was made. Decisions that a later one replaced stay here, marked **superseded**, so the reasoning trail survives. `AGENTS.md` lists only the decisions in force.

Decisions made before this repository existed (the product family, the name, licences, the engine) are in [Kompleto's decision log](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/decisions.md), 2026-10-23.

## 2026-10-24

- **Brief approved (Phase 2 gate).** `docs/product/brief.md` is the agreed direction for the spec: who Balanse is for, the first version, what it never does, and the kill-checks. Its open questions are settled in the spec (owner: "yes, approve the Balanse brief").
- **First customers: B2B service businesses and agencies**, whose debtors are companies with email inboxes (45,621 formal firms, about 8,200 with 10–199 employees). Bookkeepers through Kompleto are a close second, starting with chasing their own fees. Landlords, schools, clinics and lenders are not first (owner: "go ahead please" to Round 1).
- **Channels in the first version:** Balanse sends email itself, through the engine. For Viber, Messenger, WhatsApp and SMS it gives the owner a "copy and open" message to send from their own phone. No automated SMS, Viber or Messenger, so no engine change is needed (Round 1).
- **How Balanse knows a debt is paid:** the owner marks it paid; a debtor's reply stops the reminders; a proof-of-payment upload only marks the debt "payment claimed" until the owner confirms, because fake GCash receipts exist. No payment-gateway integration in the first version (Round 1).
- **Legal limits are product rules, enforced in code** (R1–R18 in `docs/research/legal-limits.md`): every message approved; only the debtor is contacted; messages only between 08:00 and 20:00 Manila time; a per-debtor frequency cap and an opt-out; no threats or shaming; no interest calculator; each debt carries the creditor's own invoice number and date; never produce invoices, statements of account or a receivables ledger; never touch money; no lending customers. No template mentions legal action; the owner may write it in themselves (Round 1).
- **Check the free tools before building past the core loop:** the owner joins First Circle Invoice's waitlist, tries Lista's reminders and requests PayMongo Invoices beta access (written sign-ups, no calls), and reports what each actually does (Round 1).
- **Name and domain:** Balanse stays, with `balanseapp.com` as its domain (`balanse.ph` belongs to Balanse Skincare). Sakto is dropped as the backup name: it clashes with GCash's "GLoan Sakto" and about 60 SAKTO marks (owner: "go ahead please" to the names round; `docs/research/product-names.md` in Kompleto).
- **Balanse is next, starting with research.** After Kompleto and the engine split, Balanse (polite payment follow-up) is the next product. It starts the way Kompleto did: research, then a brief, then a spec with acceptance criteria, then a readiness gate, with no product code before the owner approves that gate (owner: "go ahead please" to the recommendation).
