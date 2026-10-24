# Balanse spec: first version

Status: **Phase 3 complete (2026-10-24).** Built up round by round; everything under "Decided" is owner-approved, with dates. Background: [brief.md](brief.md) (approved 2026-10-24) and [../research/README.md](../research/README.md). The acceptance criteria were approved in Round 3.

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

### Statuses (Round 2, 2026-10-24)

- A **Debt** is **open**, **payment claimed** (the Debtor said so; it stays this way until the owner confirms), **paid**, **disputed**, **written off** (the owner gave up) or **cancelled** (entered by mistake).
- **Partial payment:** the owner edits the amount still owed. Balanse never does the arithmetic, and the change is logged.
- A **Debtor** shows one overall state: active, paused after a reply, promised until a date, needs your call, payment claimed, disputed, opted out, bounced, or settled. Reminders are drafted only while it is active.

### Dashboard (Round 2, 2026-10-24)

- The home page lists Debtors needing action first (replied, payment claimed, needs your call, disputed, bounced, Drafts waiting), then active Debtors, oldest overdue first.
- Each row shows the number of open invoices, the oldest due date, the last message and its status, and the next step.
- **No grand total of money owed, no aging buckets and no receivables export** (R11; BIR's CAS rules, `../research/legal-limits.md`).

### Bounces (Round 2, 2026-10-24)

- As in Kompleto: a permanent bounce or a spam complaint stops all reminders to that address until the owner changes it. Temporary bounces only show on the dashboard.

### Proof-of-payment files (Round 2, 2026-10-24)

- Kompleto's file rules: JPEG, PNG, HEIC/HEIF, WebP or PDF, up to 10 MB each, recognised by their contents; at most **3 files per claim**; encrypted with a per-file key and virus-scanned with ClamAV, failing closed; downloaded only as attachments.
- Deleted **30 days after the owner confirms or rejects the claim**, and never kept more than **90 days**.

### Records and retention (Round 2, 2026-10-24)

- **Reply text** is deleted after 90 days; only "replied on <date>" remains.
- Each Debt's **evidence log** (approved text, send time, delivery events, reply dates, status changes) is kept until **1 year after the Debt is closed**, then deleted together with the Debtor's contact details if no open Debt remains (R17).
- The owner can **export a Debtor's evidence log** as a file at any time.

### Leaving (Round 2, 2026-10-24)

- The owner can **download all their data**: Debtors, Debts and the message log.
- Closing the account **deletes everything within 30 days**, except Balanse's own billing records.

### Copy and open (Round 2, 2026-10-24)

- Every Draft and approved message has **"Copy for Viber/Messenger"** (the text with the private-page link), **"Open WhatsApp"** (the text filled in; needs the Contact's mobile number) and **"Copy for SMS"** (a version with no link and no phone number, R13).
- Copying never approves or sends anything.
- **"I sent it on …"** logs a manual send, which counts toward the frequency cap.

### Web technology (Round 2, 2026-10-24)

- As in Kompleto: the owner's pages are a **React app served by the Rust binary** through a JSON API; the Debtor's page is server-rendered. Postgres is the only infrastructure; pilots run on free hosting.
- Whether Balanse shares Kompleto's Resend account (100 free emails a day in total) or has its own is decided in the architecture phase.

### Pilots (Round 2, 2026-10-24)

- A pilot round with 3–5 businesses **succeeds only if all four hold**:
  - ≥ 70% of Drafts are approved as-is or with small edits;
  - ≥ 1 in 3 reminded Debtors replies or pays within 14 days;
  - each business uses Balanse at least once a week;
  - ≥ 1 business pays ₱1,500 for the next month.
- Every pilot also logs the brief's three measures: overdue invoices never chased before, Drafts softened before approval, and days-to-pay before vs after.

## Hard limits (from the research and the brief; binding)

The product rules R1–R18 in [../research/legal-limits.md](../research/legal-limits.md) §8, and "What it never does" in [brief.md](brief.md).

## Acceptance criteria (owner-approved 2026-10-24, Round 3)

Balanse's first version is done when every criterion below passes. Each is checked by an automated test whose name starts with its ID. "Owner" means a business's account; "Debtor" means one of its clients.

**Accounts and isolation**
- **A1.** A login link works once and expires after 15 minutes. An address with no account gets the same "check your email" screen and no email.
- **A2.** One owner can never see or change another owner's Debtors, Debts, messages or files; using another owner's IDs returns "not found".
- **A3.** Sign-up asks whether the business lends money; a "yes" is refused with a reason, and no account request is stored (R16).

**Debts in**
- **B1.** A Debt can't be saved without an invoice number, invoice date, amount, due date and the Debtor's Contact email.
- **B2.** A spreadsheet import (CSV or Excel) adds every valid row and lists each invalid row with its reason; importing the same file twice adds nothing new.
- **B3.** An amount is stored and shown exactly as typed; editing it is recorded in the evidence log.

**Schedule**
- **C1.** With default settings, a Debtor gets Drafts the day after its earliest open Debt's due date and 7 and 14 days after it, each appearing at 07:00 Manila time.
- **C2.** A Debtor with several overdue Debts gets one Draft listing all of them.
- **C3.** After three unanswered reminders the Debtor becomes "needs your call" and no further Draft is created until the owner acts.
- **C4.** No Debtor is sent more than one message in 3 days or two in 7 days, counting manual sends; a step that would break the cap waits.
- **C5.** A step falling on a Saturday or Sunday moves to Monday; nothing is sent outside weekdays 08:00–20:00 Manila time, and a message approved later waits for the next allowed time.
- **C6.** The heads-up is off by default; when the owner turns it on, a Draft appears 3 days before the due date.
- **C7.** No Draft is created for a Debtor that is paused, promised, disputed, payment claimed, opted out, bounced or settled.

**Approval and sending**
- **D1.** Nothing is sent without the owner's approval of that exact text; editing a Draft voids its approval.
- **D2.** Batch approval approves only Drafts whose text still matches what was shown.
- **D3.** An unapproved Draft expires 3 days after it appears; the step is logged as skipped.
- **D4.** The pre-send check refuses or blocks a message whose recipient isn't the Debtor's stored Contact, that has more than one recipient, any link other than the Debtor's own page, or an invoice list or amount differing from the stored Debts.
- **D5.** The content guard blocks threats, insults, shaming and impersonation of a lawyer, court or government office, in English, Tagalog and Taglish.
- **D6.** A crash in the middle of sending never sends a message twice.
- **D7.** Every email names the owner's business, says "sent via Balanse" and carries the privacy line.

**AI**
- **E1.** A model request carries only the Debtor's business name, the Contact's first name, the number of open invoices, the overdue range, the language and the reminder step; never an amount, invoice number, note or reply.
- **E2.** If the model fails, times out or returns invalid output (wrong shape, too long, a link, an amount, an email address or a long run of digits), the Draft uses the template at once.

**Replies and bounces**
- **F1.** A human reply to a signed reply address pauses the Debtor's reminders and notifies the owner within 5 minutes, without the reply's text.
- **F2.** An automatic reply doesn't pause reminders.
- **F3.** "Promised to pay" by a date resumes reminders the day after it; "disputed" stops them; "paid" settles the Debts; "resume" restarts the schedule.
- **F4.** A permanent bounce or complaint stops all reminders to that address until the owner changes it; delivery events count only when they match a message Balanse sent.
- **F5.** Reply text is deleted 90 days after it arrives.

**The Debtor's page**
- **G1.** The page opens only with a valid signed link younger than 60 days; any other link shows the same "not found" page.
- **G2.** "I've paid" marks the Debtor "payment claimed" and pauses reminders until the owner confirms or rejects the claim.
- **G3.** "I dispute this" stops reminders; any later message the owner approves says the amount is disputed.
- **G4.** "Stop emailing me" stops email to that Debtor; the owner alone can't turn it back on.
- **G5.** Proof files outside the allowed types, over 10 MB or beyond 3 per claim are refused; a file the virus scanner can't confirm clean is refused; files are deleted 30 days after the claim is settled and never kept more than 90 days.
- **G6.** Changing the payment instructions requires the owner to log in again.

**Dashboard and records**
- **H1.** The dashboard lists Debtors needing action first, then active Debtors oldest overdue first.
- **H2.** No page, API response or export contains a total of amounts owed across Debtors, an aging breakdown, or anything titled invoice, receipt or statement of account.
- **H3.** The owner can export one Debtor's evidence log; logs are deleted 1 year after the Debt is closed.
- **H4.** "Copy for SMS" text contains no link and no phone number; "I sent it on …" counts toward the cap; copying changes no approval.
- **H5.** The owner can download all their data, and closing the account deletes it within 30 days.

## Open

- Architecture (Phase 4): what moves from Kompleto into Nudgewright, and whether Balanse has its own Resend account.
- Before pilots: the owner's checks of First Circle Invoice, Lista and PayMongo Invoices (brief, kill-checks).
