# Channels to debtors and payment signals (Balanse)

**Status:** research note, 2026-10-24. Verification pass the same day against the live pages (see "How this was checked"). **Nothing here is an owner-approved decision.** It covers two questions: how a reminder reaches a debtor in the Philippines, and how Balanse learns that a debt was paid. The law on collecting debts and handling money is covered by a separate legal note; this one only flags where a design would pull Balanse toward handling money.

## Key conclusions

1. **Start with email through the engine, plus "copy and open" for Viber, Messenger, WhatsApp and SMS sent from the owner's own phone.** It costs ₱0 per message, needs no Meta or Viber approval, and the debtor sees a name they know. The trade-off is that only email gets the engine's guarantees (sent exactly once, replies and bounces close the loop). For the other channels, the owner confirms "I sent it" and "they replied" by hand. *(Recommendation; unchanged by verification)*
2. **Email alone will not reach most consumer debtors:**
   - In Pulse Asia's July 2026 survey, **37% of internet users** send, receive or read email (25% in March 2026). The share is 22% in class E and 16% in Mindanao [42].
   - With 85% of adults online, **inference:** about 3 in 10 Filipino adults use email.
   - By comparison, Messenger is used monthly by 90.6% of internet users and WhatsApp by 37.6% (GWI, Q2 2025) [17]. Messenger's ad reach is 65.8M, or 83.3% of adults [43].
   - Viber is put at 71% penetration, but only by Infobip, a vendor [16].
   - Business debtors (B2B invoices) probably read email more. That is a hypothesis, not tested.
   - *(Facts H; hypothesis marked)*
3. **Automated SMS is the cheapest paid channel: Semaphore at ₱0.56 per credit.** A credit covers 160 characters; longer messages cost ceiling(length/153) credits [5].
   - **Links are not banned for A2P SMS through Semaphore.** Semaphore says Globe's 2022 link block covers person-to-person SMS only, not its A2P traffic. Smart blocks link-shortener URLs, so use full domains [45].
   - The NTC's amended memo of 27 September 2023 tells telcos to block texts with clickable links, shorteners and QR codes. Whether registered A2P senders are exempt is still **not stated** in any source read [8].
   - BSP told banks in March 2022 to stop sending clickable links to retail customers [10], so debtors are trained to distrust links in texts.
   - **Recommendation unchanged:** no links in v1 SMS. This is now a trust choice, not a technical ban.
   - *(H on price and the Semaphore advisory; M on how NTC enforcement works today)*
