# Channels to debtors and payment signals (Balanse)

**Status:** research note, 2026-10-24. **Nothing here is an owner-approved decision.** It covers two questions: how a reminder reaches a debtor in the Philippines, and how Balanse learns that a debt was paid. The law on collecting debts and handling money is covered by a separate legal note; this one only flags where a design would pull Balanse toward handling money.

## Key conclusions

1. **Start with email through the engine, plus "copy and open" for Viber, Messenger, WhatsApp and SMS sent from the owner's own phone.** It costs ₱0 per message, needs no Meta or Viber approval, and the debtor sees a name they know. The trade-off is that only email gets the engine's guarantees (sent exactly once, replies and bounces close the loop). For the other channels, the owner confirms "I sent it" and "they replied" by hand. *(Recommendation)*
2. **Email alone will not reach most consumer debtors.** In a 2021 Pulse Asia survey, 20% of Filipino internet users sent or read email, against 99% using social media [18]. Messenger reaches 90.6% of internet users monthly and WhatsApp 37.6% (GWI, Q2 2025) [17]. Viber is put at 71% penetration, but that comes from Infobip, a vendor [16]. Business debtors (B2B invoices) probably read email more; that is a hypothesis, not tested. *(Facts M; hypothesis marked)*
3. **Automated SMS is the cheapest paid channel: Semaphore at ₱0.56 per 160-character credit** [5][I3]. Leave links out of SMS. NTC memos (2022, amended 2023) tell telcos to block SMS with clickable links, URL shorteners and QR codes [8], and BSP told banks in 2022 to stop sending links, so debtors are trained to treat links in a text as scams [10]. A two-part Taglish reminder costs about ₱1.12. *(H on price; M on link blocking)*
4. **Automated Viber, Messenger and WhatsApp do not fit the budget for v1.** A Viber bot costs EUR 100 a month plus per-message fees [11]. Viber Business Messages carry reseller minimums of about EUR 115 per sender ID per month [12]. Messenger only lets a business message someone who wrote to its Page first, within 24 hours, or with a utility template after Meta's verification chain; three message tags were retired on 27 April 2026 [I3]. WhatsApp templates cost about US$0.011–0.013 each, plus a Tech Provider verification chain [I3][20]. *(H/M)*
5. **The cheapest safe payment signals already exist in the engine.** These are: the owner marks a debt paid; the debtor replies "paid" (the reply stops the loop); and the debtor uploads proof of payment through a signed link (the engine encrypts and virus-scans uploads [I8]). A screenshot is a claim, not proof. In July 2025 GCash warned about AI-generated fake receipts and told users to check the Transactions tab instead [39]. **The owner must always confirm.** *(Fact H; recommendation)*
6. **Payment-link webhooks on the creditor's own account are possible, but no option is free of trade-offs:**
   - **HitPay's OAuth "Connect"** pays out to the merchant's own bank account, with scoped tokens the merchant can revoke [27]. It is in beta, and the tokens can also issue refunds.
   - **PayMongo account linking** makes Balanse a "parent" with a policy contract over "child" accounts [23]. That brings Balanse into a payments-platform relationship.
   - **Xendit xenPlatform** exists to "accept payments on behalf of" merchants and costs ₱85 per active sub-account per month [21]. That is the pull toward handling money that this design should avoid.
   - **Defer all three until pilots show demand.** *(Facts M; recommendation)*
7. **Most micro-creditors cannot get a payment gateway account easily:**
   - Xendit and HitPay require DTI (or SEC) registration [25][26].
   - GCash for Business wants DTI plus BIR 2303 and a GCash account at least 12 months old [32].
   - PayMongo is the exception. An unregistered "Individual" can open an account, gets only QR Ph, and is paid out to a PayMongo Wallet capped at ₱50k [24]. *(M)*
8. **A static personal GCash or Maya QR gives no payment notification a third party can read.** Nothing public was found for it: no API and no webhook. A gateway's dynamic QR Ph does give one, through a webhook [37][28]. *(Absence of evidence; M)*
9. **Importing statements is a weak v1 signal:**
   - Formats are known only for business accounts: GCash for Business exports CSV [34], Maya Business exports transactions [35], and BDO Business Online exports csv/xls/pdf [36].
   - A personal GCash history comes as an emailed, password-protected file [34].
   - No official CSV export was found for personal BPI, BDO or Maya.
   - Statements hold every one of the creditor's transactions, which carries high privacy risk.
   - Matching by amount is ambiguous because P2P transfers carry no Balanse invoice reference.
   - *(M/L)*
