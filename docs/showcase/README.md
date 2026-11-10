# Showcase

Screenshots and a two-and-a-half-minute captioned walkthrough of a demo Business, Reyes Creative Studio, a small design studio chasing eight clients for unpaid invoices. Reminders are drafted, one is approved, one client says they've paid with a GCash screenshot, one disputes, one promised to pay, the Owner sent one reminder by hand on Viber, and one client's address bounced.

The reminders' wording is Claude's (Sonnet, the model the blind Taglish test chose, with the owner's API key, made on 2026-11-09), drafted the way the live scheduler drafts; Balanse itself inserts the invoices, amounts and the link to the Debtor's page. Without a key, the same demo uses the fixed templates.

| File | Shows |
|---|---|
| `walkthrough.mp4` | The whole loop in about two and a half minutes: the dashboard, a bounced address and the provider's reason, drafts in Taglish and English, copy and open, review and approve, the client's own page and "I've paid", confirming the payment, the evidence log and the CSV it exports, spreadsheet import and settings |
| `desktop-1-dashboard.png` | The Debtors needing you first, then the oldest overdue, with each one's next step |
| `desktop-2-drafts.png` | Drafts waiting for approval, each with the client's private link |
| `desktop-3-review-before-approving.png` | Reviewing several drafts in full before approving them |
| `desktop-4-copy-and-open.png` | Copying a draft for Viber, Messenger, WhatsApp or SMS |
| `desktop-5-payment-claim.png` | A client who says they've paid, with their proof, to confirm or reject |
| `desktop-6-debtor.png` | One Debtor: invoices, reminders, replies, edit details and the evidence log |
| `desktop-7-import.png` | A spreadsheet import, with the row it refused and why |
| `desktop-8-settings.png` | Payment instructions, downloading all your data and closing the account |
| `desktop-9-debtors-page.png` | The client's own page, on a computer |
| `desktop-10-bounced-address.png` | A Debtor whose address bounced: the provider's reason, and reminders stopped |
| `phone-1-dashboard.png`, `phone-2-drafts.png` | The Owner's pages on a phone |
| `phone-3-debtors-page.png` | What the client sees when they open their link, in Taglish |
| `phone-4-debtors-page-english.png` | The same page for a client who reads English |

Everything here is made-up data: the businesses, people, numbers and `.example.test` addresses don't exist.

## Making them again

They are made by Playwright scripts in `web/showcase/`, against the real app: the demo Business is set up through Balanse's own sign-up, import and Debtor pages. The walkthrough approves the waiting drafts and confirms a payment, so each run needs an address that is new to the database.

```sh
docker compose up -d                 # Postgres and Mailpit
pnpm --dir web build
cargo build
RESEND_WEBHOOK_SECRET=whsec_c2hvd2Nhc2Utb25seQ== cargo run -- serve
# in another terminal; the address must be new to this database:
SHOWCASE_EMAIL=ana@reyes-creative.example pnpm --dir web showcase
```

With the API key in `.env` (or `OWNER_ANTHROPIC_API_KEY`), Claude words the drafts.

The bounce is real too: the setup approves two reminders, the outbox sends them into Mailpit, then the setup posts a signed bounce for one of them to `/hooks/resend`, as Resend would; the secret above is the one it signs with. Balanse sends only on weekdays, 08:00 to 20:00 Manila time, so outside those hours start the server on a shifted clock with [`faketime`](https://github.com/wolfcw/libfaketime) (`apt install faketime`), e.g. `RESEND_WEBHOOK_SECRET=… faketime -f +8h target/debug/balanse serve`, choosing the hours that land in a weekday morning in Manila. The setup reads the server's clock and runs its own `balanse` commands on the same shift.

Making the video needs `ffmpeg` on the path.
