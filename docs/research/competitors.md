# Balanse: competitors and substitutes

**Status: research, 2026-10-24. Draft for the owner; nothing here is a decision.** It covers who else helps a Philippine business get paid what it is owed, what each one costs, and where an approve-first, Taglish payment follow-up tool could fit.

**Builds on (not redone):** Kompleto's research in [Kompleto `docs/research`](https://github.com/Enriquez-Johnmark/kompleto/tree/main/docs/research): `self-serve-distribution.md` [I1] (Xero App Store collections apps, Xero/QBO tiers, Xero PH Lite), `follow-up-competitors.md` [I2] (PH Taglish AI chat tools such as Asikaso), `competitor-analysis.md` [I3], `ai-addons-erp.md` [I4] (Xero JAX, Intuit and Odoo AI), `opportunities-business.md` §3.6 [I5] (AR collections evidence) and `new-ideas-philippines.md` [I6] (PayMongo fees, Taxumo/Juan prices, fake GCash receipts).

**Method limits (read first):**
- **No vendor page could be opened in this session.** WebFetch and curl were refused for every host tried (DNS failure or a 403 from the egress proxy). So **every external fact below comes from search-engine result excerpts (evidence type SR)**, and none is rated above M. Facts carried over from the internal files keep their own ratings, some of which are H because those pages were read directly on 2026-10-18.
- The session's shared web-search budget ran out before these were checked: GCash "Request Money", Maya invoice reminders, Google Play install counts for PH utang apps, Zoho Books' PH price in pesos, and whether Mochi's reminders read replies. These are marked "not found" or "not verified".
- **Currency:** US$ prices are converted at about ₱61/US$ (the BSP reference rate was ₱61.35 on 1 July 2026 [52]). No October 2026 rate was found, so treat conversions as ±5%. GBP prices are not converted.

Labels: **Fact** (with a source), **Hypothesis** (my inference, untested), **Recommendation** (for the owner to decide). Evidence types and confidence for each source are in the Sources list.

---

## Key conclusions

1. **Automatic payment reminders are free or bundled almost everywhere a PH business might already be.** PayMongo Invoices (beta) emails a customer 4 times and stops when the invoice is paid [30]. Xendit payment links send email/WhatsApp reminders [27]. QuickBooks PH advertises automatic reminders [20], Zoho Books' free plan includes them [21], and Odoo has follow-up levels with email/SMS [25]. **"We send reminders" is not a product; it is a checkbox.** *(Fact, M)*
2. **A PH-made AR tool already sits at Balanse's price.** **Mochi** (Makati; pre-seed backers include Kaya Founders and Foxmont) sells digital invoices, payment links, automated reminders and collections reports. A third-party profile puts its standard plan at **₱1,899/mo**, and it partners with PayMongo [35]. **First Circle**, an SME lender, advertises a **free** invoicing tool with "unlimited, automatic" reminders, behind a waitlist [36]. *(Fact, M for features; L for Mochi's price)*
3. **The approve-before-send mechanic exists, but only in global, email-first, Xero-based tools:** Solido (AI drafts per invoice, approved before sending), Trove (review or auto), Upflow (suggestions-only or auto per skill) and Kolleno (human override) [15][16][2][5]. **None of them is PH-local, writes in Taglish, or uses Viber, Messenger or SMS in the PH sense.** *(Fact, M)*
4. **The global tools cost 2–10× Balanse and assume Xero/QBO.** Paidnice starts at US$69 (≈₱4,200), Chaser at US$259 (≈₱15,800), and Trove at GBP 50–265. Upflow, Gaviti, Kolleno and Tesorio sell on quotes to mid-market firms [I1][13][16][1][4][6][7]. Only 11% of Filipino MSMEs use digital accounting (a single vendor-cited figure) [I1]. **The global AR tools are not where PH small businesses will look.** *(Fact for prices, M; hypothesis for buyer behaviour)*
5. **Xero's JAX is the biggest platform move.** Xerocon 2026 coverage describes AI payment follow-ups that build a plan per customer, consolidate invoices and switch to SMS when email gets no response [17]. Intuit's AI reminders appear US-only [19]. **Neither matters much in PH** at 11% adoption and with Xero PH Lite capped at 5 invoices [I1]. They do matter if Balanse targets bookkeepers whose clients use Xero. *(Fact, M; implication is hypothesis)*
6. **PH "utang" apps are consumer or sari-sari ledgers, not B2B collections.** Lista PH sends a free automated SMS of the balance when an utang is recorded [39]. Bangon, Utang Free PH, UtangIna and SukliPH are tiny or pre-launch [40]. **No install counts were found.** India's Khatabook/OkCredit and Indonesia's BukuWarung/Paper.id show the free-ledger-plus-WhatsApp-reminder pattern at scale, but **none was found operating in PH** [41][42]. *(Fact, L–M)*
7. **The real competitor is doing it by hand on Viber and Messenger, for ₱0.** Viber claims over 80% PH penetration [47]. Since April 2026, Messenger's ACCOUNT_UPDATE and POST_PURCHASE_UPDATE tags return errors, so automated Messenger messages outside the 24-hour window need Meta-approved utility templates or paid marketing messages [49]. Reseller prices for Viber Business Messages carry €115–US$500 monthly minimums [47]. **A ₱1,500 product cannot afford an official Viber/Messenger send path per customer.** Email, SMS at ≈₱0.56 [48], and the owner sending a drafted message from their own phone are the affordable routes. *(Fact for rules and prices, M; implication is hypothesis)*
8. **The escalation substitutes are expensive and slow, which helps Balanse's price story.** A lawyer's demand letter costs ≈₱3,000–30,000 [44]. Commercial collection agencies charge an indicative 5–15% contingency on B2B debts, rising with age [43]. Small claims (up to ₱1M, no lawyers) is free of lawyers but not of time [45]. **₱1,500/mo is cheap next to one demand letter, but expensive next to free reminders.** *(Fact, L–M; all from law-firm blogs)*
9. **Most dangerous competitor: Mochi** (reasoning in §6): same buyer, same country, same price band, payment links already in place, and VC-funded. Adding AI-drafted Taglish messages and an approval step is a feature for them, not a rebuild. *(Judgement, M)*
10. **Where Balanse can stand (hypothesis):** businesses with no accounting software, chasing B2B or B2C debts that are **relationship-sensitive** ("nakakahiya maningil"). For them, the value is **what to say, said politely in Taglish, approved by the owner, sent once, then stopped the moment the debtor replies.** That differs from "send the invoice email again on day +3". It must be proven in pilots; nothing found shows demand for it.