10. **Syncing paid status from accounting software is defer-only.** Xero is used by about 11% of Filipino MSMEs (Xero's own figure). Its free Starter tier allows 5 connections, and the Core tier is AUD 35 a month for 50 [I6]. Philippine eligibility for QuickBooks' partner programme is unknown [I6]. Zoho Books' free plan allows 1,000 API calls a day [41]. *(M)*

## How this was checked

- Internal sources were read first and are not repeated here: [I1]–[I8].
- The sandbox's proxy refused direct fetches of every vendor and regulator site tried: paymongo.com, xendit.co, hitpayapp.com, semaphore.co, developers.facebook.com, help.viber.com, developer.xero.com, bsp.gov.ph and ntc.gov.ph. **So every external claim below comes from search-result excerpts of the named page (evidence type SR in Sources), and confidence is capped at M unless an internal source fetched the page earlier.**
- The session's web-search budget ran out before two checks: the exact `sms:` URI syntax for a prefilled body on iOS and Android, and whether `m.me` links accept prefilled text. Both are marked "not verified".
- Exchange rate: ₱62/US$, as in [I4]. No EUR rate was sourced; the only peso figure for the Viber bot fee (~₱6,500) is from [I7].
- Labels: **Fact** (sourced), **Hypothesis** (plausible, untested), **Recommendation** (my proposal, not approved). Vendor statements about themselves are labelled vendor claims.

---

## Part 1. Channels to debtors

### 1.1 What Filipino debtors read

| Fact | Source | Type / conf. |
|---|---|---|
| Messenger used in the past month by 90.6% of internet users aged 16+; WhatsApp by 37.6% (GWI, Q2 2025) | [17] | N reporting survey, SR / M |
| Messenger ad reach 65.8M, 83.3% of adults; Viber and WhatsApp independent counts not found | [I5] | TP / H |
| Viber "71% penetration", second to Messenger's 95%; PH the only one of 30 markets where Viber is major | [16] | VC (Infobip) SR / L–M |
| Infobip: PH business interactions on Viber +47% in 2025, still the leading platform; WhatsApp +1,416% from a small base | [16] | VC SR / L–M |
| 20% of internet users send or read email; 99% use social media (Pulse Asia, 2021) | [18] | N reporting survey, SR / M (dated) |
| Payment proof by GCash screenshot is common; GCash's operator warned about AI fake receipts in July 2025 | [39][I5] | VC/N SR / H |

- **Hypothesis:** consumer and micro-business debtors (a sari-sari store, a tutorial parent, a customer buying on utang) live in Messenger, Viber and SMS. Business debtors paying a supplier's invoice read email. Pilots should record the channel each debtor actually answers on.
- **Gap:** no PH survey of SMS reading or open rates was found.

### 1.2 Email (engine-native)

- **Facts from [I4] (not redone):**
  - Resend Free allows 3,000 emails a month and **100 a day**, with no overage, no card needed, 3 domains and 1 webhook endpoint.
  - Inbound mail received on Resend counts against both caps, so replies arrive through Cloudflare Email Routing instead (₱0).
  - Brevo Free is the only ₱0, no-card overflow found: 300 a day, but SPF does not align.
  - A .com costs about ₱650–690 a year.
- **What follows for Balanse (inference, H):** Resend's 100-a-day cap is shared by **all** Balanse customers, not each one. At about 5 reminders per customer per day, the free tier serves roughly 20 customers. After that comes Resend Pro (US$20/month [I4]) or a second provider.
- **Engine fit:** full. The outbox sends exactly once; Resend blocks duplicate idempotency keys; replies and bounces stop the loop; auto-replies are recognised [I8][I1].
- **Sender identity:** messages come from Balanse's domain with the creditor's name and a reply-to address. Sending from the creditor's own Gmail needs Google's `gmail.send` review, and reading replies there needs a restricted scope with a paid annual assessment [I6][I3]. Defer.

### 1.3 SMS

| Provider | Price per message | Notes | Source |
|---|---|---|---|
| **Semaphore** | **₱0.56 per credit**. Homepage says ex-VAT, FAQ says VAT-inclusive (conflict) | First sender name free, approval up to 5 business days with a sample message; extra names 200 credits/month [I3]. Priority queue 2 credits per 160 chars (third-party SDK docs) | [5][I3] PP / H; [7] TP SR / L–M |
| PhilSMS | ₱0.35 (third-party comparison) | Not checked on vendor page | [6] TP SR / L |
| UniSMS | "as low as ₱0.35" | Vendor listing claim | [6] VC SR / L |
| Twilio (PH) | US$0.241 per segment (≈ ₱15) | Alphanumeric sender at same price | [I3] PP / H |
| M360, iTexMo, Movider | not found | — | — |

- **Message length (general SMS rule, not confirmed for Semaphore):** 160 GSM-7 characters per part, 153 per part once a message is split. Emoji or other non-GSM characters cut a part to 70 characters [7]. **Inference:** a polite Taglish reminder with the creditor's name, amount and due date often runs past 160 characters, so budget 2 credits (₱1.12).
- **Sender ID:** Vonage says pre-registering an alphanumeric sender ID is mandatory on all PH networks and unregistered senders get overwritten [9] (VD SR, M). In practice Semaphore's approval step covers this. Each creditor's own sender name would need its own approval and fee, so v1 should use one Balanse sender name and put the creditor's name in the text (recommendation).
- **Links:**
  - An NTC memo of 12 Sept 2022, amended 27 Sept 2023, tells Globe, Smart and DITO to block SMS with clickable domains, URLs, shorteners, Smart Links and QR codes [8] (N/press release SR, M).
  - Whether registered A2P senders are exempt: **not found** [I3].
  - BSP memo M-2022-015 (22 Mar 2022) told banks to remove links from texts and emails to retail clients, and the public campaign says such messages should be treated as suspicious [10] (N SR, M).
  - **Design rule (recommendation):** no links in SMS. Give the amount, how to pay (the creditor's GCash name or bank details as text), and "reply to this message or contact [name]".
- **Engine fit (inference):** an SMS `Mailer`-like adapter is product or engine work. Semaphore idempotency keys: **not found**. Under the engine's rule, a timed-out SMS becomes "unconfirmed" and is never re-sent [I8]. Inbound replies to Semaphore numbers: not researched.
- **Cost per customer (hypothesis):** 40 debtors × 3 reminders × 2 credits × ₱0.56 = **₱134 a month**, about 9% of ₱1,500. Charge for SMS as metered credits or cap it per plan.

### 1.4 Viber

| Option | Cost | Facts | Source |
|---|---|---|---|
| Chatbot (bots created since 5 Feb 2024) | **EUR 100 per bot per month** + per delivered chatbot-initiated message (country rate card; PH rate not found). Session messages within 24 h of a user message unlimited | Commercial terms only, via Rakuten Viber or partners | [11] OD SR / H; [I3] |
| Viber Business Messages (sender-ID product, via resellers) | Minimum **EUR 115 per active sender ID per month** (Infobip, "all other destinations"); first calendar month free; one reseller adds a EUR 9.99 validation fee. Alibaba Cloud lists PH marketing message ≈ US$0.0096 with a US$197/month minimum | Sold only through partners | [12] PP-reseller SR / M; [13] SR / L–M |
| Free Viber business account (launched Oct 2023, PH first market) | ₱0 | Business profile, catalogue, separate Business Inbox; broadcasts and paid tiers "planned". API not found | [15] VC/N SR / M (status after 2023 not confirmed) |
| **Forward link** `viber://forward?text=<url-encoded>` | ₱0 | Opens Viber with the text; **the user picks the contact** (no number parameter documented). Reports of text truncation and iOS problems | [14] VD SR / H; issues from forums, L |

- **Inference:** Viber is probably the second channel for consumer debtors after Messenger. Every automated Viber route costs more each month than one Balanse customer pays (EUR 100–115 ≈ ₱6,500 [I7] vs ₱1,500). Copy-and-open is the only ₱0 route.

### 1.5 Facebook Messenger

All facts here are from [I3] (Meta docs fetched 2026-10-18) unless marked.

- **Who can be messaged:** only people who messaged the business's **Page**. The 24-hour standard window opens on a user action.
- **Outside the window:**
  - HUMAN_AGENT lets a person (not automation) reply within 7 days. It needs App Review.
  - CONFIRMED_EVENT_UPDATE, ACCOUNT_UPDATE and POST_PURCHASE_UPDATE return **error 100 after 27 April 2026**.
  - The replacement is **Utility Messages** templates ("order or account status updates, and appointment or event reminders"). They need the `page_utility_messaging` permission and must contain no marketing.
  - **Messenger utility pricing: not found** (again in this pass).
- **Serving other businesses' Pages:** needs Business Verification (a registered business in practice), App Review for Advanced Access, and Tech Provider Access Verification. Plan for 2–6 weeks [I3].
- **Personal Messenger accounts have no API** [I7]. Most micro-creditors chat with debtors from a personal profile, not a Page (hypothesis).
- **Whether a payment reminder counts as an "account status update" utility message:** not found. This is Meta's call per template.
- **Copy and open:** the owner copies the approved text and opens Messenger. Whether an `m.me` link can prefill text: **not verified** (search budget ran out).
- **Platform risk:** the government publicly discussed a Facebook block in August 2026 [I5]. Do not make Messenger the only channel.

### 1.6 WhatsApp

- **Usage:** 37.6% of PH internet users monthly [17]. [I5] hypothesised a minority channel used for foreign clients and OFW families. Infobip reports fast growth from a small base [16].
- **Cloud API facts from [I3]:**
  - Per-message pricing; PH is billed in "Rest of Asia Pacific".
  - Utility/authentication templates **US$0.0130**; marketing US$0.0968 (Gallabox's copy of the rate card, effective 1 Oct 2026). A third party lists **US$0.0113** utility and US$0.0842 marketing [20] (TP SR, L). **Conflict; Meta's own CSV not read.**
  - Service messages are chargeable from 1 Oct 2026 after 1,000 free a month per number (M).
  - Tech Provider chain: 10 onboardings a week until verified.
  - Each customer must add a payment method to their WhatsApp account.
- **Inference:** about ₱0.70–0.81 per template, more than an SMS credit. The customer pays Meta directly, which is an onboarding hurdle.
- **AI policy:** allowed. AI that words a creditor's own reminders is "ancillary", not the primary functionality [I3].
- **Copy and open:** `https://wa.me/<number>?text=<url-encoded>` opens **the debtor's chat** with the text prefilled. Nothing is sent until the user taps send [19] (WhatsApp FAQ quoted by Bird, SR, M). This is the cleanest copy-and-open of the four apps because it chooses the recipient.

### 1.7 Copy and open: how it fits the engine

- **Flow (recommendation):**
  1. The owner approves a draft for channel X.
  2. Balanse shows the approved text with a Copy button and an Open button (wa.me, viber://forward, `sms:` for the owner's own phone, Messenger).
  3. The owner taps "Sent" (or "Didn't send").
  4. The owner later records "replied" or "paid".
- **What it costs:** ₱0 to Balanse. The owner sends from their own app and their own load or unlimited-text promo, so the debtor sees the owner's real number or profile (fewer scam fears; hypothesis).
- **What it gives up:**
  - **Exactly-once is not guaranteed.** The engine cannot see the send, so it records the owner's statement.
  - Replies don't reach Balanse.
  - Delivery is unknown.
- **Product rule:** every copy-and-open message must be confirmed by the owner before the next step is scheduled, and a debtor's follow-up count only advances on that confirmation.
- **Engine scope (inference):** a "manual" channel is product state, not an outbox send. Whether it belongs in Loopwright (a second product needing it) is an owner decision.
- `sms:` URI body syntax across iOS and Android: **not verified** (search budget).

### 1.8 Channel comparison

| Channel | Cost to Balanse per message | Monthly fixed | Setup | Reaches consumer debtors | Engine guarantees | Links allowed |
|---|---|---|---|---|---|---|
| Email (Resend) | ₱0 up to 100/day shared; then US$20/mo plan | ₱0 (+domain ≈ ₱650/yr) | Domain + DNS | Low (20% read email, 2021) | **Full** | Yes |
| SMS (Semaphore) | ₱0.56–1.12 | ₱0 | Sender name ≤ 5 days | High (hypothesis) | Send only; "unconfirmed" on timeout | **No** |
| Copy-and-open (Viber/WhatsApp/Messenger/own SMS) | ₱0 | ₱0 | None | High | None (owner confirms) | Yes (app chat) |
| Messenger API | not found (utility) | ₱0 | Page + 3 Meta reviews, 2–6 wks | Only debtors who messaged the Page | Possible | Yes |
| WhatsApp API | ≈ US$0.011–0.013 per template | ₱0 | Tech Provider chain; customer adds card | Medium (37.6%) | Possible | Yes |
| Viber bot / Business Messages | per message (PH rate not found / ≈ US$0.0096) | EUR 100–115+ per sender | Partner contract | High | Possible | Yes |

### 1.9 Recommended channel mix for a ₱0 start *(Recommendation)*

1. **Email through the engine** for every debtor with an address. This is the default for B2B invoices.
2. **Copy-and-open** for Viber, WhatsApp (wa.me with number), Messenger and the owner's own SMS. This is the default for consumer and micro debtors.
3. **Semaphore SMS as a paid add-on**, text only with no links, once a customer pays. It is the only automated non-email channel that fits ₱1,500/month.
4. Do not apply for the Meta, WhatsApp or Viber APIs for v1. Revisit Messenger utility templates if pilots show many debtors who first wrote to the creditor's Page.

---

## Part 2. How Balanse learns a debt was paid

### 2.1 Signals the engine already supports

| Signal | How | Facts | Source |
|---|---|---|---|
| Owner marks paid | Button per debt; stops follow-ups | No external dependency | — |
| Debtor replies "paid" | Inbound email reply stops the loop; model may *suggest* "claims paid" for owner review | Engine parses replies and recognises auto-replies; the model never decides | [I8][I1] |
| Debtor uploads proof | Signed link in the email → upload page | Engine: per-file encryption, ClamAV scan that fails closed, deletion deadline | [I8] |

- **Proof is a claim.** GCash says that if a transaction doesn't appear in your own app's history it "should not be considered valid", even if the screenshot looks real. AI tools (ChatGPT, Grok) have produced convincing fake BPI and GCash receipts [39].
- **Rule (recommendation):** proof only moves a debt to "Payment claimed – check your GCash/bank". The owner confirms "Paid". Balanse never marks a debt paid from a screenshot.
- **Do not** read amounts or reference numbers out of screenshots with the model in v1. It adds an untrusted-input path and privacy exposure for little gain over the owner glancing at their own app.

### 2.2 Payment links on the creditor's own account

**Fees** (VAT treatment noted where found; all SR):

| Method | PayMongo [1] | Xendit [3] | HitPay [4] | GCash for Business [33] | Maya Business [2] |
|---|---|---|---|---|---|
| QR Ph | 1.34% | 1.4% (min ₱15) per third party; calculator shows "1.00% or ₱15", unclear | "1.0% or ₱20" in one post; unconfirmed | **1.0%** | 1.5% [2] |
| GCash | 2.23% | 2.3% | 2.3% | (QR Ph rate) | — |
| Maya | 1.96% (dev docs) vs 1.79% (blog): conflict | 1.8% | not found | — | not found |
| Local cards | 3.125% + ₱13.39 | 3.2% + ₱10 | 3% + ₱15 | 3.2% | 3.5% + ₱10 [2] |
| Online banking / direct debit | 0.71% or ₱13.39 [I3] | 1% or ₱15 (BDO ₱25) | not found | — | — |
| Monthly fee | none | none found | none (vendor claim) | none | not found |
| VAT | "already VAT-inclusive" (dev docs) vs VAT-exclusive [I5]: conflict | not found | not found | VAT-inclusive | — |

- Fees are paid by the creditor to their own gateway; Balanse touches no money.
- **Debtor side:** InstaPay transfers fell to ₱10 on GCash and Maya and ₱0 on BPI and BDO apps after a BSP pricing change in July 2026 [40] (N SR, M). **Inference:** a plain bank or e-wallet transfer costs the debtor ₱0–10 and the creditor nothing. A payment link costs the creditor 1–3%. Many micro-creditors will keep asking for direct transfers, so link-based signals will cover only part of the debts.

**Who can sign up:**

| Provider | Unregistered individual / freelancer | Sole proprietor | Source |
|---|---|---|---|
| PayMongo | **Yes, as "Individual"**: ID + liveness check, TIN on the form; **QR Ph only** on activation; payouts to a PayMongo Wallet (₱50k wallet limit, ₱10k monthly cash-in) | DTI certificate, owner ID, BIR 2303; payouts to a bank account in the business name | [24] VD SR / M |
| Xendit | **No**: "does not accommodate Individual business" (an activation table shows an "Individual" column: conflict) | Gov't ID, TIN, DTI, BIR 2303, online store link | [25] VD SR / M |
| HitPay | No evidence of an individual path | DTI (SEC for companies); third party adds BIR 2303 and business permit | [26] VD/TP SR / M |
| GCash for Business | No | DTI + BIR 2303; fully verified GCash account **≥ 12 months** old; low-risk industry. A third-party "ID-only" tier contradicts this (L) | [32] VD SR / M |
| Maya Business | not found | DTI for the Business Manager upload; BIR 2303 not mentioned | [31] VD SR / L–M |

**Webhooks:**

| Provider | Paid event | Verification | Source |
|---|---|---|---|
| PayMongo | `link.payment.paid` (Links API); also `payment.paid`, `checkout_session.payment.paid` | HMAC-SHA256 in `Paymongo-Signature`, per-webhook secret | [28] VD SR / M |
| Xendit | invoice paid/expired callback | `x-callback-token` static token + optional IP allowlist; retries, so handle duplicates | [29] VD SR / M |
| Maya | `PAYMENT_SUCCESS` / `PAYMENT_FAILED` for Checkout | IP allowlist suggested. **"Email Invoicing and Payment Links do not support webhooks"** | [30] VD SR / H |
| HitPay | not found in this pass | — | — |
| GCash for Business | no public webhook for small merchants found | — | — |

**Creating links on the merchant's behalf without holding funds:**

| Mechanism | How it works | Funds | Pull toward money handling | Source |
|---|---|---|---|---|
| **HitPay OAuth "Connect"** (beta, "available to all businesses") | Merchant authorises; platform gets scoped tokens for payment requests, charges, **refunds**; merchant can revoke. Commission needs a separate Platform Key | Settle to each merchant's own bank account; HitPay holds the licences and does KYB | **Low** if Balanse takes no commission and requests no refund scope (if scopes allow it: not found) | [27] VD SR / M |
| PayMongo account linking (formerly "Platforms"/"Seeds") | Balanse becomes a parent (own KYB); invites existing accounts (child accepts) or creates children through the Onboarding API; acts with the parent key + `Account-Id` header under a policy contract; either side can detach | Child payouts listed separately; whether they always settle to the child's own bank account: not found | **Medium**: Balanse needs its own merchant account and a contract over others' accounts; ask the legal note | [23] VD SR / M |
| Xendit xenPlatform | Sub-accounts, accept payments on behalf, split rules, payouts; ₱85 per active sub-account/month + 0.5% (cap ₱35) on splits, billed to the platform | Platform-orchestrated | **High**: avoid | [21] VD SR / H–M |
| Creditor pastes their own secret API key | Balanse calls the gateway as the merchant | Merchant's | Low on funds, but **high security risk**: a full-power key can refund; restricted keys not found | inference |
| **Creditor pastes their own payment link URL** | Balanse just puts it in the email | Merchant's | **None**. Gives no signal; the owner still marks paid | inference |

### 2.3 QR Ph

- **Facts:**
  - QR Ph is the national standard (BSP Circular 1055, 2019). Proprietary payment QRs had to be switched off from 1 July 2023 [37][32] (SR, M).
  - **Static** QR: printed, never changes, the payer types the amount. **Dynamic** QR: amount encoded; PayMongo's are single-use and expire after 30 minutes by default [37] (SR, M).
  - GCash's personal "Receive via QR" code does not expire and hides the number [38] (N/VD SR, L–M).
  - One explainer says that from 29 July 2026 "QR Ph" means merchant (P2M) codes and P2P moves to "InstaPay QR". Single source, not confirmed with BSP [37] (L).
- **Notification:**
  - A personal static QR notifies only the account holder, in their own app.
  - No API, webhook or delegated read access for personal GCash or Maya accounts was found.
  - **Conclusion: a third party cannot learn of a payment to a personal QR.** This rests on absence of evidence, not a vendor statement (M).
  - A gateway's dynamic QR Ph fires the gateway's normal webhook [28].
- **Notable path (inference):** PayMongo's unregistered "Individual" tier can take QR Ph online at 1.34% with a webhook. It is the only gateway route found for creditors without DTI, but the ₱50k wallet limit may block bigger creditors [24].

### 2.4 Statement import

| Source | Export | Notes | Source |
|---|---|---|---|
| GCash (personal) | "Request transaction history": emailed file, up to 4 years, password = last name + last 4 mobile digits; file format not stated | In-app list shows 90 days or last 15 transactions (help pages conflict) | [34] VD SR / H |
| GCash for Business | **CSV** from the business portal's Transactions tab | — | [34] VD SR / M |
| Maya Business | "Export Transactions" from Business Deposit; monthly statement PDF | Personal Maya: not found | [35] VD SR / M |
| BDO Business Online | csv / xls / pdf, up to 90 days | From a dated (≈2018) guide uploaded to Scribd | [36] TP SR / L |
| BPI, BDO personal | PDF statements (third-party converter sites); official CSV: **not found** | — | [36] TP SR / L |

- **Matching (inference):**
  - A P2P transfer carries the sender's name (often masked), the amount, a time and the sender app's reference number. None of these is Balanse's debt ID.
  - Matching must use amount + date window + name fuzziness, and would produce false matches when two debtors owe the same round amount.
  - So it can only **suggest** matches for the owner to confirm.
- **Privacy (inference):** a statement exposes all of the creditor's income and spending, including personal items, plus the names of people who are not debtors. If built, parse in memory, keep only the matched rows, and delete the file (the engine already supports deletion deadlines [I8]). Data Privacy Act duties go to the legal note.
- **Parsing:** each bank and app format is unofficial and can change without notice. Password-protected files mean asking for the password or the owner unlocking the file first.

### 2.5 Accounting software paid-status sync

| Platform | API access cost | Gate | PH fit | Source |
|---|---|---|---|---|
| Xero | Starter free (5 connections); Core AUD 35/mo (50); Plus AUD 245/mo (1,000) | No listing below Plus; certification needs customers; no PH exclusion | 11% of Filipino MSMEs use digital accounting (Xero claim); Xero PH Lite capped at 5 invoices | [I6] |
| QuickBooks Online | Builder free (500k CorePlus credits/mo); Silver US$300/mo | App assessment + security review for any production app; **PH eligibility unknown** | Has a PH site | [I6] |
| Zoho Books | API on Free plan: 1,000 calls/day, 100/min, 5 concurrent | Webhooks on Free plan and an invoice-paid event: not confirmed | Called "popular in PH" by vendors | [41] VD SR / H; [I6] |

- **Inference:** a creditor who already keeps invoices in Xero, QuickBooks or Zoho probably also uses their built-in reminders (Intuit's own Payments Agent writes reminders with AI [I6]). For Balanse's micro-SME target, sync is a later add-on. Xero Starter (5 free connections) is enough for a pilot test.

---

## 3. Signal comparison

| Signal | Cost to Balanse | Customer setup | Reliability | Privacy risk | Pull toward handling money |
|---|---|---|---|---|---|
| Owner marks paid | ₱0 | None | High (owner knows); depends on owner discipline | Low | None |
| Debtor replies "paid" (email) | ₱0 | None | Medium: a claim; engine stops the loop, owner confirms | Low | None |
| Proof upload via signed link | ₱0 (storage/ClamAV on own server) | None | Medium: screenshots can be faked [39] | **Medium**: receipts show names, numbers, balances; encrypted, deleted on deadline | None |
| Creditor's own static payment link in message | ₱0 | Paste a URL | No signal | Low | None |
| HitPay OAuth webhook | ₱0 (creditor pays 1–3%) | HitPay account (DTI) + Connect click | High (gateway event) | Low–medium (Balanse holds tokens) | Low if no commission, no refund use |
| PayMongo account linking webhook | ₱0; Balanse needs own KYB | PayMongo account + accept link | High | Low–medium | **Medium** (parent/child contract) |
| PayMongo "Individual" QR Ph (creditor's key or link) | ₱0 | ID + liveness; QR Ph only | High | Medium if Balanse holds a full key | Low on funds; high key risk |
| Xendit xenPlatform | ₱85/active sub-account/mo | Sub-account KYB | High | Medium | **High**: avoid |
| Static personal GCash/Maya QR | ₱0 | None | **No signal available** | — | None |
| Bank/e-wallet statement import | ₱0 + parser upkeep | Export, unlock, upload each time | Low–medium (ambiguous matches) | **High** (whole financial life) | None, but looks like financial-data processing |
| Xero/QBO/Zoho status sync | Free tiers; Xero AUD 35+/mo past 5 connections | OAuth connect | High where invoices live there | Medium | None |

---

## 4. Recommendation for v1 and what to defer *(Recommendation, not approved)*

**v1 channels**
- **Email through the engine** (Resend Free, Cloudflare inbound). Plan the move to a paid sender once all customers together pass about 100 emails a day.
- **Copy-and-open** for WhatsApp (wa.me with number), Viber (forward link), Messenger and the owner's own SMS, confirmed by the owner. Follow-ups advance only on that confirmation.
- No links in any SMS-shaped text. Show payment instructions as plain text: the creditor's GCash or bank name and number, entered by the owner.

**v1 payment signals**
1. Owner marks paid. This is the source of truth.
2. A debtor's email reply stops the loop. The model may label it "claims paid" for the owner to review.
3. Proof upload through a signed link, shown to the owner as "payment claimed – check your app". Never auto-paid.
4. Optional: the owner pastes their own payment-link URL (PayMongo, HitPay, Maya or other) into the message. No integration.

**Defer, with the trigger to revisit**
- **Semaphore SMS:** when the first customer pays. Add it as metered credits; about ₱134/month at the volume assumed above.
- **HitPay OAuth webhook:** when several pilot creditors already use HitPay. Ask the legal note first and request no refund scope.
- **PayMongo account linking:** after the legal note on the parent/child relationship.
- **Messenger utility templates:** if many debtors first wrote to a creditor's Page. Needs Business Verification and App Review.
- **WhatsApp Cloud API:** only if pilots show WhatsApp-heavy debtors and creditors willing to add a card for Meta.
- **Statement import and accounting sync:** after pilots. Xero Starter is enough for a test.
- **Never (for this design):** Xendit xenPlatform, collecting payments into a Balanse account, taking a commission on payments, and storing creditors' full-power secret keys.

## 5. Biggest unknowns

1. Which channel PH debtors actually answer on: no survey found. Pilots should log it per debtor.
2. Whether a registered A2P sender ID (Semaphore) is exempt from NTC link blocking: not found.
3. Messenger utility-message pricing, and whether a payment reminder qualifies as an "account status update": not found.
4. PayMongo: whether linked child accounts always settle to their own bank account, and whether restricted API keys exist. HitPay: which OAuth scopes exist (can refunds be left out?) and its webhooks. All not found.
5. PayMongo fee and VAT conflicts (Maya 1.96% vs 1.79%; VAT-inclusive vs exclusive).
6. Official CSV exports for personal BPI, BDO and Maya, and the GCash emailed-history file format: not found.
7. Semaphore idempotency and inbound-reply support: not researched.
8. `sms:` body syntax on iOS/Android and `m.me` prefill: not verified (search budget ran out).

---

## Sources

Accessed 2026-10-24 unless stated. Types: OD official/regulator, PP vendor pricing page, VD vendor docs, VC vendor claim, N news, TP third party, SR search-result excerpt only (page not fetched). "n.d." = no date shown.

**Internal (not redone)**
- [I1] Kompleto, `docs/research/inbound-email.md`, 2026-10-21. Internal research (VD fetched from GitHub sources). H.
- [I2] Kompleto, `docs/research/hosting-and-email-infra.md`, 2026-10-21. Internal. H.
- [I3] Kompleto, `docs/research/integration-constraints.md`, accessed 2026-10-18 (Meta, Viber, Semaphore, Twilio, PayMongo, Xendit sections). Internal; OD/PP H where fetched, M where excerpt.
- [I4] Kompleto, `docs/research/domain-and-email-at-zero.md`, 2026-10-21 (Resend, Brevo, Cloudflare, domain cost, ₱62/US$). Internal. H/M.
- [I5] Kompleto, `docs/research/market-philippines.md` (platform reach, payments, Facebook-block risk). Internal. H/M.
- [I6] Kompleto, `docs/research/self-serve-distribution.md` (Xero tiers, Intuit App Partner Program, Gmail rules). Internal. H/M.
- [I7] Kompleto, `docs/research/ph-industries-sectors.md` (Viber bot ≈ ₱6,500/month, personal chats have no API). Internal. M.
- [I8] Loopwright, `README.md` (outbox, exactly-once, inbound, files, scan, signed links). Internal. H.

**External**
1. PayMongo, "Pricing" / "Fees & Rates", https://developers.paymongo.com/docs/pricing and https://developers.paymongo.com/docs/fees-and-rates; blog "Maya vs GCash vs PayMongo for business", https://www.paymongo.com/blog/maya-vs-gcash-vs-paymongo-for-business. n.d. PP/VC, SR, M (conflicting figures).
2. Maya Business blog figures (cards 3.5% + ₱10, QR Ph 1.5% MDR) as surfaced in the search for [1]; Maya's own pricing page not found. n.d. VC, SR, L.
3. Xendit, "Pricing (Philippines)" https://www.xendit.co/en-ph/pricing/ and "Pricing calculator PH" https://www.xendit.co/en-ph/pricing-calculator-ph/; Prosperna, "PayMongo vs Xendit 2025" https://www.prosperna.com/blog/paymongo-vs-xendit-2025-fees-and-features-for-ph-smes. n.d./2025. PP SR M; TP SR L.
4. HitPay, "HitPay rates" https://hitpayapp.com/blog/hitpay-rates; "Payment gateway Philippines payout comparison" https://hitpayapp.com/blog/payment-gateway-philippines-payout-comparison; "HitPay vs PayMongo fees in 2025" https://hitpayapp.com/blog/hitpay-vs-paymongo-fees-in-2025-which-payment-gateway-saves-philippine-smes-more. 2025. VC, SR, L.
5. Semaphore, homepage https://www.semaphore.co and FAQ https://api.semaphore.co/faq. n.d. PP/VD, SR, H (price; consistent with [I3] fetch).
6. Sent.dm, "Philippines SMS pricing", https://sent.dm/en/resources/sms-pricing/philippines-sms-pricing, 2026-03; UniSMS listing https://peerpush.com/p/unisms-api. TP/VC, SR, L.
7. npm, `semaphore-ph-api` package docs, https://npmjs.org/package/semaphore-ph-api; general SMS segment rules (help.mobilemessage.com.au). n.d. TP, SR, L–M.
8. Globe Telecom newsroom, "Globe lauds NTC memo requiring telcos to block SMS with clickable links" (2023), https://www.globe.com.ph/about-us/newsroom/corporate/globe-lauds-ntc-memo-requiring-telcos-to-block-sms-with-clickable-links; CNN Philippines, 2022-09-13, https://www.cnnphilippines.com/business/2022/9/13/NTC-order-deactivation-links-texts-malicious-sites.html; Malaya, https://malaya.com.ph/business/corporate/telcos-told-to-block-sms-with-clickable-links/. N/company press release, SR, M.
9. Vonage, "Philippines SMS Features and Restrictions", https://api.support.vonage.com/hc/en-us/articles/204017413. n.d. VD, SR, M.
10. NewsBytes, "BSP asks banks to remove links in e-mail, SMS advisories", 2022-06-30, https://newsbytes.ph/2022/06/30/bsp-asks-banks-to-remove-links-in-e-mail-sms-advisories-to-customers/; BusinessWorld https://www.bworldonline.com/?p=437907; Interaksyon https://interaksyon.philstar.com/?p=222753. 2022. N, SR, M (memo text not read).
11. Rakuten Viber Help Center, "Bot commercial model", https://help.viber.com/hc/articles/15247629658525; "Chatbot commercial model – legacy", https://help.viber.com/hc/en-us/articles/8746671603485. n.d. OD (vendor terms), SR, H.
12. Infobip, "Viber pricing", https://www.infobip.com/viber-business/pricing; tyntec https://www.tyntec.com/?p=35147; SMSBAT https://smsbat.com/help/billing/viber-pricing/. n.d. PP (resellers), SR, M.
13. Alibaba Cloud, "Chat App Message Service – Viber", https://www.alibabacloud.com/help/en/chatapp/product-overview/viber. n.d. PP (reseller), SR, L–M.
14. Rakuten Viber Developers, "Share button" https://developers.viber.com/docs/tools/share-button/ and "Deep links" https://developers.viber.com/docs/tools/deep-links; issue reports: Habr Q&A https://qna.habr.com/q/979085, Solar2D forum. n.d. VD SR H; forums L.
15. Rakuten Viber, "Rakuten Viber debuts … toolkit for all businesses", https://www.forbusiness.viber.com/en/blog/post/rakuten-viber-debuts-giant-leap-forward-in-global-strategy-with-its-toolkit-for-all-businesses; NewsBytes 2023-10-11 https://newsbytes.ph/2023/10/11/ph-is-first-country-to-roll-out-vibers-new-toolkit-for-smes/; GMA News https://www.gmanetwork.com/news/money/companies/884757/viber-seeks-to-capture-1m-micro-enterprises-in-ph/story/. 2023. VC/N, SR, M.
16. NewsBytes, "Report: WhatsApp posts rapid growth in PH business messaging but Viber stays dominant", 2026-06-16, https://newsbytes.ph/2026/06/16/report-whatsapp-posts-rapid-growth-in-ph-business-messaging-but-viber-stays-dominant/; NoypiGeeks https://www.noypigeeks.com/?p=272178; Infobip, "Most popular messaging apps by country", https://www.infobip.com/blog/most-popular-messaging-apps-by-country. 2026. VC (Infobip data), SR, L–M.
17. Philstar, "Pinoys top Facebook, YouTube users globally" (GWI Q2 2025), 2025-10-16, https://www.philstar.com/headlines/2025/10/16/2480149/pinoys-top-facebook-youtube-users-globally. N, SR, M.
18. Inquirer, "63 percent of Filipino adults are internet users — Pulse Asia survey", 2021, https://newsinfo.inquirer.net/?p=1500767. N reporting survey, SR, M (dated).
19. Bird, "What is a WhatsApp click to chat link" (quotes WhatsApp FAQ), https://bird.com/explained/whatsapp/what-is-a-whatsapp-click-to-chat-link.md. n.d. TP quoting VD, SR, M.
20. AgentDeals, "WhatsApp Business API pricing 2026", https://agentdeals.dev/whatsapp-business-api-pricing-2026. 2026. TP, SR, L.
21. Xendit, "xenPlatform fees", https://docs.xendit.co/docs/xenplatform-fees; Help Center "How much is xenPlatform fee", https://help.xendit.co/hc/en-us/articles/4413990486041; "Split payments" https://docs.xendit.co/docs/split-payments; overview https://docs.xendit.co/xenplatform. n.d. VD/PP, SR, H–M.
22. Prosperna, "Connect Xendit (PH merchants)", https://help.prosperna.com/payments/connect-xendit-ph-merchants. n.d. TP, SR, L.
23. PayMongo, "PayMongo Platforms" https://developers.paymongo.com/docs/paymongo-platforms; "Onboarding API" https://developers.paymongo.com/docs/onboarding-api; "Child accounts onboarding" https://developers.paymongo.com/docs/child-accounts-onboarding; "Key concepts" https://docs.paymongo.com/docs/onboarding-aas-key-concepts; "Linked accounts" https://docs.paymongo.com/docs/account-settings-linked-accounts. n.d. VD, SR, M.
24. PayMongo, "Philippine entities" https://developers.paymongo.com/docs/philippine-entities; "Account setup" https://developers.paymongo.com/docs/account-settings-account-setup; "Payment method & eligibility" https://developers.paymongo.com/docs/payment-method-eligibility. n.d. VD, SR, M (table layout ambiguous).
25. Xendit Help Center, "Can personal businesses use Xendit's services?" https://help.xendit.co/hc/en-us/articles/360035083911; "[PH] Legal documents required" https://help.xendit.co/hc/en-us/articles/11103239008793; "Activating your account" https://docs.xendit.co/getting-started/activate-account. n.d. VD, SR, M.
26. HitPay Docs, "Onboarding requirements (PH)", https://docs.hitpayapp.com/onboarding/requirements-ph.md; EasyStore, "Connect with HitPay Philippines", https://support.easystore.co/en/article/connect-with-hitpay-philippines-11jnpds/. n.d. VD/TP, SR, M.
27. HitPay Docs, "OAuth" https://docs.hitpayapp.com/platforms/oauth; "Platforms overview" https://docs.hitpayapp.com/platforms/overview; "Platform APIs" https://docs.hitpayapp.com/apis/guide/platform-apis. n.d. VD, SR, M.
28. PayMongo, "Links API" https://developers.paymongo.com/docs/links-api; "Webhooks" https://developers.paymongo.com/docs/webhooks; "Webhook events" https://developers.paymongo.com/docs/developer-tools-webhooks-events. n.d. VD, SR, M.
29. Xendit, "Webhook – getting started" https://docs.xendit.co/webhook/getting-started; Help Center "About webhook" https://help.xendit.co/hc/en-us/articles/360038048871; xendit-node `Invoice.md` https://github.com/xendit/xendit-node/blob/master/docs/Invoice.md. n.d. VD, SR, M.
30. Maya Developers, "Managing webhooks" https://developers.maya.ph/docs/receive-real-time-payment-information-using-webhooks; "Configuring your webhook for Maya Checkout" https://developers.maya.ph/reference/configuring-your-webhook-for-maya-checkout. n.d. VD, SR, H.
31. Maya, "How to open a business deposit or savings account", https://www.maya.ph/business/stories/how-to-open-a-business-deposit-or-savings-account (VD, SR, L–M); Sacra, "Maya payments to lending funnel" (1.5–3.5%) (TP, SR, L).
32. GCash Help Center, "What are the requirements to create a GCash for Business merchant account?", https://help.gcash.com/hc/en-us/articles/48456974006041; ITSourceCode guide (contradicting tier), https://itsourcecode.com/gcash/how-to-set-up-gcash-for-business-2026/. n.d./2026. VD SR M; TP SR L.
33. GCash Help Center, "What is the GCash for Business Merchant Discount Rate (MDR)?", https://help.gcash.com/hc/en-us/articles/55684605325593. n.d. VD, SR, M.
34. GCash Help Center, "How to request transaction history", https://help.gcash.com/hc/en-us/articles/360034155433; "How do I review and download my GCash for Business transaction history?", https://help.gcash.com/hc/en-us/articles/48457463083545. n.d. VD, SR, H (personal) / M (business).
35. Maya Developers, "Receive settlements into your Business Deposit", https://developers.maya.ph/docs/receive-settlements-into-your-business-deposit. n.d. VD, SR, M.
36. Scribd upload, "BDO Business Online Banking guide", https://www.scribd.com/document/489780423; Bankstatemently, BPI and BDO pages, https://bankstatemently.com/banks/ph/bpi/bank-statement. n.d. TP, SR, L.
37. PayMongo, "QR Ph" https://developers.paymongo.com/docs/payment-acceptance-qr-ph (VD SR M); Skyro Help Center, "What is QR Ph" https://www.skyro.ph/helpcenter/qr-ph-usage/what-is-qr-ph (TP SR M); Philstar, "National QR code rolls out July", 2023-03-03, https://www.philstar.com/business/2023/03/03/2248821/national-qr-code-rolls-out-july (N SR M); TransFi, "QR Ph explained" https://www.transfi.com/blog/qr-ph-explained-how-p2p-and-p2m-qr-payments-work-for-individuals-and-merchants (TP SR L).
38. The Paypers, "GCash launches QR code-based money transfer feature", https://thepaypers.com/payments/news/gcash-launches-qr-code-based-money-transfer-feature; Globe GCash page. n.d. N/VD, SR, L–M.
39. Mynt (GCash), "GCash warns public on emerging AI-generated fake receipts scams", 2025-07-14, https://mynt.com.ph/newsroom/gcash-warns-public-on-emerging-ai-generated-fake-receipts-scams-urges-users-to-always-check-transactions-tab (VC/official company notice, SR, H); Unbox PH, "Scammers are using ChatGPT to generate fake transfer receipts", https://unbox.ph/ai/chatgpt-receipts/ (N, SR, M).
40. Philstar, "LIST: Banks, e-wallets cut, waived InstaPay, PESONet transfer fees", 2026-07-07, https://www.philstar.com/headlines/2026/07/07/2540294/list-banks-e-wallets-cut-waived-instapay-pesonet-transfer-fees; Manila Bulletin, "GCash slashes bank transfer fee to ₱10", 2026-07-05, https://mb.com.ph/2026/07/05/gcash-slashes-bank-transfer-fee-to-10; GMA News (Maya). N, SR, M.
41. Zoho, "Zoho Books API v4 – introduction", https://www.zoho.com/books/api/v4/introduction/; KB "Per day API limit", https://www.zoho.com/books/kb/automation/per-day-api-limit.html. n.d. VD, SR, H.
