# Balanse spec: first version

Status: **Phase 3 in progress.** Built up round by round; everything under "Decided" is owner-approved, with dates. Background: [brief.md](brief.md) (approved 2026-10-24) and [../research/README.md](../research/README.md). The acceptance criteria come in a later round.

## Decided

### Debts in (Round 1, 2026-10-24)

- A **Debtor** is a client company with one billing **Contact**: a name, an email address, and optionally a mobile number for "copy and open".
- A Debtor has one or more **Debts**. Each Debt holds the owner's **invoice number**, **invoice date**, **amount exactly as typed** (pesos), **due date** and an optional private note. Balanse never computes interest, penalties or totals beyond adding up the open Debts it lists (rule R10), and never issues or titles anything as an invoice or statement of account (R11).
- Debts are added with a form, or imported from a CSV or Excel file with an error shown for each bad row (Kompleto's import pattern).

### One reminder per Debtor (Round 1, 2026-10-24)

- A Debtor with several overdue Debts gets **one message listing all of them**, not one message per invoice.
- The frequency cap counts per Debtor, across all its Debts and channels.

### Reminder schedule (Round 1, 2026-10-24)

- **Reminders:** a friendly reminder the day after the due date, then reminders 7 and 14 days after it.
- **Pause:** after three unanswered reminders, Balanse stops and marks the Debtor **"needs your call"**.
- **Heads-up:** an optional courtesy note 3 days before the due date, off by default.
- **Cap:** never more than **1 message per 3 days** and **2 per 7 days** to a Debtor. The owner can't raise it.
- **Timing:** Drafts appear at 07:00 Manila time. Messages go out only on **weekdays, 08:00–20:00 Manila time** (R4); a step that falls on a weekend moves to Monday.

### Approval (Round 1, 2026-10-24)

- **Every message waits for the owner's approval** (R1). There is no automatic sending in the first version.
- A **Drafts page** lists every waiting Draft with its recipient and full text. The owner approves one Draft, or a batch after reviewing each in full; each approval binds the exact text shown. Editing a Draft voids any earlier approval.
- An unapproved Draft **expires after 3 days**; that step is skipped and logged.
- **Pre-send checks** run at approval (the approval is refused, with the reason) and again just before sending (the message is blocked). They require that:
  - the recipient is the Debtor's stored Contact, and only that one address (R2–R3);
  - the only link is the Debtor's own private page;
  - the invoice list and amounts are the ones Balanse inserted, matching the stored Debts (R10);
  - the content guard finds no threat, insult, shaming or impersonation (R8);
  - quiet hours, the frequency cap and any opt-out are respected (R4, R5, R7).

### Sender identity and replies (Round 1, 2026-10-24)

- Email comes from the owner's business name on Balanse's domain, e.g. `"Reyes Creative" <reminders@balanseapp.com>`, with a signed reply address. A footer says "sent via Balanse" and carries the privacy line (R9, R15). Balanse never sends from the owner's own mailbox.
- A **human reply** pauses that Debtor's reminders and notifies the owner, without copying the reply's text into the notification email.
- The owner then marks the Debtor:
  - **paid**;
  - **promised to pay** by a date, so reminders resume the day after it;
  - **disputed**, which stops reminders (R6);
  - or **resume**.
- Automatic replies (out-of-office and similar) don't count as replies.

### The Debtor's private page (Round 1, 2026-10-24)

- Each email carries a **signed link, valid for 60 days**, to a page showing the Debtor's open invoices and the owner's own payment instructions. Changing the payment instructions requires the owner to log in again (R12).
- The page has three buttons:
  - **"I've paid"**, with an optional proof upload, which marks the Debtor **"payment claimed"** until the owner confirms;
  - **"I dispute this"**, which stops reminders;
  - **"Stop emailing me"**, the opt-out (R7).

### AI and tone (Round 1, 2026-10-24)

- The same setup as Kompleto: **Claude Haiku 5.5** behind a small client, an optional API key, and **fixed templates as the fallback**, so reminders still go out if the AI is unavailable.
- **The model writes only the wording:** a subject, an opening and a closing. Balanse inserts the invoice list, amounts, dates, the owner's business name and the link, so the model can't garble or invent facts.
- **The model sees only:** the Debtor's business name and the Contact's first name, the number of open invoices, how overdue they are as a range, the language and the reminder step. **Never:** amounts, invoice numbers, notes or replies.
- **Language** per Debtor: Taglish or English. **Tone** steps up from friendly to polite to firm-but-polite, and never threatens.

### Accounts and go-to-market (Round 1, 2026-10-24)

- **Login** by email link, as in Kompleto. **Sign-up** is approved by the operator and asks whether the business is a lender; lenders are refused (R16). Staff logins come later.
- **Go-to-market,** as in Kompleto: free written outreach, 3–5 free 4-week pilots, then **₱1,500/month via GCash**.

## Hard limits (from the research and the brief; binding)

The product rules R1–R18 in [../research/legal-limits.md](../research/legal-limits.md) §8, and "What it never does" in [brief.md](brief.md).

## Open

- Round 2: the dashboard, Debtor and Debt statuses, bounces, partial payments, data retention, leaving, and the web stack.
- Round 3: acceptance criteria.