---

## 1. Global AR-collection tools

Prices are "from" prices. Most of these tools need Xero, QBO or an ERP. **PH availability** means whether a PH business could buy and use it. None blocks PH explicitly, but all are priced and built for AU/UK/US/EU.

| Tool | What it does | Price (from) | Approve before send? | Channels | Stops on reply / payment | Payment link | Src |
|---|---|---|---|---|---|---|---|
| **Chaser** (UK) | Credit control on Xero/QBO; AI email generator, risk insights | US$259/mo Compact (≈₱15,800); US$779 Core; US$1,169 Complete (aggregators; help page shows older £40–400 plans) | Not documented | Email, SMS, calls | Payment sync yes; reply handling not documented | Yes (portal) | [I1][13] |
| **Paidnice** (NZ) | "Credit control AI", late fees and interest, payment plans, statements | US$69/mo (≈₱4,200) | Drafts replies to debtors; reminders automatic | Email from own domain (SMS per directories) | Records payment promises and **holds reminders until the promised date**; routes disputes to the owner (vendor claims) | Yes | [I1][14] |
| **Solido** (2026) | AI drafts each reminder against one invoice; read-only in Xero | Tiers by email volume (50/150/300 a month); US$ price not shown | **Yes, every reminder waits for approval** | Own Gmail/M365 | **Stops when payment syncs from Xero** | Xero invoice attached | [I1][15] |
| **Trove** (UK, 2025) | Credit control AI; suggested replies; updates systems as debtors reply | GBP 50 (Capterra); GBP 135 Integrated; GBP 265 Advanced | **Review all before send, or auto** | Own Gmail/Outlook | Updates on replies (vendor) | Not found | [I1][16] |
| **Upflow** (FR) | Mid-market AR; AI collection agent with Replies, Promise-to-pay and Disputes skills | Free analytics tier; paid tiers on quote; Payments add-on US$390/mo; Vendr avg contract US$15,000/yr | **Per skill: off, suggestions-only, or auto (falls back to suggestions when unsure)** | Email, portal | Promise-to-pay handling | Yes | [1][2] |
| **Kolleno** (UK) | AR platform, AI agents for timing, tone and channel, cash application | £545–650 per user/mo (aggregators, conflicting); AI agents from the >£10M-turnover tier | "Review, adjust or override" agent actions (vendor FAQ) | Email, SMS, phone, portal | Not documented | Yes (portal) | [4][5] |
| **InvoiceSherpa** (US) | Automated reminders for QBO/Xero | US$49 / 99 / 199 per mo, plus 1% fee on invoices paid | Not documented | Email, SMS (not verified) | Payment yes | Yes | [3] |
| **Satago** (UK) | Credit control, risk insight, single-invoice finance | £25 / £45 + VAT per mo (help centre) | Not documented | Email, SMS | Payment sync | Yes | [8] |
| **Gaviti** (US/IL) | Mid-market AR; AI assistant, ACH portal | Quote only | Not documented | Email | Not documented | Yes (ACH) | [6] |
| **Tesorio** (US) | Enterprise AR; AI agent triages email, drafts replies, extracts promises | ≈US$7,500–44,500/yr (Vendr median ≈US$17,000) | Drafts replies | Email | Promise extraction | Not found | [7] |
| **Paystand** (US) | B2B payments network plus AR automation and reminders | Not published | Not found | Email | Not found | Yes | [9] |
| **collectAI** (DE, Aareal Bank) | Order-to-cash and dunning for mid/large firms | Not published | Not found | Not found | Not found | Yes | [10] |
| **Daylit** (US, 2026) | Autonomous AI agents choose who to nudge; email, calls, texts | Not published; enterprise-oriented | **No: autonomous** | Email, voice, SMS | Not found | Not found | [11] |

**2025–2026 "AI collections agents".** Daylit launched autonomous collections agents in April 2026 [11]. Upflow, Tesorio, Kolleno, Paidnice and Trove all now sell an "AI agent" layer [1][2][5][7][14][16]. Hobby builds such as CollectFlow (an n8n workflow) ship in draft mode by default [12]. **Approve-first is now an ordinary setting in this category, not a novelty** *(Fact, M)*. Helport (Feb 2025) and Home Credit PH's VoiceBot show AI collections in PH, but for lenders and BPOs, not small businesses [51].

**What none of them does (fact, as far as the excerpts show):** Taglish; Viber or Messenger; working without an accounting ledger; PH payment rails (GCash, Maya, QR Ph) in the payment link; or peso pricing.

---

## 2. Built-in reminders and AI in accounting software

| Product | Reminder feature | AI / agent | Works in PH? | PH price | Src |
|---|---|---|---|---|---|
| **Xero** | Invoice reminders (the product forum mentions a 5-reminder limit) | **JAX payment follow-ups** (Xerocon 2026): plan per customer, consolidated statements, switch to SMS when email gets no response, "persistent chasing until payment". General-availability date and regions **not found** | Yes, Xero sells in PH. **JAX follow-ups in PH: not found** | Lite US$7 (≈₱430, 5 invoices); Starter US$29 (≈₱1,770, 20 invoices per a 3P article); Standard US$50 (≈₱3,050); Premium US$75. A price rise is announced from 1 Nov 2026 | [I1][17][18] |
| **QuickBooks Online** | Up to 3 automatic email reminders, scheduled within ±90 days of the due date; the PH invoicing page advertises "personalized automatic reminders" | Intuit Assist drafts personalised reminders (announced Nov 2024). The "Payments AI" FAQ says **eligible US users** only. Payments Agent: US from July 2025 | Reminders yes (PH page); **AI reminders: US only** on the evidence found | PH prices not shown on the PH page | [I4][19][20] |
| **Zoho Books / Zoho Invoice** | Automated payment reminders **on the free Books plan** (1 user, 1,000 invoices/yr); Zoho Invoice is "free forever" (vendor listing) | Zia (see [I4]) | Yes. Wave tells non-US/CA users to move to Zoho Books [24] | Paid Books US$15–20/mo; PH peso price **not found** | [21][22][24] |
| **Zoho Books + WhatsApp** | Since April 2024, a connected WhatsApp sends payment and sales notifications through utility templates; third parties (Gallabox, Saysimple) add WhatsApp reminders | — | Country list **not found** | — | [21] |
| **FreshBooks** | Late-payment reminders and late fees on paid plans (which tier is disputed) | — | PH availability and price **not found** | US$13.50–38/mo (conflicting) | [23] |
| **Wave** | Reminders only with online payments or Pro | — | **No: businesses outside the US and Canada can no longer send invoices or reminders through Wave** | — | [24] |
| **Odoo** | Follow-up levels by days overdue; actions: email, SMS, letter, manual call task; can run automatically | Odoo 19 AI ([I4]) | Yes (39 PH partners [I4]) | Standard US$31–39 per user/mo list; a 3P site claims a "low-USD" PH price list of US$9–11 | [25][26] |
| **JuanTax / Juan** | **No invoicing reminder feature found** in its help centre | — | PH | Juan paid plans from ₱2,000/mo [I6] | [54][I6] |
| **Taxumo** | **No invoicing reminder feature found** (search returned nothing on Taxumo) | — | PH | 8% plan ₱1,374–2,699/quarter [I6] | [I6] |

