# Balanse: the Philippine market and the pain of chasing payments

Status: **complete, 2026-10-24.** Scope: which PH customer segments are owed money often enough to pay ₱1,500/month for approved, polite reminders; how they chase today; published evidence of late payment; the cultural angle; free written channels; ranking and kill-risks. Earlier Kompleto research is cited as internal sources [1]–[7] and not redone. Source tags such as [12] refer to the Sources list at the end. All sources accessed 2026-10-24.

Evidence types: OD = official document/regulator, PP = vendor pricing page, VD = vendor docs, VC = vendor claim, N = news, TP = third party, SR = search-result excerpt only (page not fetched). Confidence: H / M / L. "My calculation" marks any figure I multiplied or summed. **Fact**, **Hypothesis** and **Recommendation** labels keep the three apart.

**Method limits.** The session's proxy refused every direct page fetch (World Bank, PSA, DTI, Inquirer and law sites all returned connection refusals), so **every new external claim below rests on a search-engine excerpt (SR)**, even where the underlying page is an official document. The web-search budget ran out before four planned look-ups: the national share of renting households, the current PCAB contractor count, Facebook group sizes and the weighted World Bank Enterprise Survey indicators. They are marked "not found" with that reason.

## Key conclusions

1. **Late payment in PH is real but thinly measured.** No PH survey of SME late-payment rates was found. The best signals are indirect:
   - a July 2026 Manila Times piece saying SMEs serve larger firms, institutions and government on 30-, 60- or 90-day terms and so "finance" their clients [10];
   - Freelance Workers Protection bills refiled in 2025 that name late and non-payment as the problem they address [12];
   - a House bill citing government payment delays of "months to years" to contractors and consultants [13];
   - private-school collection stress (49% of 224 CEAP basic-ed schools at a loss or break-even; collections cover ~7.7 months of costs in a 27-college PACU sample) [1].

   *Confidence: M that the pain exists; L on how often it occurs.*
2. **The cultural angle is plausible but not evidenced.** No study was found that measures reluctance to chase (*hiya*, *pakikisama*). What exists:
   - consumer-finance blogs saying Filipinos avoid asking for money back for fear of damaging relationships [19];
   - a 2005 DLSU thesis on *hiya*, *pakikisama* and *utang na loob* in sari-sari store credit, whose findings could not be read [20];
   - an editorial noting that one word, *utang*, means both credit and debt [18].

   Treat "owners are too *nahihiya* to chase, so a polite drafted message helps" as a **hypothesis** to test in pilots. *Confidence: L.*
