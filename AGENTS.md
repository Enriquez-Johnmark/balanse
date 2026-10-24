# AGENTS.md

**Balanse** ("balance") is polite payment follow-up for Philippine small businesses: the owner lists who owes them money, Balanse drafts Taglish or English reminders, the owner approves each one, it is sent once, and follow-ups stop when the customer replies or pays. It is the second product on the engine, [Nudgewright](https://github.com/Enriquez-Johnmark/nudgewright), after [Kompleto](https://github.com/Enriquez-Johnmark/kompleto).

## Phases

Each gate needs the owner's explicit "yes"; an approval for one phase never carries into the next.

| Phase | Status |
|---|---|
| 1. Research | done, verified (`docs/research/README.md`) |
| 2. Brief: segment, channels, limits | done, approved 2026-10-24 (`docs/product/brief.md`) |
| 3. Spec & acceptance criteria | done, approved 2026-10-24 (`docs/product/spec.md`) |
| 4. Architecture (what moves into Nudgewright, what stays here) | **in progress**: Round 1 decided (`docs/architecture.md`) |
| 5. Implementation readiness gate | not started |
| 6+. Build | not started |

**No application code exists or gets written until the readiness gate passes.** Research, docs and throwaway prototypes that answer a specific question are in scope; prototypes live in `prototypes/<question>/` and are deleted once their answer is recorded.

## Owner decisions

Every decision needing the owner's judgement is put as numbered rounds of questions, each with a recommended answer, worded so "yes" accepts it. Look facts up yourself, using sub-agents for research; put only decisions to the owner. A routine "go ahead with the agreed step?" is one line.

Record every approved decision, dated, in `docs/decisions.md`, and update the summary below.

**In force** (full log: `docs/decisions.md`; decisions shared with the other products: Kompleto's `docs/decisions.md`):

- **Balanse is next (2026-10-24):** research first, then brief, spec and readiness gate, before any code.
- **First customers (2026-10-24):** B2B service businesses and agencies; bookkeepers through Kompleto a close second.
- **First version (2026-10-24):** email sent by the engine plus "copy and open" for Viber, Messenger, WhatsApp and SMS; paid status from the owner, replies and proof uploads, with no payment-gateway integration.
- **Product rules (2026-10-24):** the legal limits R1–R18 (`docs/research/legal-limits.md`) are enforced in code; above all, Balanse never touches money, never contacts anyone but the debtor, and never issues invoices, statements of account or a receivables ledger.
- **Brief (2026-10-24):** approved; `docs/product/brief.md` sets the direction for the spec.
- **Spec (2026-10-24):** decided round by round in `docs/product/spec.md`, the single source for spec decisions; complete, with 38 acceptance criteria (A1–H5) approved.
- **Architecture (2026-10-24):** decided in `docs/architecture.md`: accounts and spreadsheet reading move into Nudgewright (v0.3.0); one Resend account shared with Kompleto (budgets 60/30); build order engine, then M1–M6.
- **Name and domain (2026-10-24):** Balanse, at `balanseapp.com`.
- **Licence (2026-10-23):** all rights reserved (source readable, not reusable); only the engine, Nudgewright, is open source (AGPL-3.0-only).
- **Engine:** Nudgewright at a pinned release; an engine change is made, tested and released there, never patched here.
- **Budget:** ₱0 until pilots. No customer data goes to free LLM tiers that train on inputs, and the app never runs on a Claude subscription (API key only).
- **Signed commits (2026-10-24):** commits are signed with the owner's SSH key.
- **Owner constraints:** no network, no interviews or sales calls, no paid ads. Written, asynchronous contact is fine.

## Evidence

Research claims carry a source (URL, publisher, date, accessed date), an evidence type (OD official, PP pricing page, VD vendor docs, VC vendor claim, N news, TP third party, SR search-result excerpt only) and a confidence (H/M/L). Label vendor claims as vendor claims. When evidence is missing, write "not found"; a gap stays a gap. Keep facts, hypotheses, recommendations and owner-approved decisions visibly distinct.

## Workflow

The owner is the only developer: work directly on `main`. No branches, no pull requests.

- **Small commits:** one behaviour or one small doc change each, with imperative messages that describe it. Commit messages carry no AI attribution trailers such as "Co-Authored-By" (owner's choice).
- **Push** to `origin main` after each commit.
- Rules for code (Rust, React, tests, the engine boundary) are written when the build phase begins, starting from Kompleto's `AGENTS.md`.

## Scope

Work only inside the approved phase and scope. When something outside it looks worth doing, raise it as a proposal for the owner.
