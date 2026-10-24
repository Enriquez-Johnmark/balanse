# Accounts and spreadsheet reading live in the engine

Kompleto built login links, sessions, staff, operator-approved sign-up, account closing and a sandboxed spreadsheet reader inside its own code. Balanse needs all of them. Nudgewright's rule is that something moves into the engine once a second product needs it, so they move: an `accounts` module (logic and tables, no web layer) and a `sheets` module (CSV and the child-process Excel reader), released as v0.3.0. Each product keeps its own pages, routes, email wording and column checks.

The deciding reason is that login and untrusted-file parsing are security-critical. Two copies would drift, and a fix in one would be missed in the other.

## Considered options

- **Copy Kompleto's code into Balanse.** Fastest to start, but leaves two copies of the login and Excel-sandbox code to keep in step. Rejected.
- **A separate shared crate beside the engine.** Keeps the engine smaller, but adds a third repository and release cycle for one owner to run. Rejected for now; worth revisiting if the engine grows too broad.

## Consequences

- Nudgewright gains its first web-adjacent module. It still has no routes or pages: products call `accounts` functions from their own handlers.
- Balanse starts on v0.3.0. Kompleto keeps its own copy until it moves to the engine's in a separate, tested step, so neither product's schedule depends on the other's.
- The `accounts` tables are engine tables: keyed by `TenantId`, never referencing a product's tables, changed only by new migrations.