**Reading (hypothesis):** a PH business that already runs Xero, QBO, Zoho or Odoo gets template email reminders for free and is unlikely to pay ₱1,500 to repeat them. Balanse is worth its price only for what these tools don't do: reply-aware stopping, owner approval of wording, Taglish tone, and non-email channels. **"Kaya"** in the brief appears to be **Kaya Founders, a VC fund that backs Mochi** [35]. No accounting app called Kaya was found.

---

## 3. PH payment and invoicing platforms

| Platform | Payment links / invoices | Reminders | Stops on payment? | Approval / AI / Taglish | Fees | Src |
|---|---|---|---|---|---|---|
| **PayMongo** | Payment links; **Invoices (beta)**, by email or shared link | **Automatic: 3 days before due, on the due date, +3 days, +7 days, by email; on by default; manual "Remind customer"** | **Yes, stops when paid or voided** | None | GCash 2.23%, QR Ph 1.34%, cards 3.125% + ₱13.39 (developer page; an older page shows 2.5% / 1.5% / 3.5% + ₱15); VAT-inclusive | [29][30][I6] |
| **Xendit** | Payment links / XenInvoice | Notifications on creation, **reminder before due (configurable)** and on success; email and WhatsApp; older pages list Viber (PH) and SMS | Yes (success notice ends the cycle; inferred) | Template-based; no AI or approval found | Local cards 3.2% + ₱10; GCash 2.3%; Maya 1.8% (pricing calculator, undated) | [27][28] |
| **Maya Business** | Maya Business Manager → Checkout and Invoicing; invoices sent "via email, chat or SMS"; link format `mymaya.me/{alias}?amt=` | Reminders **not verified**; HitPay (a competitor) says no late-fee automation | Not found | None found | Not published; QR Ph 1% promo for 3 months | [32] |
| **GCash for Business** | QR and payment acceptance; "Business Basic" for sole proprietors (≈₱500k/mo cap, 3P guide) | **Not found** ("Request Money" not verified, search budget out) | — | — | MDR 1.0% QR Ph, 3.2% cards, VAT-inclusive (GCash Help Centre) | [33] |
| **HitPay** | Invoices with an embedded link; GCash, Maya, QR Ph, InstaPay, PESONet, cards; recurring billing | Renewal reminders; late-fee automation (vendor) | Not found | None | No monthly fee; per-transaction rates | [31] |
| **Dragonpay** | Gateway; Lite plan free to set up | **Not found** | — | — | ₱20/transaction (Lite); full version ₱36,000 setup (3P review, possibly dated) | [34] |
| **Mochi** (PH, est. 2023, launched Jan 2024) | Digital invoices, payment terms, **payment links**, recurring billing, customer management | **Automated reminders**; channel and reply handling **not found** | Not found | AI / approval / Taglish **not found** | **₱1,899/mo standard** plus transaction fees; 60-day trial (Dealroom profile, "Made with AI"); design-partner promo ₱188/mo | [35] |
| **First Circle Invoice** (PH lender) | Free invoicing; computes VAT and withholding; BIR fields (marketing) | **"Unlimited, automatic" reminders; follows up missing payments** | Not found | None found | **Free, waitlist**. Its terms say the e-invoice is not a BIR-accredited invoice or proof of payment | [36] |

**Reading:**
- *Fact (M):* the two biggest PH gateways already send reminders that stop on payment, and charge only when money moves.
- *Hypothesis:* a PH business that bills through PayMongo or Xendit already has "reminders" at no subscription cost. Balanse must therefore **sit on top of whatever link the business already uses** (paste any GCash/Maya/PayMongo/bank detail) rather than compete with the gateways. It should sell the conversation, not the payment.

### 3.1 PH "utang" and ledger apps

| App | What it is | Reminder | Scale | Src |
|---|---|---|---|---|
| **Lista PH** (2021) | Free finance and bookkeeping "superapp"; utang records linked to phonebook contacts | **Free automated SMS of the balance owed on each utang transaction**; billing reminders via Messenger/WhatsApp (2021 launch coverage) | iOS since Aug 2021; **downloads not found** | [39] |
| **Bangon: OLA & Debt Tracker** | For *borrowers*: tracks OLA, 5-6 and informal debts; logs harassment | None (borrower side) | Not found | [40] |
| **Utang Free PH** | Payoff planner in Taglish; 64+ lenders | None (borrower side) | "Outside top 30" (US rank, irrelevant) | [40] |
| **UtangIna** (Product Hunt) | Tracks personal loans; automated SMS/email "friendly reminders … with less hiya" | Yes | 0 upvotes | [40] |
| **SukliPH**, **Tallyx** | Sari-sari sales, utang and inventory (pre-launch or early) | Not found | 2 upvotes; GitHub only | [40] |
| *Comparators abroad:* **Khatabook / OkCredit** (India), **BukuWarung / Paper.id** (Indonesia) | Free ledgers; free SMS/WhatsApp reminders; OkCredit's include a UPI pay link; Paper.id claims 600,000+ businesses (vendor claim) | Yes | **No PH operation found** | [41][42] |

