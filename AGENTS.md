# AGENTS.md

**Balanse** ("balance") is polite payment follow-up for Philippine small businesses: the owner lists who owes them money, Balanse drafts Taglish or English reminders, the owner approves each one, it is sent once, and follow-ups stop when the customer replies or pays. It is the second product on the engine, [Nudgewright](https://github.com/Enriquez-Johnmark/nudgewright), after [Kompleto](https://github.com/Enriquez-Johnmark/kompleto).

## Phases

Each gate needs the owner's explicit "yes"; an approval for one phase never carries into the next.

| Phase | Status |
|---|---|
| 1. Research | done, verified (`docs/research/README.md`) |
| 2. Brief: segment, channels, limits | done, approved 2026-10-24 (`docs/product/brief.md`) |
| 3. Spec & acceptance criteria | done, approved 2026-10-24 (`docs/product/spec.md`) |
| 4. Architecture (what moves into Nudgewright, what stays here) | done (`docs/architecture.md`, `docs/adr/`, `GLOSSARY.md`) |
| 5. Implementation readiness gate | done, passed 2026-10-24 |
| 6+. Build | **in progress**: Nudgewright v0.3.0 first, then M1–M6 (`docs/architecture.md`) |

**The readiness gate passed on 2026-10-24; code follows the build order in `docs/architecture.md`.** Research, docs and throwaway prototypes that answer a specific question are in scope; prototypes live in `prototypes/<question>/` and are deleted once their answer is recorded.

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

## Rust

- Read the `rust-best-practices` skill (`.claude/skills/rust-best-practices`) before writing or reviewing Rust.
- One crate with modules; add a workspace only when a concrete requirement demands it.
- Agent safety lives in code, not prompts. Approval gates, idempotent sends, the frequency cap and quiet hours are enforced by the application. Model output and inbound content are untrusted data.
- Report test results only from commands actually run.
- Use the words in `GLOSSARY.md` in code, docs and messages (Business, Owner, Debtor, Contact, Debt, Claim, Dispute, Promise, Draft, Reminder…).

## React

- Read the `vercel-react-best-practices`, `vercel-composition-patterns` and `shadcn` skills (`.claude/skills/`) before writing or reviewing React, and follow `docs/frontend.md`. Where they disagree, `docs/frontend.md` wins.
- The React app only shows data and asks the Rust API to act. Every rule that protects an acceptance criterion stays in Rust.
- Third-party skills are read in full before they're added, recorded in `skills-lock.json` and `docs/frontend.md` with source, commit and licence, and may not run unpinned commands.

## Balanse code rules

Each rule protects an acceptance criterion in `docs/product/spec.md` or a product rule (R1–R18) in `docs/research/legal-limits.md`. Where a tool can enforce a rule, the tool does.

- **Sending:** only `engine::outbox` sends messages, and only an approved Draft whose content hash still matches, with an idempotency key recorded before the provider call (D1, D6). A boundary test fails if any other module names the `Mailer` trait.
- **Recipients:** a message goes only to the Debtor's stored Contact, one address, no CC or BCC; the pre-send check enforces it at approval and again before sending (D4, R2–R3).
- **Business isolation:** every repository function takes a `BusinessId` (the engine's `TenantId`); there are no unscoped queries on Business-owned tables (A2). The exceptions: system-wide workers, which still name the row's tenant in every update; operator commands run from the command line; the inbound-email and delivery-event handlers under `/hooks/`, which find the Business from a signed reply code or the provider's message ID; and login and sign-up, before any Business is known.
- **Time:** read time only through the injected `Clock`. Direct system-clock calls are banned with clippy `disallowed-methods`. The schedule, quiet hours and the frequency cap are computed from the Clock in Manila time (C1–C5).
- **Untrusted input:** model output, inbound email, anything typed on the Debtor page, proof files and imported spreadsheets are untrusted. Parse them into strict types, and never echo them into commands or SQL. Model requests carry only the fields listed in E1.
- **Money:** Balanse never touches money: no payment-provider SDKs, merchant accounts, wallets or payouts. Payment instructions are the Owner's own text, shown on the Debtor page (R12, G6).
- **No accounting records:** no total of amounts across Debtors, no aging breakdown, no receivables export, and nothing titled invoice, receipt or statement of account, in any page, API response or file (H2, R11). Amounts are stored and shown exactly as typed; no interest, penalty or late-fee arithmetic (B3, R10).
- **Engine:** Nudgewright, a git dependency pinned to a release and re-exported as `crate::engine`. An engine change is made, tested and released there, then this repo moves to the new release; it is never patched here.
- **Tests:** every acceptance criterion has a test whose name starts with its ID, e.g. `d6_crash_mid_send_never_sends_twice`.
- **Local only in development:** `docker compose` runs Postgres 17 and Mailpit (and ClamAV with `--profile scan`); no real email leaves a dev machine.

## Development

- **Coding happens in Claude Code on the web.** `scripts/cloud-session-start.sh` runs at session start (via `.claude/settings.json`) and brings up Postgres, Mailpit, the pinned Rust and commit signing. On a local machine it does nothing. Both are created in the first build commit.
- Locally: `docker compose up -d`; `cp .env.example .env` once.
- Run tests with `cargo test --all-features`: the D6 crash test needs the `failpoints` feature.
- After changing any SQL, run `cargo sqlx prepare -- --all-targets --all-features` and commit `.sqlx/`; CI compiles offline from it.
- The React app: `pnpm --dir web install`, then `pnpm --dir web build` for the Rust server to serve it, or `pnpm --dir web dev` while working on it. `cargo test` writes the API's TypeScript types into `web/src/api/generated/`; commit them.

## Milestone workflow

The owner is the only developer: work directly on `main`. No branches, no pull requests.

1. Follow the build order in `docs/architecture.md`, one milestone at a time.
2. Test first: write one criterion's test, see it fail, then make it pass.
3. **Small commits.** Each commit is one behaviour (one test plus the code that makes it pass) or one small refactor or doc change, aiming for under ~150 changed lines excluding `.sqlx/`, `Cargo.lock` and vendored files. Messages are imperative and describe the behaviour. Commit messages carry no AI attribution trailers such as "Co-Authored-By" (owner's choice).
4. Before every commit: `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings` and `cargo test --all-features --locked` all pass, and for changes under `web/`, `pnpm --dir web lint`, `typecheck`, `test` and `build` too. If SQL changed, run `cargo sqlx prepare -- --all-targets --all-features` and commit `.sqlx/` with it.
5. Push to `origin main` after each commit. If CI goes red, fixing it is the next commit.
6. At the end of a milestone, run the `code-review` skill, fix findings in small commits, and tell the owner how to try it.

## Scope

Work only inside the approved phase and scope. When something outside it looks worth doing, raise it as a proposal for the owner.
