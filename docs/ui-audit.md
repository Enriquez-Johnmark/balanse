# UI audit and polish, 2026-11-10

An audit of the Owner's pages and the Debtor's page with the `ui-taste` skill (`reference/audit.md`), then a polish pass (`reference/polish.md`) that kept the existing look. Owner's decision: `docs/decisions.md`, 2026-11-10.

## What was inspected

- **Build:** `main` at `c1dfec3`, on the showcase's demo Business (Reyes Creative Studio, eight Debtors, one bounced address).
- **Browser:** Chromium (Playwright) at 1280×860 and 390×844 (phone, touch emulated).
- **Pages:**
  - the Owner's pages: Debtors, Drafts, Settings, Import, Add a Debtor;
  - a Debtor with reminders on, one who says they've paid, and one whose address bounced;
  - the Debtor's own page.
- **Overflow:** a script measured horizontal overflow on every page at phone width.
- **Keyboard:** Tab order through the navigation.
- **Not checked:** a screen reader, zoom above 100% and real phones.

## Findings and what was done

| # | Priority | Finding | Done |
|---|---|---|---|
| 1 | P2 | On a phone the Debtors list squeezed three columns, so names and next steps broke over three lines | The reminders' state moves under the Debtor's name on a phone, leaving two readable columns (`0093cdd`) |
| 2 | P2 | On a phone a Debtor's invoice numbers broke at the hyphen ("RCS-2026-" / "031"), and once kept whole they pushed the Status column off screen | Invoice numbers stay whole; on a phone the status sits under the invoice (`0093cdd`) |
| 3 | P3 | While a Debtor's payment claim waits, the page offers two "paid" actions: the claim's "Confirm payment" and the outcome card's "They paid" | Left as is: proposed to the owner, since hiding one changes behaviour |

**Checked and fine:**

- Drafts, Settings, Import, Add a Debtor and the Debtor's own page fit a phone with no overflow.
- The bounced-address card and the evidence-log export read clearly.
- Keyboard focus moves through the navigation in order.

## Checks after the fixes

- `cargo fmt --check`, clippy and `cargo test --all-features` (225 tests) passed before the commit.
- `pnpm lint`, `typecheck`, `test` and `build` passed too.
- Both pages were captured again on a phone and inspected.
- The showcase was remade from a fresh demo Business.