3. **The law rewards a written, private paper trail and punishes public shaming.** Small claims go up to ₱1,000,000, and practitioner guides treat proof of a prior demand as essential [24]. Barangay conciliation is a precondition for many disputes [25]. Posting a debtor on Facebook risks cyber libel, Data Privacy Act and unjust-vexation claims [22]. A log of dated, approved, one-to-one reminders is therefore useful evidence as well as a courtesy. *Confidence: M (legal commentary and case law via SR, not a ruling on Balanse).*
4. **Local competitors already sell "invoice + automated reminder" to PH SMEs, cheaply or free.**
   - First Circle's invoicing promises unlimited automatic reminders, "free forever" for waitlist members (vendor claim) [33].
   - Mochi, a PH AR platform for freelancers, agencies and service firms, launched in January 2024 and partnered with PayMongo in April 2025. One unverified listing puts it at ₱1,899/month [34].
   - HitPay lists ₱0/month with automated late fees (vendor's own comparison) [35].

   **Balanse cannot win on "sends reminders".** Its room is Taglish drafts tuned to the relationship, human approval of each message, stopping when the debtor replies, and the audit trail. *Confidence: M.*
5. **The engine sends email, which fits B2B debtors much better than consumer debtors.** Corporate clients have accounts-payable inboxes. Parents, tenants, patients and sari-sari stores are reached by SMS, Viber or Messenger. Since NTC's 2023 memo, SMS with links is blocked [42][7], and Viber bots cost EUR 100/month [1]. *Inference, M.* This pushes the first version towards B2B segments.
6. **Lending (co-ops, microlenders, "5-6") is out for a first version.**
   - Collection is regulated: SEC MC 18 s.2019 limits contact hours and contacts [2], and RA 11765 sets penalties of up to ₱2M and 5 years for abusive collection [23].
   - Unlicensed lending is itself an offence under RA 9474 [41].
   - Incumbent co-op systems already ship overdue letters [1][2].
   - The sector carries a harassment stigma: 47,446 online-lending complaints were reportedly logged from Aug 2024 to Jan 2026 [2].

   *Confidence: M–H.*
7. **Ranking (recommendation):**
   1. B2B service SMEs and agencies.
   2. Bookkeepers, through Kompleto, collecting their own fees first and their clients' receivables second.
   3. Small contractors (progress billings).
   4. Small private schools and tutorial centres.
   5. Domestic-client freelancers, as a free-tier funnel rather than a paying segment.

   Landlords, distributors, clinics and lenders are not recommended for the first version (§5). *Confidence: L–M. Every segment's willingness to pay for this exact product is unvalidated.*

## 1. Published evidence of late payment in the Philippines

| # | Evidence | What it shows | Type / conf. |
|---|---|---|---|
| E1 | Manila Times, 4 Jul 2026 (seen via a republication site): SMEs "commonly serve bigger companies, institutions and government projects under 30-, 60- or even 90-day payment terms"; "every 60-day invoice functions like a 60-day interest-free loan" | Long terms are the norm for SMEs selling to larger buyers | N, SR; M–L [10] |
| E2 | World Bank Enterprise Survey, Philippines 2023: 1,002 firms, Apr 2023–Feb 2024 | The official firm survey exists. Its weighted finance indicators could not be read (tables garbled in the excerpt) | OD, SR; M for the sample [9] |
| E3 | Same survey, microdata catalogue (**unweighted**): 822 of 1,002 firms (82%) said 0% of working capital was bought on credit or financed by advances from suppliers and customers. "Days to receive payment under government contracts": only 88 answers; the most common answer was 30 days (19 firms) | Formal firms in the sample report little supplier-credit financing; government payment-day data is too thin to use. The catalogue warns these are not population estimates | OD, SR; L for interpretation [8] |
| E4 | Aon Working Capital Benchmarking APAC 2026: 3,805 *listed* companies, 14 markets; average days receivable flat at 79 days in 2025 | Long collection cycles across APAC. **A PH figure (one excerpt said 49 days) could not be confirmed in a second search, so it is not used.** Listed companies are not SMEs | VC/TP, SR; M regional, L for PH [11] |
| E5 | Freelance Workers Protection bills, 20th Congress (HB 1925, 1988, 2601, 3216, 4682, 4751; 2025). Explanatory notes cite delayed or non-payment of fees; HB 1925 proposes ₱50,000–500,000 civil penalties. The 19th-Congress version passed the House on third reading but was not enacted | Legislators treat late payment of freelancers as a recognised problem. No law yet | OD, SR; M [12] |
| E6 | HB 4583 (19th Congress, Rep. Herrera) would extend RA 11032 to payments to contractors, consultants and service providers, citing delays of months to years. No general prompt-payment law for private buyers was found | Government is itself a slow payer; no statutory payment deadline protects private SMEs | OD/TP, SR; M [13] |
| E7 | Private schools: CEAP (Apr 2023) found ~49% of 224 basic-ed schools and 64.2% of 53 colleges at a loss or break-even; PACU's 27-college sample says collections cover ~7.7 months of costs; CHED released ~₱300M to 927 private schools under Bayanihan 2 to reduce unpaid tuition; one Bulacan school was cited with ~₱12M collectibles | Tuition arrears are structural and politically visible | N, SR / internal; M [1][39] |
| E8 | Supreme Court cases: a construction-materials supplier sold on 20% within 7 days and 80% within 35 days after last delivery, all by post-dated cheques, which bounced; a bakery-ingredient supplier was paid by PDCs after each delivery; contractors suspending work over unpaid progress billings; contracts charging 2%/month after 30 days | PDC-based trade credit and progress-billing disputes are common enough to reach the Supreme Court. Not a frequency measure | OD (case law), SR; M [26][1] |
| E9 | BSP Consumer Finance and Inclusion Survey 2025: 25% of adults have outstanding loans (45% in 2021); 10% of adults borrowed from informal sources (friends, colleagues, neighbours, family). 2015 baseline: 62% of borrowers borrowed from family, relatives or friends | Informal person-to-person credit is widespread. That is consumer borrowing, not business receivables | N citing OD, SR; M [17] |
| E10 | Global comparators: QuickBooks 2025 (US, vendor-commissioned) found 56% of small firms owed money, 47% with invoices >30 days overdue [4]; Remote's 2025 report says 85% of freelancers are paid late at least sometimes [14] | Useful only as an outside benchmark | VC; M–L |

**Not found:** any PH survey of the share of SME invoices paid late, average PH SME days-sales-outstanding, or DTI/PSA/BSP data on trade-credit defaults. A DICT freelancing study (2022) and a Payoneer/GCash freelancer survey (n=5,560) were found, but neither excerpt reports non-payment rates [16].

## 2. The cultural angle (*hiya*, *pakikisama*, *utang*)

**Facts (what sources actually say):**
- One word, *utang*, means both credit and debt; a 2022 Manila Times editorial calls for "breaking the stigma" of *utang* [18] (N opinion, SR; L).
- Consumer-finance blogs (MoneyMax, 2017 and 2022) say Filipinos avoid asking for money back because they fear damaging relationships with friends and relatives [19] (TP, SR; L).
- Scholars dispute translating *hiya* as "shame"; one paper prefers "sense of propriety". *Utang na loob* is a debt of gratitude, distinct from money owed [21] (TP, SR; L–M).
- A DLSU undergraduate thesis (Chin, Manzano & Santamaria, 2005) studied how *hiya*, *pakikisama* and *utang na loob* interact in the *pa-lista* (store credit) system of Laguna sari-sari stores. **Its findings could not be read** [20] (SR, title only; L).
- A handbook chapter on PH microfinance finds that delinquency management depends on a balance between social capital and management systems [44] (TP, SR; L).
- Publicly shaming a debtor online risks cyber libel, Data Privacy Act breach and unjust vexation, even when the debt is real [22] (TP legal commentary, SR; M). An NPC decision exists on an HOA that allegedly shamed delinquent owners [2].

**Hypotheses (to test in pilots, not facts):**
- H1. Owners delay or skip chasing clients who are also friends, relatives or *suki*, because a direct demand feels *bastos* (rude) or risks the relationship. A message that is drafted for them, polite, and approved rather than written from scratch lowers that cost.
- H2. Owners feel safer when the reminder "comes from the system" (a statement of account) than from them personally. Note the tension with the engine rule that reminders go out in the business's own name.
- H3. Taglish, *po*/*opo* register and face-saving phrasing ("baka po nakaligtaan lang") matter more than in English-market tools. That is a local edge over foreign products and a gap in the template-based local ones.

**Recommendation.** Do not put H1–H3 in marketing as facts. Measure them in pilots: how many overdue items the owner had never chased before Balanse, and how often the owner edits a draft to soften it.

## 3. Segments: how they chase, terms, frequency, size and price signals

Size figures come from the PSA 2023 List of Establishments (1,241,733 MSMEs) as recorded in [3], unless another source is cited. These are formal establishments only.

### 3.1 Summary table

| Segment | Typical terms (evidence) | How they chase today | Frequency of owed items | Size in PH | Local price signal |
|---|---|---|---|---|---|
| **B2B service SMEs and agencies** (IT, marketing, consulting, manpower, cleaning, security) | 30/60/90 days to larger clients [10] | Not found. *Hypothesis:* emailed invoice/SOA, then calls and Viber to the client's AP or contact person | Monthly invoices per retainer client (*hypothesis*) | Professional/scientific 17,117 + admin/support 17,401 + info/comm 11,103 = **~45,600 formal MSMEs** (my calculation; not all sell B2B on terms) [3] | Mochi ₱1,899/mo (unverified listing) [34]; First Circle claims rivals cost ₱1,500–2,500/mo (VC) and offers free reminders [33]; HitPay ₱0 [35] |
| **Freelancers** (domestic clients) | Verbal, text or chat agreements are valid contracts; guides suggest 2%/month late interest [27] | Chat and email; then a demand letter or small claims [27] | Per project | ~1.5M, a platform-derived estimate, likely a floor [15]; 42.9% of DICT-study freelancers have clients abroad [16] | Same as above; Mochi targets freelancers [34]. Willingness to pay ₱1,500/mo: not found |
| **Small contractors** (progress billings, retention) | % accomplishment billing; ~10% retention; 2%/month late interest in contracts [1] | Excel billing at night; follow-up by owner-engineer [1]. Channel *hypothesis:* Viber/text to homeowner | Per milestone, 2–10 concurrent jobs [1] | 16,528 PCAB regular licensees (2022); ~62% small categories (2017) → **~10,250 small** (my calculation, mixes years) [1]. Current count: not found (search budget) | BuildMark ₱7,500–10,000/mo for billing, no collections (PP) [1] |
| **Distributors / suppliers to sari-sari stores** | Credit (*utang*) terms; PDCs [26]; GrowSari gives stores a 5-day grace period [36] | Field agents collect on route day; orders by Viber/text [1] | Weekly route cycles (*hypothesis*) | Wholesale+retail 604,313 MSMEs combined; **wholesale-only count not found** [3] | Viber ordering sold to FMCG distributors, price not published [1]; Peddlr free POS (VC) [37] |
| **Small private schools / tutorial centres** | Instalments; dated promissory notes preserved by RA 11984 [1] | Cashier SMS, letters; Messenger to parents [1] | Monthly instalments; promissory-note dates | 47,818 public+private schools; private count not found; ~2.67M private learners (SY 2024-25) [1][40]; 14,986 education MSMEs [3] | SIS vendors, prices unpublished [1]; no PH price for a reminders layer found |
| **Landlords / boarding houses** | Rent due within first 5 days unless contract says later; max 1 month advance, 2 months deposit; 3 months' arrears (total) is a ground for ejectment (RA 9653) [28]. Rent cap 1% for 2026 on units ≤ ₱10,000 [29] | Text/Messenger, in person (*hypothesis*); apps emerging [31] | Monthly per tenant | 25.19M occupied households (2020) [30]; national renter share **not found** (search budget). Ilocos: 37,494 of 1,306,170 households rent house and lot = **2.9%** (my calculation; one region only) [30]. Real-estate MSMEs 18,297 [3] | Renthuzz free ≤ 5 units; Paabangan free plan; RentCircle (Symph) SMS reminders behind a subscription, price not shown (VD/VC) [31] |
| **Co-ops and microlenders** (regulated) | Amortisation schedules, payroll deduction [1] | Overdue letters, SMS, promise-to-pay tracking in co-op cores [1][2] | Weekly/monthly | 20,962 co-ops, 2,874 credit co-ops [1]; "over 2,500" SEC lending companies (unverified) and 401 revoked in May 2025 [41][2] | CoopVault, eKoopBanker bundle reminders [1][2]; pa-utang spreadsheet ₱249 [2] |
| **Clinics / dentists with instalments** | Braces: e.g. ₱20,000 down + ₱3,000/month at each adjustment (listing) [38] | Payment collected at the visit (*hypothesis*) | Monthly per ortho patient | 32,378 health MSMEs [3]; ortho-instalment share not found | Dental SaaS ₱1,000–2,000/mo; AI receptionist ₱750–8,900 [3] |
| **Bookkeepers: own fees** | Monthly retainers ₱2,500–15,000 per client [2] | Not found | Monthly × 10–60 clients [2] | 607 formal establishments (2018); a reported 5,358 accredited tax agents (2022); freelancers uncounted [7] | Built for Bookkeepers ₱999/mo; Kompleto ₱1,500/mo [7] |
| **Bookkeepers chasing for clients** | Clients' own terms | Not found that PH bookkeepers do this today | Varies | Each bookkeeper serves 10–60 SMEs [2] | Global: Chaser from US$259/mo, Paidnice US$69/mo [4] |

### 3.2 Segment notes

**B2B service SMEs and agencies.**
- *Facts:* larger buyers impose 30–90-day terms [10]. Mochi's chosen market (freelancers, marketing agencies, service businesses) shows a funded local entrant believes this segment pays [34]. First Circle, an SME lender, gives reminders away to attract borrowers [33].
- *Hypothesis:* the debtor is usually a company AP clerk who answers email. Replies ("processing na po", "for counter-receipt") are exactly what the engine's stop-on-reply handles.
- B2B debts fall outside consumer-collection rules abroad [6]. No PH rule specific to first-party B2B reminders was found.

**Bookkeepers (via Kompleto).**
- *Facts:* a bookkeeper bills 10–60 clients ₱2,500–15,000 each, every month [2]. Kompleto already reaches this buyer and runs the same engine [7].
- *Not found:* any evidence that PH bookkeepers chase their clients' customers today, or that they struggle to collect their own fees.
- *Hypothesis:* collecting their own retainers is the easier first use: the bookkeeper is the creditor, so there is no extra processor link. Chasing on behalf of clients adds a controller → processor → sub-processor chain like Kompleto's [7], plus the risk the bookkeeper won't let an agent write to a client's customers [4].

**Small contractors.**
- *Facts:* the clearest PH price signal is BuildMark at ₱7,500–10,000/mo for progress billing with no collections [1]. Supreme Court cases show work stopping over unpaid billings [1].
- *Risk:* few debtors per contractor (1 homeowner per job), so value per message must be high [1].

**Small private schools.**
- *Facts:* RA 11984 removed "no permit, no exam" as leverage for certified disadvantaged students but kept promissory notes, records-withholding and other legal remedies [1].
- Reminders must never cite exams, never go to group chats, and never reveal balances to anyone but the registered payer [1][22].
- Minors' data; buying is seasonal (June–August) and runs through committees [1].

**Landlords and boarding houses.**
- *Facts:* the highest-frequency, most regular obligation in this list (monthly, due day 1–5 by law) [28]. Free or near-free PH apps already do rent tracking and SMS reminders [31]. r/RentPH has ~112k members and a landlord-tips flair [32].
- *Inference:* each rent reminder is routine, so "approve every message" adds little. Tenants are consumers reached by SMS or Messenger, not email. Price anchors are near zero.

**Distributors / sari-sari suppliers.**
- *Facts:* orders and credit run through agents' personal Viber chats with no API, and Viber bots cost EUR 100/month [1]. Funded B2B platforms (GrowSari: credit lines, 5-day grace) are taking over the FMCG end [36].
- *Inference:* collection happens in person on the route, so a written reminder engine fits poorly.

**Co-ops and microlenders.** Flagged as regulated and excluded. Lending as a business requires an SEC licence (RA 9474) [41]. Collection conduct is limited by SEC MC 18 s.2019 [2] and RA 11765 [23]. Processors can be bound by NPC Circular 20-01 [2]. Incumbents bundle reminders [1][2].

**Clinics / dentists.** Health data is sensitive personal information [3]. Instalments are mostly collected at the next visit (*hypothesis*), so chasing is a minor job inside clinic software that already exists at ₱1,000–2,000/mo [3].

## 4. Free written channels per segment

All channels below are written and asynchronous, consistent with no calls, no ads and ≤ 15–20 personalised messages a week. Facebook group member counts were **not found** for any segment (Facebook is not readable by our tools [7], and the search budget ran out).

| Segment | Channels (size where found) | Notes |
|---|---|---|
| B2B service SMEs and agencies | r/phinvest ~715k (GummySearch, undated) [32]; LinkedIn 22.0M PH members [3]; DTI Negosyo Centers (1,100–1,344) [3]; Taglish SEO ("paano maningil sa client", "statement of account template") | r/phinvest skews to personal investors. Agency-owner groups: not found |
| Bookkeepers | **Kompleto's own users and pilots** [7]; r/AccountingPH ~80k, r/taxPH ~161k [7]; tax/accounting FB groups named by QuickBooks PH (sizes not found) [7] | Cheapest channel: an add-on offer inside a product they already use |
| Freelancers | r/buhaydigital ~401k (undated) [32]; freelancer FB groups (sizes not found) | Large and reachable, but a low-ticket audience |
| Small contractors | Contractor/estimator FB groups (sizes not measured) [1]; Taglish SEO, proven in this niche by AEDO's building-permit content [1]; PCAB licensee lists (public, per CIAP statistics) [1] | Possible pairing with a free progress-billing template |
| Private schools / tutorial centres | COCOPEA, CEAP, PAPSCU and regional associations (public directories) [1] | Association newsletters are gated; outreach must hit the June–August window |
| Landlords | r/RentPH ~112k [32]; "for rent" FB groups (sizes not found) | Low willingness to pay |
| Distributors | Hardware/agri-supply FB groups (not found) [1] | Weak fit, as above |
| Co-ops / lenders | CDA regional pages, NATCCO (sells its own software) [1][2] | Excluded |
| Clinics | PDA chapters, dentist FB groups (sizes not found) [3] | Not recommended |

Subreddit counts come from GummySearch pages with no visible date; another Kompleto file notes GummySearch may have closed in late 2025, so treat all of them as approximate [7][32] (TP, SR; L–M).

## 5. Ranking for a first version

Criteria (judgement): (a) evidence that money is owed on terms, often; (b) debtor reachable by **email**, the engine's sending channel (SMS links are blocked [42], Viber bots cost money [1]); (c) low data sensitivity and no regulated collection; (d) a buyer who already pays for software near ₱1,500/mo; (e) a free written channel; (f) room against free local reminder tools.

| Rank | Segment | Why (one line) | a | b | c | d | e | f |
|---|---|---|---|---|---|---|---|---|
| **1** | B2B service SMEs and agencies | 30–90-day terms to companies with AP inboxes; replies are routine; a local entrant targets them | M | H | H | M | M | L–M |
| **2** | Bookkeepers via Kompleto (own fees first, then clients' receivables) | Warm channel, same engine, monthly recurring invoices × 10–60 clients | L | H | M | H | H | M |
| **3** | Small contractors | Highest value per overdue item; strongest PH software price (₱7.5–10k, no collections) | M | M | H | H | M | H |
| **4** | Small private schools / tutorial centres | Legal driver (RA 11984 keeps promissory notes); structural arrears | M–H | L–M | L | L | M | H |
| **5** | Domestic-client freelancers (free tier / funnel only) | Large, reachable, but low willingness to pay and many are paid through platforms abroad | L | H | H | L | H | L |

Not recommended now: **landlords** (routine, free apps, consumer SMS channel), **distributors** (Viber and in-person collection), **clinics** (sensitive data, minor job), **co-ops and microlenders** (regulated, incumbent-served, stigma).

### Kill-risks per ranked segment

**1. B2B service SMEs and agencies**
- K1. Free substitutes win: First Circle's free unlimited reminders [33] or HitPay at ₱0 [35] are "good enough", and owners won't pay ₱1,500 for approval plus Taglish drafts.
- K2. Reminders don't move payment, because corporate clients pay on fixed AP cycles (counter-receipt, check-release days) whatever the reminder says. *Hypothesis; would show as unchanged days-to-pay in pilots.*
- K3. Too few genuinely B2B-on-terms firms among the ~45,600 (no split found), and owners are unreachable without a network.
- K4. Owners won't let any tool email their biggest client (relationship fear: H1 cuts both ways).

**2. Bookkeepers via Kompleto**
- K1. Bookkeepers don't chase clients' receivables at all, and their own fees are collected without friction (both not found).
- K2. Chasing on behalf of clients adds a sub-processor role and liability for messages sent in the client's name [7].
- K3. Cannibalises or complicates Kompleto's ₱1,500 offer: a bookkeeper won't pay twice, so it may have to be a Kompleto add-on, not a separate product.
- K4. Kompleto has no pilots yet, so this channel is unproven [7].

**3. Small contractors**
- K1. Too few debtors per account (1 homeowner per job), so the tool is used a few times a month [1].
- K2. Contractor–homeowner talk lives in Viber and site visits, not email (*hypothesis*).
- K3. BuildMark adds reminders [1].
- K4. Disputes are about work quality and change orders, not forgetfulness, so polite reminders don't help.

**4. Private schools**
- K1. Minors' and household financial data raises Data Privacy Act exposure; NPC complaints [1].
- K2. One buying window a year, committee decisions [1].
- K3. Parents are reached by SMS/Messenger, not email.
- K4. SIS vendors bundle reminders [1].
- K5. RA 11984 misuse risk if a draft mentions exams [1].

**5. Freelancers**
- K1. Willingness to pay is far below ₱1,500.
- K2. Platform-paid freelancers never chase.
- K3. Mochi and First Circle already target them [33][34].

### Cheapest no-call tests (recommendation)

1. Publish a free Taglish "polite payment reminder" generator page (no login; drafts only), with SEO for "paano maningil ng utang sa client". Count which segment uses it, from a dropdown. *Kill if* fewer than 30 uses a week after 6 weeks of posting, or B2B users are under 25%.
2. Offer "chase your own unpaid fees" free to Kompleto pilot bookkeepers. *Kill the bookkeeper route if* fewer than 2 of 3–5 pilots switch it on.
3. In every pilot, log three numbers: overdue items never chased before (tests H1), drafts edited to soften (tests H3), and days-to-pay before vs after (tests K2).

## Evidence gaps (open)

- PH SME late-payment rate, average DSO, or a trade-credit default survey: **not found**.
- Weighted World Bank Enterprise Survey 2023 finance indicators: exist [9], **not retrieved** (fetch blocked).
- The Aon 2026 PH days-receivable figure: **not confirmed** [11].
- How each segment chases today: **no survey found**. Every "how they chase" cell above is a hypothesis unless a source is cited.
- Willingness to pay for approval-gated reminders in any PH segment: **not found**. Mochi's ₱1,899 is unverified [34]; First Circle's ₱1,500–2,500 "competitor cost" is a vendor claim [33].
- National renter share, current PCAB contractor count, Facebook group sizes: **not found** (search budget exhausted).
- Findings of the DLSU *hiya*/*pa-lista* thesis: **not read** [20].
- Whether PH bookkeepers chase receivables for clients: **not found**.

## Sources

Format: publisher, title, date, URL; evidence type; confidence. All accessed 2026-10-24. Internal sources keep the evidence types and confidence recorded in those files.

1. Kompleto research, [Kompleto `docs/research/ph-industries-sectors.md`](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/research/ph-industries-sectors.md) (§2 education, §4 construction, §6 co-ops, §7 retail/distributors; CON1, EDU1, COOP1, DIS1), 2026-10-18; internal synthesis; M.
2. Kompleto research, [Kompleto `docs/research/ph-industries-services.md`](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/research/ph-industries-services.md) (§2 bookkeepers, §4 lenders/co-ops incl. SEC MC 18 s.2019, NPC 20-01, PAOCC figure, ₱249 template; §5 HOA/NPC shaming decision), 2026-10-18; internal; M.
3. Kompleto research, [Kompleto `docs/research/market-philippines.md`](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/research/market-philippines.md) (PSA 2023 List of Establishments table, channel reach, price ceiling, clinics), 2026-10-18; internal; H for PSA figures, M otherwise.
4. Kompleto research, [Kompleto `docs/research/opportunities-business.md`](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/research/opportunities-business.md) §3.6 (AR collections; QuickBooks 2025 late-payments report; Chaser, Paidnice), 2026-10-18; internal; M.
5. Kompleto research, [Kompleto `docs/research/opportunity-scorecard.md`](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/research/opportunity-scorecard.md) §7 (candidate C failed the constraints: Xero listing cost, no free channel), 2026-10-18; internal; M.
6. Kompleto research, [Kompleto `docs/research/self-serve-distribution.md`](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/research/self-serve-distribution.md) (FDCPA scope, first-party B2B reminders, Xero PH Lite 5-invoice cap), 2026-10-18; internal; M–H.
7. Kompleto research, [Kompleto `docs/research/p1-kill-checks.md`](https://github.com/Enriquez-Johnmark/kompleto/blob/main/docs/research/p1-kill-checks.md) (§3 reachability: r/AccountingPH, r/taxPH, PSA bookkeeping counts, tax-agent list; §4 privacy chain; §5 NTC SMS-link block, Viber), 2026-10-20; internal; M.
8. World Bank Microdata Library, "Philippines – Enterprise Survey 2023" (catalog 6464; variable pages for working capital on credit and days to receive government payment), https://microdata.worldbank.org/catalog/6464 ; OD; SR; M existence, L interpretation (unweighted).
9. World Bank Enterprise Surveys, "What businesses experience: Philippines 2023 Country Profile", https://www.enterprisesurveys.org/content/dam/enterprisesurveys/documents/country/Philippines-2023.pdf ; OD; SR; M (sample and dates only).
10. The Manila Times, "The problem of delayed payments in SMEs", 2026-07-04, via republication https://newswav.com/article/the-problem-of-delayed-payments-in-smes-A2607_Aag3MC ; N; SR; M–L.
11. Aon, "APAC businesses wait 79 days for payment as cash flow pressures persist: Aon study", 2026-08-17, https://www.aon.com/apac/in-the-press/asia-newsroom/2026/apac-businesses-wait-79-days-for-payment-as-cash-flow-pressures-persist-aon-study ; Insurance Business Asia, "Cash flow squeeze persists across Asia-Pacific", 2026, https://www.insurancebusinessmag.com/asia/news/breaking-news/cash-flow-squeeze-persists-across-asiapacific-new-aon-benchmarking-shows-586512.aspx ; VC/TP; SR; M regional, L PH.
12. House of Representatives (20th Congress), Freelance Workers Protection bills, e.g. HB 2601 https://docs.congress.hrep.online/legisdocs/basic_20/HB02601.pdf , HB 1925 https://docs.congress.hrep.online/legisdocs/basic_20/HB01925.pdf , HB 3216 https://docs.congress.hrep.online/legisdocs/basic_20/HB03216.pdf , 2025; Inquirer, "Bill safeguarding freelancers gets House panel nod", 2022, https://newsinfo.inquirer.net/1666718/bill-safeguarding-freelancers-gets-house-panel-nod ; OD/N; SR; M.
13. House of Representatives (19th Congress), HB 4583, https://docs.congress.hrep.online/legisdocs/basic_19/HB04583.pdf ; Respicio & Co., "Laws governing government agency payments to suppliers", https://www.respicio.ph/commentaries/laws-governing-government-agency-payments-to-suppliers-deadlines-interest-and-remedies-in-the-philippines ; OD/TP; SR; M.
14. Remote, "Paying freelancers late has become the norm…" (Contractor Management Report 2025), https://remote.com/blog/contractor-management/reversing-late-payment-culture ; Bonsai, "How often do freelancers get paid late?", https://www.hellobonsai.com/blog/late-freelance-payment ; VC; SR; L–M.
15. Rappler, "Freelancer market growing in the Philippines – PayPal", https://rappler.com/business/freelancer-market-growing-philippines-paypal ; Manila Bulletin, "The Filipino online freelancers: digital workforce of the world", 2025-01-07, https://mb.com.ph/2025/1/7/the-filipino-online-freelancers-digital-workforce-of-the-world ; N; SR; L–M.
16. PIA, "DICT launches the PH Online Freelancing Industry Study", 2022-09-02, https://mirror.pia.gov.ph/news/2022/09/02/dict-launches-the-ph-online-freelancing-industry-study ; Payoneer & GCash, "The Philippines Freelance Market 2022", https://pubs.payoneer.com/docs/payoneer-gcash-philippines-freelance-report-2022.pdf ; OD/VC; SR; M.
17. GMA News, "BSP: Filipino account ownership down to 50% in 2025, household access up to 85%", 2025/2026, https://www.gmanetwork.com/news/money/economy/984131/bsp-filipino-account-ownership-down-to-50-in-2025-household-access-up-to-85/story/ ; Inquirer Business, "Most Filipino borrowers go to informal sources for money", 2015, https://business.inquirer.net/?p=192542 ; N citing BSP surveys; SR; M.
18. The Manila Times, editorial "Breaking the stigma of utang", 2022-02-02, https://manilatimes.net/2022/02/02/opinion/editorial/breaking-the-stigma-of-utang/1831488/amp ; N (opinion); SR; L.
19. MoneyMax, "5 Ways to Get Your Money Back From Friends", https://www.moneymax.ph/personal-finance/articles/effective-ways-ask-money-back ; "May extra money ka ba diyan? Pautang naman", https://www.moneymax.ph/personal-finance/articles/how-to-say-no-loan-money ; TP (comparison-site blog); SR; L.
20. Chin, Manzano & Santamaria, DLSU bachelor's thesis on Hiya, Pakikisama and Utang na Loob in the pa-lista mode of payment, Laguna sari-sari stores, 2005, https://animorepository.dlsu.edu.ph/etd_bachelors/12828 ; TP (academic, undergraduate); SR (title and abstract line only); L.
21. UP Diliman Main Library repository, "Priests and the Filipina Hiya Norm", https://repository.mainlib.upd.edu.ph/omekas/s/rare-periodicals/media/260591 ; Wikipedia, "Utang na loob", https://en.wikipedia.org/wiki/Utang_na_loob ; TP; SR; L–M.
22. Respicio & Co., "Legal remedies for online debt shaming and cyber libel", https://www.respicio.ph/commentaries/legal-remedies-for-online-debt-shaming-and-cyber-libel ; Lawyer-Philippines, "Legal consequences of public shaming for unpaid debts on social media", https://www.lawyer-philippines.com/articles/legal-consequences-of-public-shaming-for-unpaid-debts-on-social-media-in-the-philippines ; TP (legal commentary); SR; M.
23. ABS-CBN News, "6 companies, 32 individuals face abusive collection raps", 2023-06-20, https://news.abs-cbn.com/news/06/20/23/6-companies-32-individuals-face-abusive-collection-raps ; N; SR; M.
24. Supreme Court rules via DLSU Law, "Rules on Expedited Procedures in the First Level Courts – primer", https://www.dlsu.edu.ph/wp-content/uploads/pdf/law/primer/rules-on-expedited-procedures-in-first-level-courts-primer.pdf ; Batas Natin, A.M. No. 08-8-7-SC, https://batasnatin.com/laws/rule-of-procedure-for-small-claims-cases-am-no-08-8-7-sc ; OD via TP; SR; M–H (one commentary gives a conflicting ₱400,000 figure, judged an error).
25. Supreme Court E-Library, decision on Katarungang Pambarangay as precondition (LGC §412), https://elibrary.judiciary.gov.ph/thebookshelf/showdocs/1/32002 ; Miguel v. Montanez, G.R. No. 191336, 2012-01-25, https://batasnatin.com/laws/gr-191336 ; OD (case law); SR; H for the principle.
26. John Dy v. People, G.R. No. 158312, 2008-11-14, https://batasnatin.com/laws/gr-158312 ; Supreme Court decision (Dec 2015) via ChanRobles, https://chanrobles.com/cralaw/2015decemberdecisions.php?id=1072 ; Daily Tribune legal column on BP 22, 2026-09-01, https://tribune.net.ph/2026/09/01/bp22 ; OD/N; SR; M (which case holds which fact was not separable in the excerpt).
27. Digest.ph, "The freelancer's legal guide to making sure your client pays", 2023, https://www.digest.ph/blog/the-freelancers-legal-guide-to-making-sure-your-client-pays ; TP; SR; M.
28. Republic Act No. 9653 (Rent Control Act of 2009), Senate text, https://legacy.senate.gov.ph/republic_acts/ra%209653.pdf ; Batas Natin, https://batasnatin.com/laws/ra-9653 ; OD; SR; H for text (current extension status via [29]).
29. NHSB Resolution No. 2024-01 (rent control 2025–2026), https://philpropertyexpert.com/public/NHSB_Resolution_2024-01_Rent-Control-2025-2026.pdf ; Inquirer, "Gov't cuts rent cap for units leased at P10,000 and below", https://newsinfo.inquirer.net/2021978/govt-cuts-rent-cap-for-units-leased-at-p10000-and-below ; OD/N; SR; M–H.
30. PSA, "Philippines recorded about 28.5 million housing units in 2020", https://psa.gov.ph/statistics/population-and-housing/node/1684059631 ; PSA RSSO I, "Majority of households in Ilocos Region own/amortize their housing units/lots (2020 Census)", https://rsso01.psa.gov.ph/content/majority-households-ilocos-region-ownamortize-their-housing-unitslots-2020-census ; OD; SR; H.
31. Product Hunt, Renthuzz, https://www.producthunt.com/products/renthuzz ; Apple App Store (PH), RentCircle by Symph, https://apps.apple.com/ph/app/rentcircle/id6651860576 ; Chrome-Stats, Paabangan – Rental Manager PH, https://chrome-stats.com/d/com.paabangan.mobile ; VD/VC; SR; M.
32. GummySearch pages, r/RentPH https://gummysearch.com/r/RentPH/ , r/buhaydigital https://gummysearch.com/r/buhaydigital , r/phinvest https://gummysearch.com/r/phinvest/ (undated); TP; SR; L–M.
33. First Circle, "Invoice", https://www.firstcircle.ph/invoice ; "Terms & Conditions for E-Invoicing" (not a BIR-accredited e-invoice), https://www.firstcircle.ph/company/terms-conditions-for-e-invoicing ; VC/VD; SR; M.
34. The Independent Investor, "Mochi launches hassle-free invoicing for Filipino SMEs with new accounts receivable platform", https://theindependentinvestor.ph/mochi-launches-hassle-free-invoicing-for-filipino-smes-with-new-accounts-receivable-platform/ ; PayMongo blog, "PayMongo × Mochi", 2025-04-10, https://www.paymongo.com/blog/paymongo-mochi-powering-business-growth ; Dealroom profile (AI-generated summary; ₱1,899/mo), https://app.dealroom.co/companies/mochi_4 ; N/VC/TP; SR; M existence, L price.
35. HitPay, "Best invoicing software Philippines: comparison", https://hitpayapp.com/blog/best-invoicing-software-philippines-comparison ; VC (vendor-authored comparison); SR; L–M.
36. UNSGSA, "Empowering local commerce: GrowSari", https://www.unsgsa.org/stories/empowering-local-commerce-fintech-growsaris-digital-platform-gives-boost-small-merchants-philippines ; Wavemaker, "Spotlight on B2B startup GrowSari", https://wavemaker.vc/sea/transforming-traditional-trade-spotlight-on-b2b-startup-growsari ; VC/TP; SR; M.
37. GetApp, Peddlr POS, https://www.getapp.com/retail-consumer-services-software/a/peddlr-pos/ ; VC; SR; L–M (utang feature not confirmed).
38. ClinicFinder PH, "Presyo ng braces sa Pilipinas", https://www.clinicfinderph.com/blog/presyo-braces-philippines ; WhatClinic, "Braces Philippines", https://www.whatclinic.com/dentists/philippines/braces?page=1 ; TP/directory; SR; L–M.
39. Inquirer, "Almost 55,000 private school students received P5K cash aid amid pandemic – CHED", https://newsinfo.inquirer.net/1383262/almost-55000-private-school-students-received-p5k-cash-aid-amid-pandemic-ched/amp ; 2023 coverage of the "no permit, no exam" debate (Inquirer, Philstar, GMA, UCA News results; the article citing the ₱12M Bulacan school was not identified), e.g. https://www.ucanews.com/amp/philippine-bill-to-allow-exams-without-tuition-fees-sparks-row/102369 ; N; SR; M (CHED), L (Bulacan figure).
40. Inquirer, "DepEd still looking to hit 27M enrollment target", 2024-08, https://newsinfo.inquirer.net/1970100/deped-still-looking-to-hit-27m-enrollment-target ; N citing DepEd; SR; M.
41. Lawyer-Philippines, "Status of lending company registration in the Philippines" ("over 2,500" registered, unverified), https://www.lawyer-philippines.com/articles/status-of-lending-company-registration-in-the-philippines ; Kapronasia, "Philippines SEC increases scrutiny of lending platforms", https://kapronasia.com/insight/blogs/fintech-research/philippines-sec-increases-scrutiny-of-lending-platforms ; Respicio & Co. on RA 9474, https://www.respicio.ph/commentaries/how-to-verify-registration-of-lending-companies-in-the-philippines ; TP; SR; L.
42. Globe Telecom newsroom, "Globe lauds NTC memo requiring telcos to block SMS with clickable links", 2023-10-18 (memo dated 2023-09-27), https://www.globe.com.ph/about-us/newsroom/corporate/globe-lauds-ntc-memo-requiring-telcos-to-block-sms-with-clickable-links ; N/VC; SR; M (A2P scope not confirmed).
43. (Reserved: Facebook group sizes for agencies, contractors, landlords, school administrators; searched, not found.)
44. Routledge handbook chapter on Philippine credit sources and microfinance delinquency, https://test.routledgehandbooks.com/doi/10.4324/9781315709215-24 ; TP; SR; L.