4. **Automated Viber, Messenger and WhatsApp do not fit the budget for v1:**
   - **Viber:** a bot now costs **EUR 115 a month** (Viber's own page, edited 1 Sept 2026; earlier sources said EUR 100) plus per-message fees [11]. Viber Business Messages carry a minimum of **EUR 115 per sender ID per month** [12].
   - **Messenger:** a business can only message someone who wrote to its Page first. Inside 24 hours that is free text; after that it needs a utility template. Meta's utility documentation states no price. Three message tags were retired on 27 April 2026 [46].
   - **WhatsApp:** templates cost about US$0.011–0.013 each, plus a Tech Provider verification chain [I3][20].
   - *(H/M)*
5. **The cheapest safe payment signals already exist in the engine:**
   - the owner marks a debt paid;
   - the debtor replies "paid", and the reply stops the loop;
   - the debtor uploads proof of payment through a signed link. The engine encrypts and virus-scans uploads [I8].
   - A screenshot is a claim, not proof. In July 2025 GCash warned about AI-generated fake receipts and told users to check the Transactions tab instead [39]. **The owner must always confirm.** *(Fact H; recommendation)*
6. **Payment-link webhooks on the creditor's own account are possible. Verification made HitPay cleaner and Xendit worse:**
   - **HitPay "Connect your HitPay account" OAuth** (beta, "available to all businesses"): payouts go straight to each merchant's own bank account and are "not pooled through the platform". Scopes are granular, so Balanse can request `payments:create` and `payments:read` and **leave out `payments:refund`**. HitPay reviews scope requests [27].
   - **PayMongo account linking:** a parent can be given narrow, per-module access (for example, view payments only). The policy contracts that govern this ship in Q4 2026 [23].
   - **Xendit xenPlatform:** from 1 Feb 2027 it costs USD 10,000 to set up plus **USD 7,500 a month** per platform [21]. It exists to "accept payments on behalf of" sub-accounts. Avoid.
   - **Defer all three until pilots show demand.** *(Facts H; recommendation)*
7. **Getting a gateway account without DTI is easier than first reported:**
   - **GCash for Business Starter Wallet** (since 1 Sept 2026) needs **no business documents**: a GCash account verified for at least 12 months, a ₱5M monthly limit, and ready "within 10 minutes". It offers a CSV export only; no API or webhook was found [32][53].
   - **PayMongo "Individual" accounts** get QR Ph on activation. With a website, they can also request online banking, Maya, GrabPay and ShopeePay. GCash and cards need a registered business [24]. **The "₱50k wallet cap" reported earlier is not in PayMongo's current docs** (the default merchant wallet cap is ₱25M).
   - **Xendit** accepts individuals only as xenPlatform sub-accounts [25]. **HitPay** needs DTI, BIR 2303, proof of business and a business bank account [26].
   - *(H)*
8. **A static personal GCash or Maya QR gives no payment notification a third party can read.** Nothing public was found: no API and no webhook. A gateway's dynamic QR Ph does give one. PayMongo's are single-use, expire after 30 minutes and confirm by webhook [37][28]. *(Absence of evidence; M)*
9. **Importing statements is a weak v1 signal:**
   - **GCash personal:** the history arrives as an emailed PDF, password = last name in lowercase plus the last 4 mobile digits [34].
   - **GCash for Business:** CSV export [34].
   - **Maya Business:** exports transactions [35].
   - **BDO Business Online:** csv/xls/pdf, from an old guide [36].
   - No official CSV export was found for personal BPI, BDO or Maya.
   - Statements hold all of the creditor's money movements (high privacy risk), and matching by amount is ambiguous.
   - *(H for GCash; L for banks)*
10. **Syncing paid status from accounting software is defer-only:**
    - **Xero:** used by about 11% of Filipino MSMEs (Xero's own figure). Starter is free for 5 connections; Core costs AUD 35 a month for 50 [I6].
    - **QuickBooks:** Philippine eligibility for its partner programme is unknown [I6].
    - **Zoho Books:** the free plan allows 1,000 API calls a day [41].
    - *(M; not re-verified in this pass)*

## How this was checked

- Internal sources were read first and are not repeated here: [I1]–[I8].
- **First pass (2026-10-24):** direct fetches were blocked, so external claims came from search-result excerpts (SR).
- **Verification pass (2026-10-24, later the same day):** pages were opened directly where they would load. **Read directly, with evidence types updated:**
  - **Reach and email:** Pulse Asia's July 2026 media release (PDF) [42]; DataReportal Digital 2026 Philippines [43]; Philstar's GWI report [17]; Resend pricing [44].
  - **SMS rules:** Semaphore homepage, FAQ, API docs and advisories [5][45]; BSP memo M-2022-015 (PDF) [10]; Globe's 2023 statement as republished by The POST [8].
  - **Meta and Viber:** Meta's Messenger docs (utility messages, changelog, policy overview, marketing messages, m.me links) [46][47]; Viber's "Bot commercial model" (through the Zendesk API) [11]; Infobip Viber pricing [12]; Viber share-button docs [14].
  - **Payment providers:** PayMongo pricing page and docs (account setup, capabilities, wallets, linked accounts, policy contracts, payment links, webhooks, QR Ph API) [1][23][24][28][37]; HitPay OAuth, platforms and PH requirements docs [26][27]; Xendit docs and help (individuals, xenPlatform pricing policy) [21][25]; GCash help articles (through the Zendesk API) [32][33][34][53].
  - **Prefill links:** RFC 5724, Apple's SMS-links doc and Android's common-intents doc [48][49][50].
- **Still SR (not re-opened, or blocked):**
  - Xendit's PH fee table and the current ₱85 xenPlatform fee (xendit.co returned 403) [3][21].
  - HitPay and Maya fee figures [2][4][31]; the PH Viber per-message rate [13]; Vonage's sender-ID note [9].
  - WhatsApp FAQ text [19] and WhatsApp rate conflicts [20]; Infobip's Viber share [16].
  - The 2021 Pulse Asia figure [18]; QR Ph background [37 non-PayMongo parts]; InstaPay fee cuts [40]; statement-format pages [35][36]; Zoho [41].
- **Exchange rate:** ₱62/US$, as in [I4]. No EUR rate was sourced; the only peso figure for a Viber bot (~₱6,500 at the old EUR 100 fee) is from [I7].
- **Labels:** **Fact** (sourced), **Hypothesis** (plausible, untested), **Recommendation** (my proposal, not approved). Vendor statements about themselves are labelled vendor claims.

---

## Part 1. Channels to debtors

### 1.1 What Filipino debtors read

| Fact | Source | Type / conf. |
|---|---|---|
| 85% of Filipino adults are internet users; **37% of internet users send, receive or read email** (25% in March 2026); class ABC 49%, D 36%, E 22%; NCR 34%, rest of Luzon 52%, Visayas 24%, Mindanao 16%. Face-to-face, 2,400 adults, 28 Jun–6 Jul 2026, ±2% | [42] | TP (survey firm's own release, fetched) / H |
| 2021 Pulse Asia: 20% of internet users used email (older; superseded by the 2026 figure above) | [18] | N, SR / M (dated) |
| Messenger used in the past month by 90.6% of internet users aged 16+; Telegram 55.7%; WhatsApp 37.6% (GWI, Q2 2025). The reported list does not include Viber | [17] | N reporting GWI, fetched / H |
| Messenger ad reach 65.8M = 83.3% of adults and 67.1% of internet users (late 2025) | [43] | TP (DataReportal, fetched) / H |
| Viber "71% penetration", second to Messenger's 95% | [16] | VC (Infobip), SR / L–M |
| Infobip: PH business interactions on Viber +47% in 2025, still the leading business platform; WhatsApp +1,416% from a small base | [16] | VC, SR / L–M |
| GCash's operator warned about AI-made fake receipts in July 2025 | [39][I5] | VC/N, SR / H |

- **Hypothesis:** consumer and micro-business debtors (a sari-sari store, a tutorial parent, a customer buying on utang) live in Messenger, Viber and SMS. Business debtors paying a supplier's invoice read email. Pilots should record the channel each debtor actually answers on.
- **Gap:** no PH survey of SMS reading or open rates was found. No independent (non-vendor) Viber usage figure was found.

### 1.2 Email (engine-native)

- **Resend (verified on the pricing page [44]):**
  - **Free:** 3,000 emails a month and **100 a day**, 3 custom domains, 1 webhook endpoint, 30-day retention. Over-quota sending (pay-as-you-go) exists only on paid plans.
  - **Pro:** **US$20 a month** for 50,000 emails a month, no daily limit, 10 domains, 5 webhook endpoints.
  - Every email "sent or received through an inbox" counts toward the quota. So replies should keep arriving through Cloudflare Email Routing (₱0) [I4].
- From [I4], not redone: Brevo Free is the only ₱0, no-card overflow found (300 a day, but SPF does not align). A .com costs about ₱650–690 a year.
- **What follows for Balanse (inference, H):** the 100-a-day cap is shared by **all** Balanse customers, not each one. At about 5 reminders per customer per day, the free tier serves roughly 20 customers. Pro removes the daily cap.
- **Engine fit:** full. The outbox sends exactly once; Resend blocks duplicate idempotency keys; replies and bounces stop the loop; auto-replies are recognised [I8][I1].
- **Sender identity:** messages come from Balanse's domain with the creditor's name and a reply-to address. Sending from the creditor's own Gmail needs Google's `gmail.send` review, and reading replies there needs a restricted scope with a paid annual assessment [I6][I3]. Defer.

### 1.3 SMS

| Provider | Price per message | Notes | Source |
|---|---|---|---|
| **Semaphore** | **₱0.56 per credit**. FAQ: "inclusive of VAT"; homepage: "ex. VAT" (**conflict**, both seen today) | 160 bytes per credit; longer messages cost ceiling(length/153) credits. Priority and OTP routes cost 2 credits per 160 characters | [5] PP/VD fetched / H |
| PhilSMS | ₱0.35 (third-party comparison) | Not checked on the vendor page | [6] TP SR / L |
| UniSMS | "as low as ₱0.35" | Vendor listing claim | [6] VC SR / L |
| Twilio (PH) | US$0.241 per segment (≈ ₱15) | Alphanumeric sender at the same price | [I3] PP / H |
| M360, iTexMo, Movider | not found | — | — |

- **Sender names (Semaphore FAQ and advisories, fetched [5][45]):**
  - Up to 11 alphanumeric characters, approved in up to 5 business days with a real sample message.
  - Refused if "the account is not associated with a business". Election, gambling and similar content is banned, and "P2P Messages … are strictly not allowed".
  - The first name is free; extra names cost 200 credits a month.
  - Since 1 July 2024 the shared "Semaphore" sender can no longer be used, so **every account needs its own approved name**.
  - Vonage also says PH alphanumeric sender IDs must be pre-registered [9] (SR, M).
  - **Recommendation:** one Balanse sender name, with the creditor's name in the text.
- **Links:**
  - NTC memo of 12 Sept 2022, amended 27 Sept 2023. The amendment tells telcos "to block or deactivate text messages containing clickable domains, URLs, URL shortening services links, Smart Links, and QR Codes". Globe had already blocked links in P2P SMS since September 2022 [8] (N, fetched; memo text itself not read; H for what was reported).
  - **Semaphore advisory, 30 Sept 2022:** Globe's block covers P2P SMS. "Semaphore provides A2P … SMS and our service is not affected. You may continue sending SMS containing clickable links." **Advisory, 3 Oct 2022:** Smart blocks links from shortening services such as bit.ly, without failure reports, so "use full domains" [45] (VD fetched, H). Semaphore has posted no advisory about the 2023 amendment.
  - **Whether the 2023 amended memo exempts registered A2P senders: not stated** in the coverage read. The regulator's text was not found.
  - BSP memo M-2022-015 (22 Mar 2022) recommends "removal of clickable links in emails or SMS sent to retail customers", with a campaign telling customers banks no longer send links [10] (OD fetched, H).
  - **Design rule (recommendation, unchanged):** no links in v1 SMS. Give the amount, how to pay (the creditor's GCash name or bank details as text), and "reply to this message or contact [name]". A full-domain link through Semaphore appears technically possible. Leaving it out is about debtor trust and the unresolved 2023 memo.
- **Engine fit:**
  - Semaphore's send API documents no idempotency key; message status is one of queued, pending, sent, failed or refunded [5] (VD fetched, H).
  - Under the engine's rule, a timed-out SMS becomes "unconfirmed" and is never re-sent [I8].
  - Sender-name SMS are one-way. Inbound replies were not found in Semaphore's docs.
- **Cost per customer (hypothesis):** 40 debtors × 3 reminders × 2 credits × ₱0.56 = **₱134 a month**, about 9% of ₱1,500. Charge for SMS as metered credits or cap it per plan.

### 1.4 Viber

| Option | Cost | Facts | Source |
|---|---|---|---|
| Chatbot (bots created since 5 Feb 2024) | **EUR 115 per bot per month** (Viber's page, edited 2026-09-01; resellers still say EUR 100) + per delivered chatbot-initiated message (rate card not reachable; PH rate not found). Session messages within 24 h of a subscriber's message are unlimited and not billed | Apply through Rakuten Viber or a verified partner; manual review, then contract | [11] OD (vendor terms) fetched / H; [55] TP fetched |
| Viber Business Messages (sender-ID product, via resellers) | Minimum **EUR 115 per active sender/service ID per month** for destinations outside the listed EUR 175 countries; first calendar month free. Alibaba Cloud lists PH marketing messages at ≈ US$0.0096 with a US$197/month minimum | Sold only through partners; transactional templates need Viber approval | [12] PP (Infobip) fetched / H; [13] SR / L–M |
| Free Viber business account (launched Oct 2023, PH first market) | ₱0 | Business profile, catalogue, separate Business Inbox; broadcasts and paid tiers "planned". API not found | [15] VC/N, SR / M (status after 2023 not confirmed) |
| **Forward link** `viber://forward?text=<url-encoded>` | ₱0 | For "mobile website visitors"; the text "can be sent to any Viber contact selected by your user" (**the user picks the contact**); the user can edit before sending | [14] VD fetched / H; truncation/iOS reports, forums / L |

- **Inference:** Viber is probably the second channel for consumer debtors after Messenger. Every automated Viber route costs more each month (EUR 115+) than one Balanse customer pays (₱1,500). Copy-and-open is the only ₱0 route.

### 1.5 Facebook Messenger

Verified against Meta's docs on 2026-10-24 [46][47]; older details from [I3].

- **Who can be messaged:**
  - The standard window lasts 24 hours after a user action, or up to 7 days after a first message that came from a Click-to-Messenger ad.
  - Promotional content is allowed inside the window [46].
  - Utility messages are sent to a "Page-scoped ID". So the debtor must already have interacted with the creditor's **Page** [46].
- **Outside the window:**
  - The HUMAN_AGENT tag lets a person reply manually within a separate 7-day period [46]. It needs App Review [I3].
  - **Changelog, 27 Mar 2026:** requests using CONFIRMED_EVENT_UPDATE, ACCOUNT_UPDATE and POST_PURCHASE_UPDATE receive **error code 100 from 27 April 2026**. Meta points to Utility Templates or the Marketing Messages API instead [46] (OD, H).
  - **Utility messages:**
    - They cover "order or account status updates, and appointment or event reminders".
    - They need the `page_utility_messaging` permission and must not contain marketing.
    - Templates are approved "within seconds".
    - "Facebook Pages are not required to be linked to a business to send utility messages".
    - **The utility doc states no price and no country restriction.** Messenger utility pricing: **not found**.
  - **Marketing Messages API:** open only to tech providers that have passed App Review. It **can serve businesses in the Philippines**, goes only to opted-in subscribers, and "Meta charges the sending business directly for each marketing message" [46] (OD, H). A reminder to pay is not marketing, so this does not fit.
- **Serving other businesses' Pages:** needs Business Verification, App Review for Advanced Access and Tech Provider Access Verification. Plan for 2–6 weeks [I3].
- **Personal Messenger accounts have no API** [I7]. Most micro-creditors chat with debtors from a personal profile, not a Page (hypothesis).
- **Whether a payment reminder counts as an "account status update":** not found. Meta reviews each template, and the doc says prices and fees in a template count as "transactions" content [46].
- **`m.me` prefill (now verified):**
  - Meta documents `http://m.me/PAGE-NAME?text=…` for a customer opening a chat **with a Page**, plus a `ref` parameter.
  - Clicking such a link to an existing conversation "will reset the 24-hour standard messaging window" [47] (OD, H).
  - **Prefilled text for a link to a personal profile (owner → debtor) is not documented.** For copy-and-open, the Copy button is the reliable path.
  - *(Later idea, not v1: a "Message us on Messenger" `m.me` link to the creditor's Page in an email would let the debtor open a window.)*
- **Platform risk:** the government publicly discussed a Facebook block in August 2026 [I5]. Do not make Messenger the only channel.

### 1.6 WhatsApp

- **Usage:** 37.6% of PH internet users monthly (GWI) [17]. Infobip reports fast growth from a small base [16].
- **Cloud API facts from [I3] (not re-verified):**
  - Per-message pricing; PH is billed in "Rest of Asia Pacific".
  - Utility/authentication templates **US$0.0130**; marketing US$0.0968 (Gallabox's copy of the rate card, effective 1 Oct 2026). A third party lists **US$0.0113** utility and US$0.0842 marketing [20] (TP SR, L). **Conflict; Meta's own rate CSV not read.**
  - Service messages are chargeable from 1 Oct 2026 after 1,000 free per number per month (M).
  - Tech Provider chain: 10 onboardings a week until verified. Each customer must add a payment method.
- **Inference:** about ₱0.70–0.81 per template, more than an SMS credit. The customer pays Meta directly, which is an onboarding hurdle.
- **AI policy:** allowed. AI that words a creditor's own reminders is "ancillary", not the primary functionality [I3].
- **Copy and open:** `https://wa.me/<number>?text=<url-encoded>` opens **the debtor's chat** with the text filled in. Nothing is sent until the user taps send [19] (WhatsApp FAQ quoted by Bird, SR, M). This is the cleanest copy-and-open of the four apps because it chooses the recipient.

### 1.7 Copy and open: how it fits the engine

- **Flow (recommendation):**
  1. The owner approves a draft for channel X.
  2. Balanse shows the approved text with a **Copy** button and an Open button: wa.me, viber://forward, `sms:` to the debtor's number, or Messenger.
  3. The owner taps "Sent" (or "Didn't send").
  4. The owner later records "replied" or "paid".
- **`sms:` prefill (now verified):**
  - RFC 5724 defines `sms:+63…?body=…` [48] (OD, H).
  - Apple's (archived, older) SMS-links page says the URL "must not include any message text" [49] (VD, H for what it says, M for current iOS behaviour).
  - Android documents the body only as an `sms_body` intent extra for native apps, not as a URI query [50] (VD, H).
  - **Conclusion:** a prefilled SMS body from a web page is not officially supported on either platform. The `sms:` link should open the right number, and the Copy button carries the text.
- **What it costs:** ₱0 to Balanse. The owner sends from their own app and their own load or unlimited-text promo, so the debtor sees the owner's real number or profile (fewer scam fears; hypothesis).
- **What it gives up:**
  - **Exactly-once is not guaranteed.** The engine cannot see the send, so it records the owner's statement.
  - Replies don't reach Balanse.
  - Delivery is unknown.
- **Product rule:** every copy-and-open message must be confirmed by the owner before the next step is scheduled. A debtor's follow-up count advances only on that confirmation.
- **Engine scope (inference):** a "manual" channel is product state, not an outbox send. Whether it belongs in Loopwright (a second product needing it) is an owner decision.

### 1.8 Channel comparison

| Channel | Cost to Balanse per message | Monthly fixed | Setup | Reaches consumer debtors | Engine guarantees | Links |
|---|---|---|---|---|---|---|
| Email (Resend) | ₱0 up to 100/day shared; then Pro US$20/mo (50k/mo) | ₱0 (+domain ≈ ₱650/yr) | Domain + DNS | Low–medium (37% of internet users, 2026) | **Full** | Yes |
| SMS (Semaphore) | ₱0.56–1.12 | ₱0 | Sender name ≤ 5 business days; business account | High (hypothesis) | Send only; "unconfirmed" on timeout | Technically yes for A2P (full domains); **avoid in v1** |
| Copy-and-open (Viber/WhatsApp/Messenger/own SMS) | ₱0 | ₱0 | None | High | None (owner confirms) | Yes (app chat) |
| Messenger API | not found (utility) | ₱0 | Page + Meta reviews, 2–6 wks | Only debtors who messaged the Page | Possible | Yes |
| WhatsApp API | ≈ US$0.011–0.013 per template | ₱0 | Tech Provider chain; customer adds card | Medium (37.6%) | Possible | Yes |
| Viber bot / Business Messages | per message (PH rate not found / ≈ US$0.0096) | EUR 115+ per bot or sender | Partner contract | High | Possible | Yes |

### 1.9 Recommended channel mix for a ₱0 start *(Recommendation; unchanged by verification)*

1. **Email through the engine** for every debtor with an address. This is the default for B2B invoices.
2. **Copy-and-open** for Viber, WhatsApp (wa.me with number), Messenger and the owner's own SMS. This is the default for consumer and micro debtors.
3. **Semaphore SMS as a paid add-on**, with no links, once a customer pays. It is the only automated non-email channel that fits ₱1,500/month.
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

**Fees** (PayMongo and GCash verified; others SR):

| Method | PayMongo [1] (fetched) | Xendit [3] (SR) | HitPay [4] (SR) | GCash for Business [33] (fetched) | Maya Business [2] (SR) |
|---|---|---|---|---|---|
| QR Ph | **1.34%** (2.23% with instant settlement) | 1.4% (min ₱15) per third party; calculator "1.00% or ₱15": unclear | "1.0% or ₱20" in one post | **1.0%** | 1.5% |
| GCash | 2.23% | 2.3% | 2.3% | (QR Ph rate) | — |
| Maya | **1.79%** | 1.8% | not found | — | not found |
| GrabPay / ShopeePay | 1.96% / 1.70% | 2.0% / 2.0% | not found | — | — |
| Local cards | 3.125% + ₱13.39 (international 4.02% + ₱13.39) | 3.2% + ₱10 | 3% + ₱15 | 3.2% | 3.5% + ₱10 |
| Online banking | 0.71% or ₱13.39 | 1% or ₱15 (BDO ₱25) | not found | — | — |
| Setup / monthly fee | none / none; payment links at standard rates | none found | none (vendor claim) | none | not found |
| VAT | **rates exclude VAT; 12% VAT is added on the fee** | not found | not found | **rates include VAT** | — |

- Fees are paid by the creditor to their own gateway; Balanse touches no money.
- **Debtor side:** InstaPay transfers fell to ₱10 on GCash and Maya and to ₱0 on the BPI and BDO apps after a BSP pricing change in July 2026 [40] (N SR, M).
- **Inference:** a plain bank or e-wallet transfer costs the debtor ₱0–10 and the creditor nothing. A payment link costs the creditor 1–3%. Many micro-creditors will keep asking for direct transfers, so link-based signals will cover only part of the debts.

**Who can sign up:**

| Provider | Unregistered individual / freelancer | Sole proprietor | Source |
|---|---|---|---|
| PayMongo | **Yes, "Individual"**: ID + liveness check; TIN on the KYB form. QR Ph is activated automatically. "Individual (with website)" can also request online banking, Maya, GrabPay, ShopeePay. **GCash and cards need a registered business type.** Funds settle into a PayMongo Wallet. Default merchant wallet cap ₱25M; **the earlier "₱50k" cap was not found** | BIR 2303 + DTI certificate | [24] VD fetched / H |
| Xendit | **No**: "we [do] not accommodate Individual business". PH docs: "Individual applications are only allowed if they are XP [xenPlatform] sub-accounts" | Gov't ID, TIN, DTI, BIR 2303, online store link (SR) | [25] VD fetched / H; documents list SR / M |
| HitPay | No individual path found | DTI, BIR 2303, proof of business, business bank proof | [26] VD fetched / H |
| GCash for Business | **Starter Wallet: no business documents.** "Verified GCash user for at least 12 months", automated, usually ready within 10 minutes; ₱5M monthly wallet limit, 1 branch, 1 user (effective 1 Sept 2026) | Premium Wallet: DTI + BIR 2303 + 9-digit TIN | [32][53] VD fetched / H |
| Maya Business | not found | DTI for the Business Manager upload; BIR 2303 not mentioned | [31] VD SR / L–M |

**Webhooks:**

| Provider | Paid event | Verification | Source |
|---|---|---|---|
| PayMongo | `link.payment.paid` for Payment Links made through the API; dynamic QR Ph confirms through the Payment Intent webhook | Per-endpoint secret; signature in the `Paymongo-Signature` header | [28] VD fetched / H |
| Xendit | invoice paid/expired callback | `x-callback-token` static token + optional IP allowlist; retries, so handle duplicates | [29] VD SR / M |
| Maya | `PAYMENT_SUCCESS` / `PAYMENT_FAILED` for Checkout | IP allowlist suggested. **"Email Invoicing and Payment Links do not support webhooks"** | [30] VD SR / H |
| HitPay | Per-request `webhook` URL on each payment request, notified when the payment completes, even under OAuth | Signature validation (per HitPay webhook guide) | [27] VD fetched / H |
| GCash for Business | none found; the portal offers balance, transfers, history and CSV export only | — | [53] VD fetched / M (absence) |

**Creating links on the merchant's behalf without holding funds:**

| Mechanism | How it works | Funds | Pull toward money handling | Source |
|---|---|---|---|---|
| **HitPay OAuth "Connect your HitPay account"** (beta; "stable and available to all businesses") | New apps get `business:read` only; other scopes are requested and **reviewed by HitPay**. Scopes are granular: `payments:create`, `payments:read`, separate `payments:refund`, `payments:cancel`, plus commerce/customer families. Tokens last one year (refreshable); merchants can revoke from their dashboard. Commission needs a separate Platform Key | Payouts "go directly to each sub-merchant's own bank account — funds are not pooled through the platform"; HitPay holds the BSP licence and does KYB | **Low** if Balanse requests only create/read scopes and no Platform Key or commission | [27] VD fetched / H |
| PayMongo account linking | Balanse (an activated PayMongo account) invites a new or existing account by signup link; the child accepts. Access is chosen per module (`none` / `related` / `global` for view and manage), e.g. **view payments only**, no refunds or payouts. "Full access" needs a signed mandate. Policy contracts governing each link **ship Q4 2026**; the docs point to Part IV of PayMongo's Terms for rights and liability | Child keeps its own account and wallet; parent's payout access is a separate permission | **Medium → low** with view-only scope; still a contractual relationship over others' accounts: ask the legal note | [23] VD fetched / H |
| Xendit xenPlatform | Master account accepts payments on behalf of sub-accounts, splits fees, pays out. **From 1 Feb 2027:** USD 10,000 one-time setup, **USD 7,500/month platform fee**, USD 5 per active sub-account, USD 15 per KYC/KYB attempt (waived for existing merchants who migrate in time). Current fee (₱85 per active sub-account) is SR | Platform-orchestrated | **High**: avoid | [21] VD fetched / H; ₱85 SR / M |
| Creditor pastes their own secret API key | Balanse calls the gateway as the merchant | Merchant's | Low on funds, but **high security risk**: HitPay calls a merchant API key "full account access" | [27]; inference |
| **Creditor pastes their own payment link URL** | Balanse just puts it in the email | Merchant's | **None**. Gives no signal; the owner still marks paid | inference |

### 2.3 QR Ph

- **Facts:**
  - QR Ph is the national standard (BSP Circular 1055, 2019). Proprietary payment QRs had to be switched off from 1 July 2023 [37] (SR, M).
  - **Static** QR: printed, never changes, the payer types the amount [37] (SR, M).
  - **Dynamic** QR Ph (PayMongo): "generated per transaction", "single-use, encodes the exact payment amount, and expires after 30 minutes by default"; confirmed by webhook or polling [37] (VD fetched, H).
  - GCash's personal "Receive via QR" code does not expire and hides the number [38] (N/VD SR, L–M).
  - One explainer says that from 29 July 2026 "QR Ph" means merchant (P2M) codes and P2P moves to "InstaPay QR". Single source, not confirmed with BSP [37] (L).
- **Notification:**
  - A personal static QR notifies only the account holder, in their own app.
  - No API, webhook or delegated read access for personal GCash or Maya accounts was found. GCash for Business (even Starter) lists no API either [53].
  - **Conclusion: a third party cannot learn of a payment to a personal QR.** This rests on absence of evidence (M).
- **Notable paths (inference):**
  - PayMongo's "Individual" tier takes QR Ph at 1.34% with a webhook. It is the cheapest gateway route for creditors without DTI.
  - GCash for Business Starter needs no DTI either, but gives Balanse no signal beyond a CSV the owner would export.

### 2.4 Statement import

| Source | Export | Notes | Source |
|---|---|---|---|
| GCash (personal) | "Request transaction history": email "Your GCash Transaction History" with an attached **PDF**, up to 4 years; password = last name in lowercase + last 4 mobile digits | App shows 90 days, loaded 15 at a time | [34] VD fetched / H |
| GCash for Business | **CSV** ("Export" / "Download CSV"): reference number, date/time, status, gross, fees, net, product, source/destination | Requires a verified business account | [34] VD fetched / H |
| Maya Business | "Export Transactions" from Business Deposit; monthly statement PDF | Personal Maya: not found | [35] VD SR / M |
| BDO Business Online | csv / xls / pdf, up to 90 days | From an old (≈2018) guide uploaded to Scribd | [36] TP SR / L |
| BPI, BDO personal | PDF statements (third-party converter sites); official CSV: **not found** | — | [36] TP SR / L |

- **Matching (inference):**
  - A P2P transfer carries the sender's name (often masked), the amount, a time and the sender app's reference number. None of these is Balanse's debt ID.
  - Matching must use amount + date window + name fuzziness, and would produce false matches when two debtors owe the same round amount.
  - So it can only **suggest** matches for the owner to confirm.
- **Privacy (inference):** a statement exposes all of the creditor's income and spending, including personal items, plus the names of people who are not debtors. If built, parse in memory, keep only the matched rows, and delete the file (the engine already supports deletion deadlines [I8]). Data Privacy Act duties go to the legal note.
- **Parsing:** each bank and app format is unofficial and can change without notice. GCash's personal PDF is password-protected.

### 2.5 Accounting software paid-status sync

Not re-verified in this pass.

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
| HitPay OAuth webhook | ₱0 (creditor pays 1–3%) | HitPay account (DTI) + Connect click | High (gateway event) | Low–medium (Balanse holds scoped tokens) | Low with create/read scopes only |
| PayMongo account linking webhook | ₱0; Balanse needs its own activated account | PayMongo account (Individual OK) + accept link | High | Low–medium | **Low–medium** (view-only scope possible; contract) |
| PayMongo "Individual" QR Ph (creditor's key or link) | ₱0 | ID + liveness; QR Ph | High | Medium if Balanse holds a full key | Low on funds; high key risk |
| Xendit xenPlatform | US$7,500/mo from Feb 2027 | Sub-account KYB | High | Medium | **High**: avoid |
| GCash for Business Starter (CSV) | ₱0 + parser | Verified GCash ≥ 12 months; export and upload | Medium (business wallet only) | Medium–high | None |
| Static personal GCash/Maya QR | ₱0 | None | **No signal available** | — | None |
| Bank/e-wallet statement import | ₱0 + parser upkeep | Export, unlock, upload each time | Low–medium (ambiguous matches) | **High** (whole financial life) | None, but looks like financial-data processing |
| Xero/QBO/Zoho status sync | Free tiers; Xero AUD 35+/mo past 5 connections | OAuth connect | High where invoices live there | Medium | None |

---

## 4. Recommendation for v1 and what to defer *(Recommendation, not approved)*

**v1 channels (unchanged)**
- **Email through the engine** (Resend Free, Cloudflare inbound). Move to Resend Pro (US$20/mo, no daily cap) once all customers together pass about 100 emails a day.
- **Copy-and-open** for WhatsApp (wa.me with number and text), Viber (forward link; owner picks contact), Messenger and the owner's own SMS (`sms:` with number; text via the Copy button). Confirmed by the owner; follow-ups advance only on that confirmation.
- No links in SMS-shaped text. Show payment instructions as plain text: the creditor's GCash or bank name and number, entered by the owner.

**v1 payment signals (unchanged)**
1. Owner marks paid. This is the source of truth.
2. A debtor's email reply stops the loop. The model may label it "claims paid" for the owner to review.
3. Proof upload through a signed link, shown to the owner as "payment claimed – check your app". Never auto-paid.
4. Optional: the owner pastes their own payment-link URL (PayMongo, HitPay, GCash for Business, Maya or other) into the message. No integration.

**Defer, with the trigger to revisit (order adjusted after verification)**
- **Semaphore SMS:** when the first customer pays. Needs Balanse's own approved sender name (a business account). Metered; about ₱134/month at the volume assumed above.
- **HitPay OAuth webhook (first integration candidate):** when several pilot creditors already use HitPay. Request only `payments:create` and `payments:read`; no Platform Key. Ask the legal note first.
- **PayMongo account linking (second candidate):** after policy contracts ship (Q4 2026) and the legal note reviews the parent/child terms. Use view-only payment scope. It reaches unregistered creditors, because PayMongo Individuals can be linked.
- **Messenger utility templates:** if many debtors first wrote to a creditor's Page. Needs App Review; price not published.
- **WhatsApp Cloud API:** only if pilots show WhatsApp-heavy debtors and creditors willing to add a card for Meta.
- **Statement import (GCash for Business CSV first, the only documented machine-readable export) and accounting sync:** after pilots.
- **Never (for this design):** Xendit xenPlatform, collecting payments into a Balanse account, taking a commission on payments, and storing creditors' full-power secret keys.

## 5. Biggest unknowns

1. Which channel PH debtors actually answer on: no survey found. Pilots should log it per debtor.
2. Whether the NTC's 2023 amended memo applies to registered A2P senders. Semaphore's 2022 advisory says A2P links still work, but the regulator's text was not found.
3. Messenger utility-message pricing, and whether a payment reminder passes Meta's template review as an "account status update": not found.
4. PayMongo policy contracts (Q4 2026) and Terms Part IV: what a parent may do in practice; whether HitPay approves narrow scopes for a reminder tool.
5. Viber's chatbot-initiated message rate for PH (rate card not reachable).
6. Xendit and HitPay PH fee tables (still SR); Semaphore VAT treatment (its own pages conflict).
7. Official CSV exports for personal BPI, BDO and Maya: not found.
8. Real behaviour of `sms:` body prefill on current iOS and Android from a web page: not officially documented. Test on devices if it matters.

---

## Sources

Accessed 2026-10-24. Types: OD official/regulator, PP vendor pricing page, VD vendor docs, VC vendor claim, N news, TP third party, SR search-result excerpt only (page not fetched). "Fetched" means the page itself was read in the verification pass. "n.d." = no date shown.

**Internal (not redone)**
- [I1] Kompleto, `docs/research/inbound-email.md`, 2026-10-21. Internal research (VD fetched from GitHub sources). H.
- [I2] Kompleto, `docs/research/hosting-and-email-infra.md`, 2026-10-21. Internal. H.
- [I3] Kompleto, `docs/research/integration-constraints.md`, accessed 2026-10-18 (Meta, Viber, Semaphore, Twilio, PayMongo, Xendit sections). Internal; OD/PP H where fetched, M where excerpt.
- [I4] Kompleto, `docs/research/domain-and-email-at-zero.md`, 2026-10-21 (Resend, Brevo, Cloudflare, domain cost, ₱62/US$). Internal. H/M.
- [I5] Kompleto, `docs/research/market-philippines.md` (platform reach, payments, Facebook-block risk). Internal. H/M.
- [I6] Kompleto, `docs/research/self-serve-distribution.md` (Xero tiers, Intuit App Partner Program, Gmail rules). Internal. H/M.
- [I7] Kompleto, `docs/research/ph-industries-sectors.md` (Viber bot ≈ ₱6,500/month at EUR 100, personal chats have no API). Internal. M.
- [I8] Loopwright, `README.md` (outbox, exactly-once, inbound, files, scan, signed links). Internal. H.

**External**
1. PayMongo, "Pricing", https://www.paymongo.com/en/pricing (redirect from /pricing). n.d. PP, fetched, H. ("All prices are exclusive of VAT"; QR Ph 1.34%, GCash 2.23%, Maya 1.79%, GrabPay 1.96%, ShopeePay 1.70%, cards 3.125% + ₱13.39, banking 0.71% or ₱13.39; setup fee free.)
2. Maya Business blog figures (cards 3.5% + ₱10, QR Ph 1.5% MDR) as surfaced in a search for PayMongo pricing; Maya's own pricing page not found. n.d. VC, SR, L.
3. Xendit, "Pricing (Philippines)" https://www.xendit.co/en-ph/pricing/ (403 on fetch) and "Pricing calculator PH" https://www.xendit.co/en-ph/pricing-calculator-ph/; Prosperna, "PayMongo vs Xendit 2025" https://www.prosperna.com/blog/paymongo-vs-xendit-2025-fees-and-features-for-ph-smes. n.d./2025. PP SR M; TP SR L.
4. HitPay, "HitPay rates" https://hitpayapp.com/blog/hitpay-rates; "Payment gateway Philippines payout comparison" https://hitpayapp.com/blog/payment-gateway-philippines-payout-comparison; "HitPay vs PayMongo fees in 2025" https://hitpayapp.com/blog/hitpay-vs-paymongo-fees-in-2025-which-payment-gateway-saves-philippine-smes-more. 2025. VC, SR, L.
5. Semaphore, homepage https://semaphore.co/ ("Php 0.56/text (ex. VAT)"); FAQ https://semaphore.co/faq ("Each credit is P0.56 inclusive of VAT", sender-name rules, segmenting); API docs https://semaphore.co/docs (priority/OTP 2 credits, statuses). n.d. PP/VD, fetched, H.
6. Sent.dm, "Philippines SMS pricing", https://sent.dm/en/resources/sms-pricing/philippines-sms-pricing, 2026-03; UniSMS listing https://peerpush.com/p/unisms-api. TP/VC, SR, L.
8. The POST, "Globe lauds NTC memo requiring telcos to block SMS with clickable links", 2023-10-18, https://thepost.net.ph/society/globe-lauds-ntc-memo-requiring-telcos-to-block-sms-with-clickable-links/ (republished Globe statement; Globe's own page returned 403). N/company statement, fetched, H for what it reports. Also CNN Philippines, 2022-09-13, https://www.cnnphilippines.com/business/2022/9/13/NTC-order-deactivation-links-texts-malicious-sites.html (SR, M). NTC memo text: not found.
9. Vonage, "Philippines SMS Features and Restrictions", https://api.support.vonage.com/hc/en-us/articles/204017413. n.d. VD, SR, M.
10. Bangko Sentral ng Pilipinas, Memorandum M-2022-015, "Recommended Control Measures Against Cyber Fraud and Attacks on Retail Electronic Payments and Financial Services (EPFS)", 2022-03-22, https://www.bsp.gov.ph/Regulations/Issuances/2022/M-2022-015.pdf. OD, fetched, H.
11. Rakuten Viber Help Center, "Bot commercial model", https://help.viber.com/hc/en-us/articles/15247629658525 (read through the help centre's public Zendesk API; edited 2026-09-01). OD (vendor terms), fetched, H. ("maintenance fee of EUR 115 per bot"; sessions 24 h, unbilled.)
12. Infobip, "Viber pricing", https://www.infobip.com/viber-business/pricing. n.d. PP (reseller), fetched, H. (Minimum "115 € per active sender/service ID for all other destinations"; first calendar month free; PH per-message price not shown in the static page.)
13. Alibaba Cloud, "Chat App Message Service – Viber", https://www.alibabacloud.com/help/en/chatapp/product-overview/viber. n.d. PP (reseller), SR, L–M.
14. Rakuten Viber Developers, "Share button", https://developers.viber.com/docs/tools/share-button/. n.d. VD, fetched, H. Issue reports: Habr Q&A https://qna.habr.com/q/979085 (forum, SR, L).
15. Rakuten Viber, "Rakuten Viber debuts … toolkit for all businesses", https://www.forbusiness.viber.com/en/blog/post/rakuten-viber-debuts-giant-leap-forward-in-global-strategy-with-its-toolkit-for-all-businesses; NewsBytes 2023-10-11 https://newsbytes.ph/2023/10/11/ph-is-first-country-to-roll-out-vibers-new-toolkit-for-smes/. 2023. VC/N, SR, M.
16. NewsBytes, "Report: WhatsApp posts rapid growth in PH business messaging but Viber stays dominant", 2026-06-16, https://newsbytes.ph/2026/06/16/report-whatsapp-posts-rapid-growth-in-ph-business-messaging-but-viber-stays-dominant/; Infobip, "Most popular messaging apps by country", https://www.infobip.com/blog/most-popular-messaging-apps-by-country. 2026. VC (Infobip data), SR, L–M.
17. Philstar, "Pinoys top Facebook, YouTube users globally", 2025-10-16, https://www.philstar.com/headlines/2025/10/16/2480149/pinoys-top-facebook-youtube-users-globally (reports Digital 2026 / GWI Q2 2025, internet users 16+). N, fetched, H.
18. Inquirer, "63 percent of Filipino adults are internet users — Pulse Asia survey", 2021, https://newsinfo.inquirer.net/?p=1500767 (403 on fetch). N reporting survey, SR, M (dated; superseded by [42]).
19. Bird, "What is a WhatsApp click to chat link" (quotes WhatsApp FAQ), https://bird.com/explained/whatsapp/what-is-a-whatsapp-click-to-chat-link.md. n.d. TP quoting VD, SR, M.
20. AgentDeals, "WhatsApp Business API pricing 2026", https://agentdeals.dev/whatsapp-business-api-pricing-2026. 2026. TP, SR, L.
21. Xendit Help Center, "XenPlatform Pricing Policy", https://help.xendit.co/hc/en-us/articles/62671958518297 (via Zendesk API; edited 2026-10-01), VD/PP, fetched, H. Xendit Docs, "XenPlatform overview", https://docs.xendit.co/docs/xenplatform-overview.md, VD, fetched, H. Current ₱85/active sub-account fee: Xendit help/docs excerpts, SR, M.
22. Prosperna, "Connect Xendit (PH merchants)", https://help.prosperna.com/payments/connect-xendit-ph-merchants. n.d. TP, SR, L.
23. PayMongo Docs, "Linked accounts" https://docs.paymongo.com/docs/account-settings-linked-accounts.md and "Policy contracts" https://docs.paymongo.com/docs/account-settings-policy-contracts.md. n.d. (policy contracts "ship in Q4 2026"). VD, fetched, H.
24. PayMongo Docs, "Account setup" https://docs.paymongo.com/docs/account-settings-account-setup.md; "Account capabilities" https://docs.paymongo.com/docs/account-settings-account-capabilities.md; "Wallets" https://docs.paymongo.com/docs/money-movement-wallets.md; "Key concepts" https://docs.paymongo.com/docs/account-settings-key-concepts.md (updated 2026-08-27). VD, fetched, H.
25. Xendit Help Center, "Can Individual businesses use Xendit's services?", https://help.xendit.co/hc/en-us/articles/360035083911 (via Zendesk API; edited 2026-03-06); Xendit Docs, "Philippines" business documents, https://docs.xendit.co/docs/philippines-business-documents.md (2026-07-07). VD, fetched, H. Document list from "[PH] Legal documents required" https://help.xendit.co/hc/en-us/articles/11103239008793 (SR, M).
26. HitPay Docs, "Onboarding requirements (PH)", https://docs.hitpayapp.com/onboarding/requirements-ph.md. n.d. VD, fetched, H.
27. HitPay Docs, "OAuth" https://docs.hitpayapp.com/platforms/oauth.md and "Platforms overview" https://docs.hitpayapp.com/platforms/overview.md. n.d. VD, fetched, H.
28. PayMongo Docs, "Payment Links" https://docs.paymongo.com/docs/payment-channels-payment-links.md; "Webhooks – key concepts" https://docs.paymongo.com/docs/developer-tools-webhooks-key-concepts.md. n.d. VD, fetched, H.
29. Xendit, "Webhook – getting started" https://docs.xendit.co/webhook/getting-started; Help Center "About webhook" https://help.xendit.co/hc/en-us/articles/360038048871; xendit-node `Invoice.md` https://github.com/xendit/xendit-node/blob/master/docs/Invoice.md. n.d. VD, SR, M.
30. Maya Developers, "Managing webhooks" https://developers.maya.ph/docs/receive-real-time-payment-information-using-webhooks; "Configuring your webhook for Maya Checkout" https://developers.maya.ph/reference/configuring-your-webhook-for-maya-checkout. n.d. VD, SR, H.
31. Maya, "How to open a business deposit or savings account", https://www.maya.ph/business/stories/how-to-open-a-business-deposit-or-savings-account (VD, SR, L–M); Sacra, "Maya payments to lending funnel" (1.5–3.5%) (TP, SR, L).
32. GCash Help Center, "What are the requirements to create a GCash for Business account as a merchant?", https://help.gcash.com/hc/en-us/articles/48456974006041 (via Zendesk API; edited 2026-10-23). VD, fetched, H.
33. GCash Help Center, "What is the GCash for Business Merchant Discount Rate (MDR)?", https://help.gcash.com/hc/en-us/articles/55684605325593 (via Zendesk API; edited 2026-03-04). VD, fetched, H.
34. GCash Help Center, "How to request transaction history", https://help.gcash.com/hc/en-us/articles/360034155433 (edited 2026-09-02); "How do I review and download my GCash for Business Transaction History?", https://help.gcash.com/hc/en-us/articles/48457463083545 (edited 2025-07-11). Both via Zendesk API. VD, fetched, H.
35. Maya Developers, "Receive settlements into your Business Deposit", https://developers.maya.ph/docs/receive-settlements-into-your-business-deposit. n.d. VD, SR, M.
36. Scribd upload, "BDO Business Online Banking guide", https://www.scribd.com/document/489780423; Bankstatemently, BPI and BDO pages, https://bankstatemently.com/banks/ph/bpi/bank-statement. n.d. TP, SR, L.
37. PayMongo Docs, "QR Ph API", https://docs.paymongo.com/docs/payment-acceptance-qr-ph-api.md (VD, fetched, H); Skyro Help Center, "What is QR Ph" https://www.skyro.ph/helpcenter/qr-ph-usage/what-is-qr-ph (TP SR M); Philstar, "National QR code rolls out July", 2023-03-03, https://www.philstar.com/business/2023/03/03/2248821/national-qr-code-rolls-out-july (N SR M); TransFi, "QR Ph explained" https://www.transfi.com/blog/qr-ph-explained-how-p2p-and-p2m-qr-payments-work-for-individuals-and-merchants (TP SR L).
38. The Paypers, "GCash launches QR code-based money transfer feature", https://thepaypers.com/payments/news/gcash-launches-qr-code-based-money-transfer-feature. n.d. N, SR, L–M.
39. Mynt (GCash), "GCash warns public on emerging AI-generated fake receipts scams", 2025-07-14, https://mynt.com.ph/newsroom/gcash-warns-public-on-emerging-ai-generated-fake-receipts-scams-urges-users-to-always-check-transactions-tab (VC/official company notice, SR, H); Unbox PH, https://unbox.ph/ai/chatgpt-receipts/ (N, SR, M).
40. Philstar, "LIST: Banks, e-wallets cut, waived InstaPay, PESONet transfer fees", 2026-07-07, https://www.philstar.com/headlines/2026/07/07/2540294/list-banks-e-wallets-cut-waived-instapay-pesonet-transfer-fees; Manila Bulletin, "GCash slashes bank transfer fee to ₱10", 2026-07-05, https://mb.com.ph/2026/07/05/gcash-slashes-bank-transfer-fee-to-10. N, SR, M.
41. Zoho, "Zoho Books API v4 – introduction", https://www.zoho.com/books/api/v4/introduction/; KB "Per day API limit", https://www.zoho.com/books/kb/automation/per-day-api-limit.html. n.d. VD, SR, H.
42. Pulse Asia Research, "July 2026 Ulat ng Bayan Survey: Media Release on Internet and Social Media Use", 2026-07-27, https://pulseasia.ph/wp-content/uploads/2026/07/MR4-UB2026-2-MR-on-Internet-and-Social-Media-Use-Final.pdf (Table 3), linked from https://pulseasia.ph/updates/july-2026-nationwide-survey-on-internet-and-social-media-use/. TP (survey firm's primary release), fetched, H.
43. DataReportal (Kepios / We Are Social / Meltwater), "Digital 2026: The Philippines", 2025-11-05, https://datareportal.com/reports/digital-2026-philippines. TP, fetched, H (Meta ad-reach figures; publisher warns of platform revisions).
44. Resend, "Pricing", https://resend.com/pricing. n.d. PP, fetched, H.
45. Semaphore, "Advisories", https://semaphore.co/advisories (entries 2022-09-30, 2022-10-03, 2024-06-15). VD, fetched, H.
46. Meta for Developers, Messenger Platform: "Send a utility message" https://developers.facebook.com/documentation/business-messaging/messenger-platform/send-messages/utility-messages.md; "Changelog" (entry 2026-03-27) https://developers.facebook.com/documentation/business-messaging/messenger-platform/changelog.md; "Policy overview" https://developers.facebook.com/documentation/business-messaging/messenger-platform/policy.md; "Marketing Message API for Messenger" https://developers.facebook.com/documentation/business-messaging/messenger-platform/marketing-messages-on-messenger.md. n.d. OD, fetched, H.
47. Meta for Developers, "m.me Links", https://developers.facebook.com/documentation/business-messaging/messenger-platform/discovery/m-me-links.md. n.d. OD, fetched, H.
48. IETF, RFC 5724, "URI Scheme for Global System for Mobile Communications (GSM) Short Message Service (SMS)", January 2010, https://www.rfc-editor.org/rfc/rfc5724.txt. OD (standard), fetched, H.
49. Apple, "SMS Links" (Apple URL Scheme Reference, archived documentation), https://developer.apple.com/library/archive/featuredarticles/iPhoneURLScheme_Reference/SMSLinks/SMSLinks.html. n.d. (archived, older). VD, fetched, H for its text; M for current iOS.
50. Android Developers, "Common intents" (Text messaging: `sms_body` extra), https://developer.android.com/guide/components/intents-common. n.d. VD, fetched, H.
53. GCash Help Center, "What are the GCash for Business wallet tiers and transaction limits?" https://help.gcash.com/hc/en-us/articles/48457224157081 (edited 2026-10-15); "How to apply for GCash for Business?" https://help.gcash.com/hc/en-us/articles/63489589168025; "What are the available features of GCash for Business?" https://help.gcash.com/hc/en-us/articles/48456930314649 (edited 2026-10-09). All via Zendesk API. VD, fetched, H.
55. respond.io, "Viber overview" https://respond.io/help/viber/viber-overview; SleekFlow, "Viber channel overview" https://help.sleekflow.io/en_US/viber-channel-overview. n.d. TP (resellers), fetched, M (they still state EUR 100, older than [11]).
