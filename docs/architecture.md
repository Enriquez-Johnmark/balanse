# Balanse architecture

Status: **Phase 4 complete (2026-10-24).** Everything under "Decided" is owner-approved, with dates. The spec is [product/spec.md](product/spec.md); the engine is [Nudgewright](https://github.com/Enriquez-Johnmark/nudgewright); the patterns reused come from [Kompleto](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/architecture.md).

## Decided

### Shared account machinery moves into Nudgewright (Round 1, 2026-10-24)

- Kompleto's login links and sessions, owner plus up to 5 staff, operator-approved sign-up, and account closing with a grace period move into the engine as an **`accounts`** module: logic and tables only, no web layer. Each product keeps its own pages, routes and email wording ([ADR 0001](adr/0001-accounts-and-sheets-live-in-the-engine.md)).
- The spreadsheet reader moves too, as a **`sheets`** module: CSV reading and the sandboxed Excel reader (a memory- and CPU-limited child process, so a crafted file can't take the server down). Each product keeps its own column checks: Clients in Kompleto, Debts in Balanse.
- Released as **Nudgewright v0.3.0** (a new module and a new migration; no breaking change). Balanse uses it from its first milestone. Kompleto moves onto it later as a separate step and keeps its own copy until then.

### Web app (Round 1, 2026-10-24)

- The owner's pages are a **React app** (Vite, shadcn/ui) served by the Rust binary through a JSON API; the Debtor's page is server-rendered HTML. Every rule that protects an acceptance criterion stays in Rust.
- The app starts as a **copy of Kompleto's web shell** (layout, components, login pages, the API-client pattern and generated TypeScript types). No shared JavaScript package until a third product needs one.

### Email sending (Round 1, 2026-10-24)

- **One Resend account for all products**, each sending from its own domain (Resend Free allows 3). Resend's acceptable-use policy forbids "creating or using an account or multiple accounts with the aim of circumventing any quotas or limits" ([ADR 0002](adr/0002-one-resend-account-shared-by-the-products.md)).
- Each product has a **daily send budget in configuration**. While both run on Resend Free: **Kompleto 60, Balanse 30**, leaving about 10 of the 100 a day for login emails and inbound mail, which count toward Resend's quota.
- **Resend Pro** (US$20/month, no daily cap) comes out of the first customer payment.
- Kompleto's budget is fixed at 90 today; making it configurable is a small Kompleto change made before both products run live.

### Code home and checks (Round 1, 2026-10-24)

- **One Rust crate** with modules, the engine re-exported as `crate::engine`, as in Kompleto.
- The same CI checks as Kompleto: `cargo fmt --check`, clippy with `-D warnings` and the banned system-clock calls, `cargo test --all-features --locked`, `cargo deny`, the offline sqlx check, and for `web/` lint, typecheck, test, build and browser tests.
- `scripts/cloud-session-start.sh` as in Kompleto: Postgres, Mailpit, the pinned Rust, and commit signing. Development database: **`balanse`**.
- Words follow [../GLOSSARY.md](../GLOSSARY.md).
- **Firm isolation becomes Business isolation:** every repository function takes a `BusinessId` (the engine's `TenantId`), with the same narrow exceptions as Kompleto (system-wide workers, operator commands, inbound-email and delivery-event handlers, login and sign-up).

### Build order (Round 1, 2026-10-24)

One milestone at a time, test first, each criterion's test named by its ID (spec, "Acceptance criteria").

| Step | What | Criteria |
|---|---|---|
| Engine | Nudgewright v0.3.0: `accounts` and `sheets` | A1 (in the engine's own tests) |
| M1 | The core loop: Businesses and login, Debtors and Debts by form, the schedule, template Drafts, approval, sending, replies pausing reminders | A1–A3, B1, B3, C1–C7, D1–D4, D6–D7, F1–F3 |
| M2 | The Debtor's page: "I've paid" with proof upload, disputes, opt-out, payment instructions | G1–G6 |
| M3 | AI Drafts and the content guard | D5, E1–E2 |
| M4 | Spreadsheet import, the dashboard, copy and open | B2, H1–H2, H4 |
| M5 | Bounces, the evidence log and its export, leaving, retention | F4–F5, H3, H5 |
| M6 | Deployment, once `balanseapp.com` is bought | — |

The owner's checks of First Circle Invoice, Lista and PayMongo Invoices (brief, kill-checks) come before pilots (moved from before M2 on 2026-10-25, `docs/decisions.md`).

## Open

- Nothing: the readiness gate passed on 2026-10-24.