**Install counts for PH utang apps on Google Play: not found** (the search tool showed no Play listings with counts). *Hypothesis:* this segment expects **free**. UtangIna's pitch ("collect debts with less hiya") names the same emotional job as Balanse, but at the consumer level, where ₱1,500/mo is implausible.

---

## 4. Substitutes

| Substitute | What it costs | What it does well | Where it fails | Src |
|---|---|---|---|---|
| **Typing it on Viber / Messenger / SMS by hand** | ₱0 plus the owner's time and awkwardness | Reaches the debtor where they are (Viber claims >80% PH penetration); personal | Forgotten, inconsistent, emotionally costly; no record of who was chased when; no stop rule | [47] |
| **Excel / Google Sheets aging list** | ₱0 (paid templates ≈CAD 10) | Aging buckets, status columns | Does not send or remind; manual | [53] |
| **Free reminders in invoicing / gateways** (PayMongo, Xendit, Zoho, QBO) | ₱0 subscription; 1.3–3.2% per payment | Automatic; stops on payment | Email-centric, fixed wording, no reply reading, needs invoices issued in that tool | [20][21][27][30] |
| **Lawyer's demand letter** | ≈₱3,000–30,000+ (simple: ₱5,000–20,000 in Metro Manila) + 12% VAT + notarisation ₱500–2,000 + registered mail ₱150–300; IBP schedule is advisory | Signals seriousness; prerequisite to suit | One-off; damages the relationship | [44] |
| **Collection agency** | Indicative B2B contingency 5–15%; 10–25% at 60–180 days, up to 30–50% past a year; lawyer-led 10–30% of recovered amounts | No effort for the owner | Relationship damage. SEC MC 18-2019 bans shaming and contact with third parties, but only for lending and financing companies and their collectors | [43][46] |
| **Small claims court** | Docket and other fees (official schedule not fetched); no lawyers at the hearing | Claims up to ₱1,000,000 principal; single hearing; no appeal | Time; needs documentation | [45] |
| **Invoice financing** (First Circle, Acudeen) | First Circle: "as low as" 0.99–1.39%/month (2018–2022 coverage; current rates **not found**); Acudeen: up to 90% advance (dated) | Cash now; the chasing problem moves to the lender | Only for invoices to creditworthy corporates; cost; current terms unknown | [37][38] |
| **Adjacent PH AI chat tools** (Asikaso ₱1,499/mo, Replia ₱799–1,999) | ₱800–2,500/mo | Taglish AI on Messenger/IG, 3 AI follow-ups per conversation | Lead and inquiry follow-up, not debts; Meta-only; no approval by default | [I2] |

**Rough cost anchors on a ₱100,000 overdue B2B invoice** *(arithmetic on the ranges above, not sourced quotes):*
- Agency at 5–15%: ₱5,000–15,000.
- Demand letter: ₱5,000–20,000 + VAT.
- Financing at a 1.39%/month historical rate for 60 days: ≈₱2,800.
- Balanse: ₱1,500 for a month of follow-ups across *all* debtors.

**Messaging channel facts that constrain any automated PH tool:**
- **Messenger:** ACCOUNT_UPDATE, POST_PURCHASE_UPDATE and CONFIRMED_EVENT_UPDATE tags return error 100 from 27 April 2026. Outside 24 hours, automated sends need Meta-approved **utility templates** (beta access reported as patchy) or paid **marketing messages**. The HUMAN_AGENT tag allows *manual* replies within 7 days [49].
- **WhatsApp:** utility templates are free inside the customer-service window and charged outside it (since July 2025) [50]. WhatsApp is minor in PH next to Viber and Messenger (no PH share figure found).
- **Viber Business Messages:** reseller prices ≈€0.0026 or US$0.0019 per transactional message, but with **€115 to US$500 monthly minimums** [47].
- **SMS:** Semaphore ₱0.56 per message ex-VAT to all PH networks [48].

---

## 5. Comparison table

✔ = documented; ✘ = documented absent; ? = not found. Prices per month unless stated.

| | Price | PH-local | Approve each message | Taglish | Email | SMS | Viber | Messenger | Stops on reply | Stops on payment | Payment link | Needs accounting app |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Balanse (planned)** | ₱1,500 | ✔ | ✔ | ✔ | ✔ | planned | owner-sent | owner-sent | ✔ | ✔ (marked paid) | pastes any | ✘ |
| Mochi | ₱1,899 (L) | ✔ | ? | ? | ? | ? | ? | ? | ? | ? | ✔ | ✘ |
| First Circle Invoice | Free (waitlist) | ✔ | ? | ? | ? | ? | ? | ? | ? | ? | ? | ✘ |
| PayMongo Invoices | % per payment | ✔ | ✘ (auto) | ✘ | ✔ | ✘ | ✘ | ✘ | ✘ | ✔ | ✔ | ✘ |
| Xendit links | % per payment | ✔ | ✘ (auto) | ✘ | ✔ | old docs | old docs (PH) | ✘ | ✘ | ✔ | ✔ | ✘ |
| Lista PH | Free | ✔ | ✘ | ? | ✘ | ✔ | ? | ✔ (2021) | ✘ | ? | ✘ | ✘ |
| Zoho Books/Invoice | Free–US$20 | sold in PH | ✘ | ✘ | ✔ | ? | ✘ | ✘ | ✘ | ✔ | ✔ | is one |
| QuickBooks Online PH | ? (PH) | sold in PH | ✘ (AI US only) | ✘ | ✔ | ✘ | ✘ | ✘ | ✘ | ✔ | ? in PH | is one |
| Xero + JAX follow-ups | US$29–75 | sold in PH | ? | ✘ | ✔ | ✔ (JAX) | ✘ | ✘ | ? | ✔ | ✔ | is one |
| Odoo follow-up levels | US$9–39/user | partners | manual option | ✘ | ✔ | ✔ | ✘ | ✘ | ✘ | ✔ | ✔ | is one |
| Solido | ? (3 tiers) | ✘ | ✔ | ✘ | ✔ | ✘ | ✘ | ✘ | ? | ✔ | invoice | Xero |
| Trove | GBP 50–265 | ✘ | ✔ (or auto) | ✘ | ✔ | ? | ✘ | ✘ | updates | ? | ? | Xero |
| Paidnice | US$69 | ✘ | replies drafted | ✘ | ✔ | ? | ✘ | ✘ | holds on promise | ✔ | ✔ | Xero/QBO |
| Chaser | US$259 | ✘ | ? | ✘ | ✔ | ✔ | ✘ | ✘ | ? | ✔ | ✔ | Xero/QBO |
| Upflow | quote | ✘ | ✔ (suggestions mode) | ✘ | ✔ | ? | ✘ | ✘ | promise skill | ✔ | ✔ | ERP |
| Asikaso AI (adjacent) | ₱1,499 | ✔ | ✘ | ✔ | ✘ | ✘ | ✘ | ✔ | n/a | n/a | ✘ | ✘ |
| By hand on Viber | ₱0 | ✔ | ✔ (it's you) | ✔ | — | ✔ | ✔ | ✔ | ✔ (you) | ✔ (you) | you paste | ✘ |

Sources by row: [35][36][30][27][39][21][19][20][17][18][25][15][16][14][13][1][2][I2]. Balanse's row is the plan from the brief, not evidence.

---

## 6. Analysis

### 6.1 Where the gap is (hypothesis, unproven)

Nothing found combines all of these:
1. **Works without accounting software.** The debtor list comes from a paste, a CSV or typed entries. *Why:* 11% digital-accounting adoption [I1]. Mochi and First Circle also don't need one, so this alone is not a gap.
2. **AI-drafted, owner-approved wording in Taglish or English**, with a polite-to-firm tone ladder. *Why:* every PH reminder tool found sends fixed templates automatically [27][30][39]. The approve-first tools are Xero-only and English [15][16].
3. **Reads replies and stops** ("nabayaran ko na po", "sa 15 po", "mali ang amount"), then records promises and disputes. *Why:* PH tools stop on *payment* through their own rails; reply-aware stopping was found only in Paidnice, Upflow and Trove [2][14][16].
4. **Sends through PH-realistic routes:** email and SMS automatically, and Viber/Messenger as **one-tap "copy and open chat" for the owner to send** (no Meta/Viber API cost). *Why:* the channel economics in §4.
5. **Sent exactly once, with an audit trail** of who was chased, when, and with what words. Useful if a demand letter or small-claims case follows [44][45].

*Hypothesis on who feels this most:* B2B suppliers, contractors, agencies, tutors and landlords with 10–100 debtors, often paid by GCash or bank transfer, who chase on Viber today and find asking for money awkward.
- *Fact:* PH SMEs commonly extend 30/60/90-day terms (opinion piece) [55].
- *Not found:* any PH survey of overdue receivables.

### 6.2 What a PH small business compares ₱1,500/month against

| Anchor | Amount | Src |
|---|---|---|
| Doing it by hand, Sheets, gateway reminders, Zoho Invoice, Lista | ₱0 | [21][22][30][39][53] |
| Mochi (PH AR tool) | ₱1,899/mo (L) | [35] |
| Asikaso, the PH Taglish AI chat tool | ₱1,499/mo | [I2] |
| Xero PH Starter / Standard (includes reminders) | ≈₱1,770 / ≈₱3,050 | [18] |
| Juan accounting | from ₱2,000/mo | [I6] |
| Paidnice / Chaser (if they look abroad) | ≈₱4,200 / ≈₱15,800 | [I1] |
| One lawyer's demand letter | ₱3,000–30,000+ once | [44] |
| Agency on a ₱100k B2B debt (5–15%) | ₱5,000–15,000 | [43] |
| A part-time admin or VA to chase | **not found** (no PH wage source gathered) | — |

*Hypothesis:* the honest comparison is "**₱0 and my own awkwardness**" against "₱1,500 and nothing goes out without me". At ₱1,500, Balanse must visibly bring in more than ₱1,500 a month in cash that would otherwise be late or lost, or replace one demand letter a year. Pilots should measure cash collected after a Balanse message, not messages sent.

### 6.3 The most dangerous competitor: Mochi

- **Same buyer and country:** Filipino SMEs that bill customers and chase payments [35].
- **Same price band:** ₱1,899/mo against ₱1,500 (L: from an AI-written Dealroom profile).
- **It already holds the hard parts Balanse lacks:** invoices, payment links, recurring billing, and a PayMongo partnership that puts it in front of "thousands of active PayMongo merchants" [35].
- **Funded and focused** (pre-seed: Tenity, Kaya Founders, Foxmont) [35]. Adding LLM-drafted Taglish reminders with an approve button is weeks of work for them.
- **Runners-up:**
  - **First Circle Invoice:** free, run by a lender with an SME customer base, and with every reason to give reminders away to originate loans [36].
  - **PayMongo:** if Invoices leaves beta and adds a "review before send" switch, the gateway itself becomes the default [30].
  - **Xero JAX** is the strongest *technology* threat but weak in PH distribution [17][I1].
- **Unknowns that could change this ranking:** Mochi's current price, its user count, its channels (email only?), whether its reminders read replies, and whether it is still active in late 2026. **None was found.**

### 6.4 Threats from adjacent players (hypothesis)

- **Asikaso and the other PH Taglish AI chat tools** [I2] could add "payment reminder" flows inside Messenger. They are Meta-only and auto-send, but already have the Taglish and the price point.
- **Viber Pay** (planned for PH, awaiting approval in 2025; current status not found) could bundle request-to-pay and reminders inside Viber, the channel PH businesses chase on.

---

## 7. Recommendations (for the owner)

1. **Do not position Balanse as "automatic payment reminders".** That is free in PayMongo, Xendit, Zoho and QBO [21][27][30]. Position it as **"polite follow-ups in your voice, Taglish or English, that you approve, that stop the moment they reply"**: the conversation, not the reminder.
2. **Be payment-rail neutral.** Let the owner paste any GCash number, QR, Maya or PayMongo link, or bank details, and mark debts paid. Don't build a gateway; compete with Mochi on the conversation, not on invoicing.
3. **Plan channels around cost:** email and SMS automated (≈₱0.56/SMS [48]); Viber and Messenger as owner-sent drafts (copy and open the chat), so no €115+/mo Viber minimum or Meta utility-template approval is needed for pilots [47][49].
4. **Run a kill check before building past the core loop:** sign up for Mochi's trial and First Circle's waitlist, and read PayMongo Invoices in its beta, to see whether any already drafts per debtor, reads replies or uses SMS/Viber. If Mochi does approve-first Taglish replies, Balanse's differentiation is gone.
5. **Keep Balanse outside debt-collector rules by design:** messages go out in the business's own name, there is no third-party contact, and nothing resembling shaming. SEC MC 18-2019's limits (no contact before 6 a.m. or after 10 p.m., no disclosure to others) make sensible defaults even though they bind only lenders [46].

---

## 8. Unresolved questions

1. Mochi: current price, user count, channels, reply handling, AI features, still operating?
2. First Circle Invoice: live or still waitlisted; channels; whether reminders stop on reply.
3. PayMongo Invoices: still beta? Will it add SMS/Viber or editable per-debtor wording?
4. Xero JAX payment follow-ups: general-availability date and whether PH Starter/Standard get them.
5. Google Play install counts for Lista PH and any PH utang ledger with reminders: **not found**.
6. GCash "Request Money" and Maya reminders as free substitutes: **not verified** (search budget ran out).
7. Any PH data on overdue receivables, days to pay, or how many small businesses sell on credit: **not found**.
8. Whether Meta utility templates on Messenger are open to small pages in PH, and their price: **not found**.

---

## Sources

All accessed 2026-10-24 unless an internal source says otherwise. Types: OD official/regulator, PP vendor pricing page, VD vendor docs, VC vendor claim, N news, TP third party, SR search-result excerpt only (page not fetched). **Every external source here is SR** because no page could be fetched; the second label is what the underlying page is. Confidence: H/M/L.

**Internal (Kompleto research, read in full or in part):**
- I1. [Kompleto `docs/research/self-serve-distribution.md`](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/research/self-serve-distribution.md), 2026-10-18. Xero App Store debtor apps (Chaser US$259, Paidnice US$69, Solido, Trove GBP 50), Xero PH Lite US$7/5 invoices, 11% MSME adoption (vendor-cited), Intuit Payments Agent (US). Ratings as stated there (mostly H).
- I2. [Kompleto `docs/research/follow-up-competitors.md`](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/research/follow-up-competitors.md), 2026-10-18. Asikaso ₱1,499/mo, Replia ₱799–1,999, PH Taglish AI chat tools. H for prices there.
- I3. [Kompleto `docs/research/competitor-analysis.md`](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/research/competitor-analysis.md). Background; not cited for specific facts.
- I4. [Kompleto `docs/research/ai-addons-erp.md`](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/research/ai-addons-erp.md). JAX announced follow-ups; Intuit agents; Odoo AI and PH partners. M–H.
- I5. [Kompleto `docs/research/opportunities-business.md`](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/research/opportunities-business.md) §3.6. AR collections evidence (QuickBooks Late Payments Report). M.
- I6. [Kompleto `docs/research/new-ideas-philippines.md`](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/research/new-ideas-philippines.md). PayMongo GCash 2.23%; Juan from ₱2,000/mo; Taxumo prices. H for prices there.

**External:**
1. Upflow pricing: ERPResearch, https://erpresearch.com/erp-add-ons/ar-collections/upflow/pricing ; Vendr, https://www.vendr.com/marketplace/upflow ; Capterra, https://www.capterra.com/p/193097/Upflow/pricing/ (2026 pages, dates not shown). SR/TP. M.
2. Upflow docs, "Collection agents" and "Draft email replies automatically with Upflow's AI", https://docs.upflow.io/en-us/collection-and-collaboration/emails/collection-agents (undated); feature page https://upflow.io/en-gb/features/cash-collection. SR/VD (outcomes VC). M.
3. InvoiceSherpa: Software Advice, https://www.softwareadvice.com/accounting/invoicesherpa-profile/ ; SaaSworthy, https://www.saasworthy.com/product/invoicesherpa/pricing (2026). SR/TP. M.
4. Kolleno pricing: ERPResearch, https://erpresearch.com/erp-add-ons/ar-collections/kolleno/pricing ; PricingSaaS, https://pricingsaas.com/companies/kolleno (2026). SR/TP, sources conflict. L–M.
5. Kolleno features: ERPResearch, https://erpresearch.com/erp-add-ons/ar-collections/kolleno ; Kolleno FAQ, https://www.kolleno.com/faqs/ (undated). SR/VD+VC. M.
6. Gaviti: ToolRadar, https://toolradar.com/tools/gaviti ; G2 compare, https://www.g2.com/compare/gaviti-vs-upflow-upflow (2026). SR/TP. M.
7. Tesorio: ERPResearch, https://erpresearch.com/erp-add-ons/treasury/tesorio/pricing ; Vendr, https://www.vendr.com/marketplace/tesorio ; Tesorio AI agents page, https://tesorio.com/product/ai-agents (undated). SR/TP and VC. M (features), L (prices).
8. Satago help centre, "What does an SME subscription cost", https://help.satago.com/en/articles/13214-what-does-an-sme-subscription-cost ; "What is Satago", https://help.docs.satago.com/wiki/spaces/SCS/pages/3952771588/What+is+Satago (undated). SR/PP+VD. M.
9. Paystand: Fitgap, https://us.fitgap.com/products/paystand ; Capterra UK, https://capterra.co.uk/reviews/131733/paystand. SR/TP. L.
10. collectAI: Capterra IE, https://www.capterra.ie/software/180760/collectai ; CB Insights, https://www.cbinsights.com/investor/collectai. SR/TP. M.
11. "Daylit Launches AI Agents for Automated Collections", CPA Practice Advisor, 2026-04-16, https://www.cpapracticeadvisor.com/2026/04/16/daylit-launches-ai-agents-for-automated-collections/180602/ ; neuronfeed comparison. SR/N (results figures VC). M.
12. CollectFlow, Product Hunt, https://www.producthunt.com/products/collectflow-ai-invoice-collection-agent (2026). SR/VC. L.
13. Chaser pricing: FrontDeskReview (June 2026), https://frontdeskreview.com/software/ar-automation/chaser/ ; ToolRadar, https://toolradar.com/tools/chaser/pricing ; Chaser help, https://help.chaserhq.com/chaser-pricing-details (legacy plans). SR/TP+VD. M.
14. Paidnice: Xero App Store NZ listing, https://apps.xero.com/nz/function/debtor-tracking/app/paidnice ; Software Advice, https://softwareadvice.com/debt-collection/paidnice-profile/ (2026). SR/VD+VC. M.
15. Solido: Xero App Store AU listing, https://apps.xero.com/au/app/solido (2026). SR/VD. M.
16. Trove: Xero App Store UK listing, https://apps.xero.com/uk/function/debtor-tracking/app/trove ; Capterra, https://www.capterra.com/p/10040699/Trove/ (2026). SR/PP+VD. M.
17. Xero JAX: IT Brief Australia, "Xero adds AI tools to JAX for small business finance", https://itbrief.com.au/story/xero-adds-ai-tools-to-jax-for-small-business-finance ; Accounting Today, "Xerocon 2026: JAX AI improvements…", https://www.accountingtoday.com/news/xerocon-2026-jax-ai-improvements-focused-on-automating-unbillable-admin-work (2026); Xero Product Ideas forum. SR/N reporting VC. M.
18. Xero PH pricing: https://www.xero.com/ph/pricing-plans/ and https://www.xero.com/ph/pricing-plans/lite/ (current); quasa.io (Tagalog), https://quasa.io/tl/media/quickbooks-o-xero-sa-ph-unlimited-users-laban-sa-mas-pamilyar-na-workflow. SR/PP (M) and TP (L, for US$ amounts and the 20-invoice cap).
19. Intuit AI reminders: The CFO Club, https://thecfoclub.com/partner-spotlight/intuit-quickbooks-online/how-to-automate-payment-follow-ups-in-quickbooks-payments/ ; Intuit/Business Wire, 2024-11-20, https://www.businesswire.com/news/home/20241120869309/en. SR/TP and VC. M.
20. QuickBooks PH invoicing page, https://quickbooks.intuit.com/ph/invoicing ; PH pricing, https://quickbooks.intuit.com/ph/pricing/ ; QBO help "Send invoice reminders automatically or manually" (global/IE/CA), https://quickbooks.intuit.com/learn-support/en-global/help-article/invoicing/send-invoice-reminders-automatically-manually/L84cQjpxo_ROW_en. SR/VD. M.
21. Zoho Books pricing (AU), https://www.zoho.com/au/books/pricing/ ; Zoho Books "What's New, April 2024" (WhatsApp), https://help.zoho.com/portal/en/community/topic/whats-new-in-zoho-books-april-2024 ; Merchant Maverick review. SR/PP+VD. M.
22. Zoho Invoice app listing (PH), AppFollow, https://apps.appfollow.io/ios/zoho-invoice-maker-app/413017364?country=ph (updated July 2026). SR/VC. L–M.
23. FreshBooks help, "What are Late Payment Fees and Reminders", https://support.freshbooks.com/hc/en-us/articles/227559727 ; Capterra pricing, https://www.capterra.com/p/142390/FreshBooks/pricing/. SR/VD+TP. L–M.
24. Wave support, "Changes for Wave users outside the United States and Canada", https://support.waveapps.com/hc/en-us/articles/27277914806804. SR/VD. M.
25. Odoo follow-ups: user docs, https://documentation-user.readthedocs.io/en/latest/accounting/receivables/getting_paid/automated_followups.html ; Cybrosys "Odoo 19 Accounting Book", https://www.cybrosys.com/odoo/odoo-books/odoo-19-accounting/configuration-and-setup-tools/configuring-invoicing/ ; odoo/documentation commit 096cb6d. SR/VD+TP. M.
26. Odoo pricing, https://www.odoo.com/pricing ; OEC.sh "Odoo pricing Philippines", https://oec.sh/odoo-pricing/philippines. SR/PP (M) and TP (L).
27. Xendit docs, "Payment Link – Notification and Callback", https://docs.xendit.co/payment-link/notification-and-callback ; Xendit help, "Can I send invoice notification by Email/SMS/WhatsApp", https://help.xendit.co/hc/en-us/articles/4408407796749 (older). SR/VD. M.
28. Xendit PH pricing calculator, https://www.xendit.co/en-ph/pricing-calculator-ph/ (undated). SR/PP. M.
29. PayMongo developer pricing, https://developers.paymongo.com/docs/pricing ; PayMongo pricing, https://paymongo.com/pricing ; Fees & Rates (older), https://developers.paymongo.com/docs/fees-and-rates. SR/PP, schedules conflict. M.
30. PayMongo docs, "Invoices – Reminders", https://docs.paymongo.com/docs/payment-channels-invoices-reminders ; "Invoicing", https://docs.paymongo.com/docs/invoicing. SR/VD. M.
31. HitPay blog, "Best invoicing software Philippines", https://hitpayapp.com/blog/best-invoicing-software-philippines ; "Recurring billing PH", https://hitpayapp.com/blog/recurring-billing-ph. SR/VC (competitor-authored). L–M.
32. Maya, "How a digital invoicing solution like Maya Invoice Payments can help", https://www.maya.ph/business/stories/how-a-digital-invoicing-solution-like-maya-invoice-payments-can-help-your-business ; Maya developers, "Payment Links", https://developers.maya.ph/docs/payment-links. SR/VD. M.
33. GCash Help Centre, "What is the GCash for Business MDR?", https://help.gcash.com/hc/en-us/articles/55684605325593 ; itsourcecode, "How to set up GCash for Business in 2026", https://itsourcecode.com/gcash/how-to-set-up-gcash-for-business-2026/. SR/PP (M) and TP (L).
34. Dragonpay: DayTrading.com review, https://www.daytrading.com/dragonpay ; Wise PH, https://wise.com/ph/blog/what-is-dragonpay. SR/TP, possibly dated. L.
35. Mochi: The Independent Investor, "Mochi launches hassle-free invoicing for Filipino SMEs…", https://theindependentinvestor.ph/mochi-launches-hassle-free-invoicing-for-filipino-smes-with-new-accounts-receivable-platform/ ; PayMongo blog, 2025-04-10, https://www.paymongo.com/blog/paymongo-mochi-powering-business-growth ; Dealroom profile ("Made with AI"), https://app.dealroom.co/companies/mochi_4 ; Mochi Design Lab, https://lu.ma/ri5prrsq. SR/N (M), VC (M), TP (L for price and funding).
36. First Circle, Invoice page, https://www.firstcircle.ph/invoice ; "Terms & Conditions for E-Invoicing", https://www.firstcircle.ph/company/terms-conditions-for-e-invoicing. SR/VC+VD. M.
37. First Circle financing: Manila Times digital edition (2022), https://digitaledition.manilatimes.net/article/282827899911081 ; FintechNews SG, https://fintechnews.sg/24227/fintechphilippines/philippine-based-fintech-startup-first-circle-raises-26m-launches-credit-facility-for-smes/ ; Dealroom. SR/N, dated. L.
38. Acudeen: BusinessWorld, https://www.bworldonline.com/?p=138859 ; CoinGeek, https://coingeek.com/acudeen-extends-direly-needed-financial-services-msmes-philippines/ (2018–2020). SR/N, dated. L.
39. Lista PH: Malaya Business Insight, 2025-09-15, https://malaya.com.ph/info-tech/tech-this-week/the-free-superapp-for-people-looking-to-manage-their-finances ; App Store, https://apps.apple.com/app/id1580527951 ; NewsBytes PH, 2021-11-16, https://newsbytes.ph/2021/11/16/filipino-tech-startup-officially-launches-finance-tracker-app-for-small-businesses/ ; mwm.ai listing. SR/N+VC. M.
40. PH utang apps: Bangon (chrome-stats), https://chrome-stats.com/d/com.bangonapp.app ; Utang Free PH (mwm.ai), https://mwm.ai/apps/utang-free-ph/6768990224 ; UtangIna, https://hunted.space/product/utangina ; SukliPH, https://www.hunted.space/product/sukliph ; Tallyx, https://github.com/Polqt/tallyx. SR/TP+VC. L.
41. OkCredit, https://okcredit.in and https://okcredit.in/khatabook-alternative ; Khatabook App Store, https://apps.apple.com/us/app/id6633420817. SR/VC. M (features); no PH presence found.
42. BukuWarung: Twilio customer story, https://customers.twilio.com/en-us/bukuwarung ; Paper.id, https://www.paper.id/en/sales-invoice.php ; Paper.id App Store, https://apps.apple.com/us/app/id6463997183. SR/VC. M.
43. Collection fees PH: Lawyer-Philippines, "Debt Collector Commission Regulations Philippines", https://www.lawyer-philippines.com/articles/debt-collector-commission-regulations-philippines ; Respicio, "Debt collection services on a no-win-no-fee basis", https://www.respicio.ph/commentaries/debt-collection-services-on-a-no-win-no-fee-basis-in-the-philippines (2025). SR/TP (law-firm blogs). L.
44. Demand letter fees: Respicio, "How Much Does a Lawyer Charge for a Demand Letter in the Philippines?", https://www.respicio.ph/commentaries/how-much-does-a-lawyer-charge-for-a-demand-letter-in-the-philippines ; Lawyer-Philippines, "Lawyer Fees for Demand Letter and Small Claims", https://www.lawyer-philippines.com/articles/lawyer-fees-for-demand-letter-and-small-claims-philippines. SR/TP. L–M.
45. Small claims: NDV Law, https://ndvlaw.com/how-to-navigate-the-small-claims-court-why-collecting-unpaid-debts-in-the-philippines-is-faster-than-you-think-2/ ; Respicio, "Small Claims Filing Fees in the Philippines", https://www.respicio.ph/commentaries/small-claims-filing-fees-in-the-philippines (secondary to A.M. No. 08-8-7-SC as amended 2022; official text not fetched). SR/TP. M.
46. SEC MC No. 18 s. 2019: Digest.ph, https://www.digest.ph/corporate/prohibition-on-unfair-debt-collection-practices-of-financing-companies-fc-and-lending-companies-lc ; PhilStar via creditinfo.gov.ph, https://creditinfo.gov.ph/node/339 ; Tribune, 2026-06-17, https://tribune.net.ph/2026/06/17/sec-fines-lender-over-abusive-debt-collection. SR/OD (secondary) and N. M.
47. Viber PH: BusinessWorld, "Viber sees 506% growth in transactional messages, PHL top market…", https://www.bworldonline.com/?p=458501 (date not shown); reseller prices: Klink, https://www.klink.cloud/viber-for-business ; SMSBAT, https://smsbat.com/help/billing/viber-pricing/ ; Infobip, https://www.infobip.com/viber-business/pricing ; Viber Pay: FintechNews PH, https://fintechnews.ph/65816/e-wallets/viber-pay-digital-wallet-philippines-2025/. SR/N (penetration is a vendor executive's claim) and TP. M / L for prices.
48. Semaphore SMS gateway, https://www.semaphore.co (undated). SR/PP. M.
49. Meta, Messenger Platform changelog, https://developers.facebook.com/documentation/business-messaging/messenger-platform/changelog ; "Send a Message", https://developers.facebook.com/docs/messenger-platform/send-messages ; ManyChat community thread (2026). SR/VD (M–H); community anecdotes L.
50. Zoho community, "WhatsApp Message Pricing Changes (Effective July 1, 2025)", https://help.zoho.com/portal/en/community/topic/whatsapp-message-pricing-changes-effective-july-1-2025. SR/TP summarising Meta. M.
51. Helport: Inquirer Technology, Feb 2025, https://technology.inquirer.net/140906/helport-enhances-debt-collection-in-the-philippines-with-ai ; Home Credit PH story, https://www.homecredit.ph/stories/ai-powered-collections-home-credit-ph. SR/N and VC. M.
52. PesoHub, "USD to PHP exchange rate today" (BSP reference rate 61.35, 2026-07-01), https://www.pesohub.ph/rates/exchange-rates/usd-to-php-today/. SR/TP. L–M.
53. AR Aging Report Google Sheets template (CAD 10), Gumroad, https://averitedge.gumroad.com/l/ohuvp ; InfoInspired tutorial, https://infoinspired.com/google-docs/spreadsheet/create-ageing-analysis-report-using-google-sheet-formula/. SR/TP. L.
54. JuanTax help centre, https://help.juan.tax/en (no reminder feature found). SR/VD. L (absence of evidence).
55. "The problem of delayed payments in SMEs" (Manila Times commentary, republished July 2026), https://newswav.com/article/the-problem-of-delayed-payments-in-smes-A2607_Aag3MC. SR/N (opinion). L.
