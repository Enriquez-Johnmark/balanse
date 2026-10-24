# Balanse: legal and regulatory limits

Status: **research complete, 2026-10-24. Not owner-approved. Not legal advice.** This is desk research by a non-lawyer for product design. A Philippine lawyer should review it before paid launch.

Scope: what Philippine law allows when a business owner (or their bookkeeper) uses Balanse to draft, approve and send payment reminders to people who owe them money, and which rules Balanse should enforce in code. Tags like [3] point to the Sources list. Internal sources are [I1]–[I5] and are not redone here.

**Research limits in this pass.** WebFetch failed on every host (DNS error) and the session proxy refused direct downloads (403), so **no web page was read in full**. Every web source below is a search-result excerpt (**SR**), including primary texts on bsp.gov.ph, privacy.gov.ph and the Supreme Court E-Library. The web-search budget ran out before RA 8792 (e-commerce) and two RA 11765 checks; those gaps are marked "not found". Confidence is therefore capped at M for most web claims.

Evidence types: OD official document/regulator, VD vendor docs, N news, TP third party (law firms count as TP), SR search-result excerpt only. Confidence H/M/L. Labels: **Fact** (what a source says), **Interpretation** (our reading), **Recommendation** (what Balanse should do).

## Key conclusions

1. **Nothing found kills Balanse.** The debt-collection conduct rules (SEC MC 18-2019, RA 11765, BSP Circular 1160) bind lenders and other regulated financial service providers, not ordinary trade creditors or the software they use. *(Interpretation, M.)*
2. **Balanse should still follow those rules as product rules.** Courts and the NPC judge collection conduct by the same standards through the Civil Code (abuse of rights), the Data Privacy Act, cyberlibel and unjust vexation. The regulators' lists are the clearest available standard. *(Recommendation.)*
3. **Exclude lending and financing businesses as customers.** If a lender, financing company, online lending app or informal lender uses Balanse, MC 18, RA 11765 and NPC Circular 20-01 apply directly, and Balanse could look like its collection service provider. *(Recommendation; MC 18 covers third-party collection providers [1][3].)*
4. **Balanse is a personal information processor (PIP) for each creditor.** The creditor is the controller. Kompleto's privacy work (DPA terms, sub-processor list, 72-hour breach chain, NPC registration threshold) carries over [I1][I2].
5. **Never contact anyone but the debtor**: no family, references, employer, group chats, CC or public posting. This is banned for lenders [1][15][16] and is the core of "debt shaming" cases under privacy and libel law [18][25].
6. **Do not compute interest or penalties.** Conventional interest needs a written stipulation (Art. 1956); legal interest (6%) runs from demand (Arts. 1169, 2209; MB Circular 799). A wrong computed figure is a "false representation" risk. Show only amounts the creditor typed [22][23][24].
7. **The biggest BIR grey zone is CAS, not invoicing.** RMO 9-2021 names software serving "collection" facilities and systems recording "Accounts Receivable" as CAS components [I1]. Balanse must stay a follow-up list, never the creditor's receivables ledger, and must never issue invoices, receipts, statements of account or "billing invoices" [30][31][I3].
8. **Linking only to the creditor's own GCash/Maya/bank QR or their own gateway account keeps Balanse outside the payment-system rules** on the definitions found. Holding funds, collecting for many merchants under one account, or processing payments on a creditor's behalf would bring it in [34][35]. *(Interpretation, M.)*
9. **No Philippine anti-spam statute for email was found**: the Cybercrime Act's spam clause was struck down in 2014 [26]. SMS is the regulated channel: registered sender IDs, no links, opt-out keywords, quiet hours [36][37][I1].

## 1. Debt-collection conduct rules

### Facts

**SEC Memorandum Circular No. 18, s. 2019** ("Prohibition on Unfair Debt Collection Practices of Financing Companies and Lending Companies"). Issued 19 Aug 2019, effective 7 Sep 2019 [2][5] (N, SR, M). It covers financing and lending companies and the third-party service providers they hire to collect, which are treated as their agents [3][4] (TP, SR, M). Prohibited acts reported in summaries [2][3][4][6] (TP/N, SR, M; exact lettering not verified):
- use or threat of violence or other criminal means to harm the person, reputation or property of the borrower (one summary adds "or their family");
- threats to take action that cannot legally be taken;
- obscenities, insults or profane language;
- publication of names and other personal information of borrowers ("debt shaming");
- communicating, or threatening to communicate, loan information known or which should be known to be false, including failing to disclose that the debt is disputed;
- any false representation or deceptive means to collect, or to obtain information about a borrower;
- contact at unreasonable or inconvenient times: **before 6:00 a.m. or after 10:00 p.m.**, unless the account is more than 15 days past due or the borrower expressly consented (the exception is reported by one news summary and one unverified site [2][6], L–M);
- contacting persons in the borrower's contact list other than named guarantors or co-makers, even with the borrower's consent [2] (N, SR, M).
- §4 requires collectors to disclose their true identity [6] (TP, SR, L–M).

Penalties reported: from ₱25,000 (lending) / ₱50,000 (financing) for a first offence up to ₱1,000,000 and licence revocation [2] (N, SR, M). The SEC still enforces it: a ₱50,000 fine on Global Dominion Financing for breaching §1(A), (B) and (H) (reported Feb 2026), and 2023 cease-and-desist orders citing MC 18 together with **§4.4 of the RA 11765 IRR**, which "prohibits financial service providers from making use of threats … as well as contacting persons in the borrower's contact list other than those … named as guarantors or co-makers" [7] (N, SR, M). One commentary says MC 18 was consolidated into "SEC MC 32-2024"; not confirmed by any other source [7] (SR, L).

**RA 11765, Financial Products and Services Consumer Protection Act (2022).** §3(e): a financial service provider is "a person, natural or juridical, which provides financial products or services that are under the jurisdiction of financial regulators" (BSP, SEC, IC, CDA). §3(c) lists "savings, deposits, credit, insurance, … payments, remittances and other similar products" [8] (OD, SR, M–H). The Act states that financial service providers "are prohibited from employing abusive collection or debt recovery practices against their financial consumers" [8] (OD, SR, M); the section number differs between sources and was not confirmed. Each regulator issued its own IRR (e.g. Insurance Commission IMC 2023-01, CDA MC 2023-14) [9][10] (OD/TP, SR, M).

**BSP Circular No. 1160** was issued **28 Nov 2022** (not 2023) and implements RA 11765 for BSP-supervised institutions by amending the bank and non-bank manuals [11][12] (OD/TP, SR, M). Summaries say it bars supervised institutions and their collection agencies from threats, profanity and insults [13] (TP, SR, M). **Its enumerated list of collection acts was not found** in this pass. The 6 a.m.–10 p.m. window for BSP-supervised credit-card collection is older (Circular 454, 2004; a 60-day past-due exception in later wording); sources conflict on the numbers [14] (TP, SR, L).

### Interpretation

- **Ordinary trade creditors are outside all three.** A supplier, contractor, landlord or school extending payment terms is not a lending or financing company (MC 18) and does not provide financial products "under the jurisdiction of financial regulators" (RA 11765 §3(e)) [8]. No source said otherwise; no source addressed trade creditors directly [7] (M).
- **Balanse as a tool is outside them too**, as long as its customers are not regulated lenders. MC 18 reaches third-party collection providers hired by lenders [3]; a reminder tool used by a lender could be argued to be one (L–M).
- **Grey cases:** cooperatives are CDA-regulated, so a credit or multi-purpose cooperative chasing loan arrears is a financial service provider [8][10]; a cooperative selling goods on credit is unclear (not found). BNPL sellers, pawnshops, "5-6" lenders and in-house financing (e.g. real-estate in-house financing) are lenders in substance.

### Recommendation

Exclude lenders, financing companies, online lending apps, pawnshops, credit cooperatives' loan collection and informal money-lending in the terms of service and with a sign-up question. Adopt the MC 18 list as Balanse's conduct standard for everyone (see §8).

## 2. Data Privacy Act (RA 10173) and the NPC

### Facts

- **Roles.** A PIP is a person "to whom a personal information controller may outsource the processing of personal data" (RA 10173 §3(i)). Outsourcing to a PIP is not "data sharing", so the IRR §44 outsourcing contract terms apply, not the data-sharing circular [I1] (statute/IRR, M–H).
- **Lawful basis.** NPC treats processing "necessary for the establishment, exercise, or defense of legal claims" as a possible legitimate interest under §12(f) (Advisory Opinion 2025-003); in AO 2025-002 (asset searches on defaulting bank borrowers) the NPC analysed the bank's claim under §12(f) [19] (OD, SR, M). NPC Circular 2023-07 requires a documented three-part legitimate-interest assessment (purpose, necessity, balancing), compliance by 13 Apr 2024 [20] (TP, SR, M). Commentary also names contract fulfilment (§12(b)) as a ground for debt-collection processing [19] (TP, SR, L–M). No NPC opinion applying §12(b) to collection was found.
- **NPC Circular 20-01 (14 Sep 2020), amended by NPC Circular 2022-02**, covers personal data processed for loan-related transactions by lending and financing companies and "any natural or juridical person" acting as such, SEC-authorised or not [15][16] (OD, SR, M). It bars requiring access to phone, email or social-media contact lists; character references may be contacted only to verify the application, not to collect; §5(B) as amended "expressly prohibit[s]" contacting persons in the borrower's contact list "other than those who were declared as guarantors" [16][17] (OD/N, SR, M).
- **Public posting.** NPC Advisory Opinion 2023-001 treated a condominium's plan to post delinquent units and amounts as processing of personal information subject to §12 and to transparency, proportionality and legitimate purpose; **its conclusion was not retrieved** [18] (OD, SR, L–M). Kompleto's sector research recorded legal commentary that posting debtors' names in school group chats is a likely DPA violation [I5].
- **Registration, breach, DPO**: NPC Circular 2022-04 (register at ≥ 250 employees, SPI of ≥ 1,000 individuals, or processing likely to pose a risk to data subjects; otherwise a sworn declaration of exemption), the 72-hour breach rule (IRR §38, Circular 16-03), and the founder as DPO are covered in [I1] §4 and [I2] §3 and are not repeated.

### Interpretation

- **The creditor is the controller; Balanse is its PIP** for debtor data (name, contact, amount, due date, replies). Balanse is a controller only for its own users' account data. This mirrors Kompleto [I1] (M).
- **A creditor has a lawful basis to remind its own debtor**: contract (§12(b)) for the debtor's own contract, or legitimate interest (§12(f)) documented per Circular 2023-07 [19][20] (M). That basis does **not** extend to the debtor's relatives, references or employer: contacting them fails the necessity and balancing tests, and for lenders it is expressly banned [16] (M).
- **Debt amounts are not "sensitive personal information"** under §3(l) as summarised in [I1] (M). Balanse stays below the SPI-of-1,000 trigger only if it never collects TINs, government IDs or similar. Whether debt data counts as processing "likely to pose a risk" under 2022-04 was not found; treat it as open.
- **Schools:** debtors are parents but the subject is a minor. Address the parent or guardian only, never the student [I5] (interpretation, M).

### Recommendation

Reuse Kompleto's privacy package (DPA with the nine IRR §44 clauses, sub-processor list, 24-hour processor-to-controller breach notice, DPO designation, sworn declaration until registration) [I1][I2][I4]. Give each creditor a one-page legitimate-interest assessment template. Put a short notice line in every message naming the creditor as controller and linking a privacy notice (RA 10173 §16 right to be informed, [I2]). Collect only name, contact channels, amount, due date and the creditor's own reference.

## 3. Civil Code: demand, delay and interest

### Facts

- **Art. 1169**: "Those obliged to deliver or to do something incur in delay from the time the obligee judicially or extrajudicially demands from them the fulfillment of their obligation" [22] (TP quoting statute, SR, H for the text). Exceptions in commentary: the obligation or law says demand is unnecessary, time is of the essence, or demand would be useless [22] (TP, SR, M).
- **Proof of demand matters.** In G.R. No. 233774 (2019) the respondent could not be considered in default for failing to prove the demand; the registry return card was not formally offered in evidence [22] (OD court decision, SR, M).
- **Email as demand**: no Supreme Court ruling found. One commentary says electronic demand may be valid under RA 8792 "if agreed upon"; its full analysis was not retrieved [22] (TP, SR, L). RA 8792's functional-equivalence text: **not checked** (search budget).
- **Interest.** Art. 1956: "No interest shall be due unless it has been expressly stipulated in writing." Art. 2209: for delay in a money obligation the indemnity is the agreed interest or, absent agreement, legal interest [23] (TP, SR, M–H). **Legal interest is 6% a year from 1 Jul 2013** under BSP-MB Circular 799 (2013), applied prospectively and embodied in *Nacar v. Gallery Frames* (G.R. No. 189871, 2013), as restated in *Odiamar v. Valencia* (G.R. No. 213582, 2018) [24] (OD court decision, SR, H). Courts may reduce iniquitous or unconscionable penalties (Art. 1229, per commentary) [23] (TP, SR, M).

### Interpretation

- A dated, delivered email or SMS that asks for payment of a stated amount is plausibly an extrajudicial demand, but it is unsettled; a reminder worded as a soft nudge ("baka nakaligtaan lang po") may not read as a demand at all (L–M).
- If Balanse computes interest or penalties it must know whether a written stipulation exists, the agreed rate, the date of default (which depends on demand) and whether a penalty is unconscionable. Getting any of these wrong puts a false amount in a message, which is the MC 18 "false information" harm and an abuse-of-rights risk [1][28] (M). Computing running balances also moves Balanse towards generating accounting records (§5).

### Recommendation

- **Do not compute interest, penalties or late fees.** Show only amounts the creditor entered, labelled as from the creditor's records, with the creditor's own invoice or contract reference.
- Keep Balanse's audit trail (approved text, send time, provider message ID, delivery and reply events) exportable per debt, so a creditor can prove what was sent and when. Do not market reminders as legal demand letters.
- Owner decision for later: whether to offer a clearly labelled "formal demand" template, which would need lawyer-reviewed wording.

## 4. Harassment and defamation risk

### Facts

- **Cyberlibel**: RA 10175 §4(c)(4) punishes libel (RPC Art. 355) committed through a computer system, one degree higher than ordinary libel; sources conflict on the resulting range [25] (TP, SR, M). *Disini v. Secretary of Justice* (G.R. No. 203335, 18 Feb 2014) upheld online libel for the original author [26] (TP, SR, M).
- **Unjust vexation**: RPC Art. 287 second paragraph covers conduct that "unjustifiably annoys or vexes" another; a positive act is required; the fine is ₱1,000–₱40,000 and/or arresto menor (1–30 days) after RA 10951 (2017) [27] (TP, SR, M). No case on collection texts was found.
- **Civil Code Arts. 19, 21, 26**: Art. 19 is a "primordial limitation on all rights"; a right exercised against it that causes damage is a legal wrong (*Unicapital v. Consing*, G.R. No. 175277); abuse of rights needs a right, bad faith and intent to prejudice (*Navarro-Banaria v. Banaria*, G.R. No. 217806) [28] (TP/OD summaries, SR, M). Commentary: a valid debt does not by itself show good faith; courts look at how, how often and to whom contacts were made [28] (TP, SR, L–M).
- **RA 11313 Safe Spaces Act §12** (gender-based online sexual harassment) covers online threats and unwanted sexual, misogynistic, homophobic or sexist remarks; no source applies it to collection [29] (OD/TP, SR, M).
- **RA 12010 (AFASA, 20 Jul 2024)** punishes social-engineering schemes that obtain financial-account credentials by deception, including posing as an institution through electronic communications [42] (OD/TP, SR, M).

### Interpretation

- Libel needs publication to a third person, so one-to-one messages to the debtor carry much less libel risk than group chats, CCs, posts or messages to relatives (M). Repetition and tone, not the existence of the debt, drive unjust-vexation and Art. 19/21 exposure (M).
- No source gives a legal maximum number of reminders. Any frequency cap is a product choice, not a legal line (not found).
- Payment-request messages with links look like scams to recipients and telcos; Balanse must never resemble an AFASA-style scheme (M).

### Recommendation: wording Balanse templates and the AI must never produce

- Threats: arrest, jail ("ipapakulong", "makukulong"), police, NBI, barangay blotter, court case ("kakasuhan"), lawyer, "blacklist", "credit record", "final legal notice", unless the creditor types it themselves (and see the open question on legal-action wording).
- Labels: scammer, manloloko, magnanakaw, estafa, "walang hiya", any insult or profanity; anything sexual, gendered or about appearance.
- Shaming: "ipopost", "sasabihan namin ang pamilya/amo mo", mentions of relatives, employer or neighbours.
- Impersonation: posing as a lawyer, court, government office or collection agency.
- Scam signals: requests for OTPs, PINs or passwords; payment to an account other than the creditor's registered one; shortened links.

## 5. BIR: invoices, statements and CAS

### Facts

- **Invoices.** Under RA 11976 and RR 7-2024 the invoice is the primary proof of sale for goods and services; official, collection and payment receipts are supplementary [30] (OD/TP, SR, M–H). "Supplementary documents" include delivery receipts, purchase orders, collection receipts, **billing statements and statements of account**; they are not valid proof for input-tax claims and must carry the legend "THIS DOCUMENT IS NOT VALID FOR CLAIM OF INPUT TAX" [30] (TP, SR, M). RMC 77-2024 (digest): issuing a billing statement or statement of account "without the corresponding Invoice or Billing Invoice" on a sale is non-compliance [30] (OD, SR, M). RR 7-2024 lets taxpayers convert billing statements and statements of account into "Billing Invoices"; RR 11-2024 lets converted stock be used until consumed [30][31] (OD/TP, SR, M).
- **CAS.** RMO 9-2021 defines CAS components as software "adopted to generate accounting records, reports and/or documents, which includes … ordering, booking, **collection**, receipting/invoicing and other related business transaction facilities", and scopes in non-large taxpayers who opt to use systems that record "Sales, Purchases, **Accounts Receivable**, Accounts Payable, Inventory, and Payroll" [I1] (OD, primary read, H). No successor to RMO 9-2021 was found; later RR 11-2025 (amended by RR 26-2025) adds e-invoicing data and audit-trail duties for registered CAS, per vendor guides [33] (TP/VD, SR, L–M). RR 9-2009 set the original electronic-records rules [32] (OD summary, SR, M).
- **Kompleto's ADR 0004** keeps Kompleto from keeping books, computing tax, producing BIR-format files or archiving documents [I3].

### Interpretation

- **A reminder that states an amount due is not an invoice** and is not a document issued "upon the sale"; it refers to a sale already invoiced by the creditor. It should not trigger the invoicing rules (M).
- **Risk rises if Balanse produces a document**: a PDF titled "Statement of Account" is a supplementary document needing the legend, and anything titled "Invoice" or "Billing Invoice" needs registration or ATP and may make Balanse a CAS component (M).
- **CAS is the real grey zone.** A list of who owes how much is close to an AR subsidiary record, and "collection facilities" are named. The line that keeps Balanse outside: the creditor's books of accounts live elsewhere (manual books or their accounting software), Balanse holds a follow-up copy of amounts the creditor already invoiced, and Balanse generates no accounting record, report or document for the BIR (M). The same three crossings as Kompleto apply: recording sales or AR as the system of record, issuing invoices or receipts, or producing BIR-format reports [I1] §2.

### Recommendation (an ADR 0004 equivalent for Balanse)

Balanse never issues, numbers or titles anything as an invoice, billing invoice, official receipt, collection receipt or statement of account; "marked as paid" in Balanse is a status, not a receipt; no aging report or export styled as a ledger or for BIR filing; terms of service say Balanse is not a book of accounts. A future "attach statement" feature needs a new owner decision and a fresh check. Open question: a written BIR ruling request (cost and timing not found).

## 6. Payments

### Facts

- RA 11127 (National Payment Systems Act) and **BSP Circular 1049 (2019)** require any "operator of a payment system" (OPS) to register within one month of starting. The functions include maintaining a platform that enables payments or fund transfers, operating a system that enables payments through a payment instrument, and providing a system that processes payments on behalf of any person [34] (OD/TP, SR, M). News coverage lists payment gateways, platform providers, payment facilitators, merchant acquirers, cash-in and bills-payment providers as covered [34] (N, SR, M). The BSP ordered an unregistered operator (Lyka) to halt in 2021 [34] (OD, SR, M). For bills payment, the operator of the service registers, not its agent banks (BSP M-2020-004) [35] (N, SR, M).
- No general merchant or "technical service provider" exemption was found [34] (not found). E-money issuer rules (BSP Circular 649) were **not checked** in this pass.

### Interpretation (M)

- **Out of scope:** showing the creditor's own static GCash/Maya/InstaPay QR, bank details or a link to the creditor's own PayMongo/Xendit payment link, with money going straight to the creditor and Balanse never seeing it. Balanse then maintains no payment platform and processes nothing on anyone's behalf.
- **In scope or close:** collecting into Balanse's own account and remitting (payment facilitator / bills payment); one merchant account with sub-merchants or platform payouts (platform provider); stored balances or credits usable for payments (possible e-money); a "Pay now" checkout run through Balanse's merchant account. Creating payment links on a creditor's own gateway account through its API key is untested: no exemption text found.

### Recommendation

First version: static payment details and creditor-owned links only. Payment details shown in messages may change only after the creditor re-authenticates, and every message carries a line such as "Magbabayad lang po sa [registered account]; hindi po namin hihingin ang inyong OTP o PIN" [42]. Any API payment integration needs a new decision and a fresh BSP check.

## 7. Messaging channels

### Facts

- **SMS (NTC).** MC 03-03-2005 (broadcast messaging) required opt-in for commercial broadcasts and barred broadcast/push messages **between 9:00 p.m. and 7:00 a.m.**; MC 04-07-2009 banned content providers from initiating push messages; MC 07-08-2018 requires double opt-in and free opt-out by "STOP", "END", "CANCEL", "UNSUBSCRIBE" and "QUIT" in any case [36] (OD, SR, M). Telcos must block SMS containing clickable links (NTC 2023 memo), and Semaphore rejects sender names not tied to a business [I1] §5.
- **Sender IDs.** Vendor guides say alphanumeric sender IDs must be pre-registered with Globe and Smart, are blocked if unregistered **from 7 Apr 2025**, must match the business name (≤ 11 characters), need a letter of authorisation, and may still be overwritten by operators [37] (VD, SR, M). NTC primary text: not found.
- **RA 11934 SIM Registration Act** (IRR NTC MC 001-12-2022) requires every SIM, including eSIMs, to be registered before activation [38] (OD, SR, M–H). Nothing on A2P SMS was found.
- **Email.** The Cybercrime Act's ban on unsolicited commercial communications (§4(c)(3)) was struck down in *Disini* (2014) [26] (TP, SR, H). **No other PH anti-spam statute for email was found** (search incomplete: RA 8792 not checked).
- **Messenger.** Businesses may reply within a 24-hour window after a person messages them; outside it, message tags allow only approved non-promotional updates, and misuse can restrict the Page [40] (VD, SR, M). Whether payment reminders fit a current tag: not found.
- **Viber.** The share scheme `viber://forward?text=` opens Viber with pre-filled text that "the user will either be able to edit … or simply send" [41] (VD, SR, M). Rakuten Viber terms on business use of personal accounts: not found.

### Interpretation

- The NTC broadcast rules target content providers sending commercial broadcasts; whether a one-to-one transactional reminder sent through an aggregator is a "broadcast/push message" is unclear (L). Following their stricter hours and opt-out keywords costs little.
- Copy-and-open on Viber or Messenger means the creditor sends from their own account; Balanse sends nothing on those platforms (M).
- Absence of an email spam law does not remove DPA duties on email addresses (§2) (M).

## 8. Product rules Balanse should enforce in code

Each rule is a **recommendation**. "Strict" means a law or regulator binds lenders or everyone; "adopted" means Balanse borrows it although it does not bind trade creditors.

| # | Rule (enforced by the application, not by prompts) | Source | Basis |
|---|---|---|---|
| R1 | Every message is approved by a person before sending; the approved text's hash must match what is sent (the engine already does this) | Engine guarantee; MC 18 conduct list [1] | Adopted |
| R2 | Recipients are only the debtor's own contacts (and, for a company debtor, its billing contact). No relatives, references, employers or other third parties; a guarantor or co-maker only if the creditor records a written guarantee | MC 18 [2]; NPC 20-01/2022-02 §5(B) [16]; DPA proportionality [19][20] | Strict for lenders; adopted |
| R3 | One recipient per message: no CC/BCC, no group chats, no bulk "blast" to a list in one message, no public posting or shareable debtor lists | MC 18 publication ban [2]; cyberlibel publication [25]; NPC AO 2023-001 [18]; [I5] | Adopted |
| R4 | Quiet hours: send only **08:00–20:00 Asia/Manila** (stricter than NTC 21:00–07:00 and SEC 22:00–06:00); approvals outside the window queue until it opens; time read only from the injected `Clock` | NTC MC 03-03-2005 [36]; MC 18 [2]; BSP [14] | Adopted |
| R5 | Frequency cap per debtor across all debts and channels (proposed: at most 1 message per 3 days, 2 per week, and a hard stop after N unanswered reminders per debt until the creditor acts) | Art. 287 unjust vexation [27]; Arts. 19/21 [28]. Legal number: not found | Adopted (product choice) |
| R6 | Stop on reply, payment or dispute. A "disputed" status stops reminders, and any later message must say the amount is disputed | MC 18 false-information/dispute rule [2] | Adopted |
| R7 | Opt-out per channel: SMS "STOP/END/CANCEL/UNSUBSCRIBE/QUIT" in any case, an email opt-out line, both free and automatic; opted-out channels cannot be re-enabled by the creditor alone | NTC MC 07-08-2018 [36]; DPA rights [I2] | Adopted |
| R8 | Content guard on templates and model output: block threat, insult, shaming, sexual/gendered and impersonation terms (English, Tagalog, Taglish list in §4) before approval; the approver sees why | MC 18 [2]; RA 10175 [25]; Art. 287 [27]; RA 11313 [29] | Adopted |
| R9 | Truthful identity: each message names the creditor's business and that it was sent via Balanse; never presented as a lawyer, court, agency or government office | MC 18 §4 and false representation [2][6]; AFASA [42] | Adopted |
| R10 | Amounts only as entered by the creditor, with the creditor's own reference; no interest, penalty or late-fee calculator; no auto-escalating amounts | Arts. 1956, 2209, 1229 [23]; Circ. 799 [24]; MC 18 false info [2] | Adopted |
| R11 | Never issue or title anything as invoice, billing invoice, receipt or statement of account; no ledgers, aging reports or BIR-format exports; "paid" is a status only | RR 7-2024 [30]; RMC 77-2024 [30]; RMO 9-2021 [I1]; ADR 0004 [I3] | Strict line |
| R12 | Never touch money: only creditor-owned payment details or links; no Balanse merchant account, wallet or payouts; payment details change only after re-authentication; messages never ask for OTP, PIN or password | RA 11127 / Circ. 1049 [34]; AFASA [42] | Strict line |
| R13 | SMS: no links of any kind (including bare domains), registered business sender ID only, sent through a PH aggregator | NTC link memo [I1]; sender-ID rules [37] | Strict (telco) |
| R14 | Data minimisation: no TINs, IDs, photos, phone contact-list import or social-media scraping; fields limited to name, contacts, amount, due date, reference, notes | DPA [I1]; NPC 20-01 [15] | Strict principle |
| R15 | Privacy line in every message (creditor as controller, notice link); DPA with each creditor; 24-hour breach notice to creditors; deletion on exit | RA 10173 §16, IRR §44, §38 [I1][I2] | Strict |
| R16 | Refuse lending, financing, OLA, pawnshop and informal-lending customers (terms + sign-up question) | MC 18 [1]; RA 11765 [8]; NPC 20-01 [15] | Adopted |
| R17 | Keep a per-debt evidence log (approved text, time, provider ID, delivery, replies) for a set retention period, exportable by the creditor | Art. 1169 proof of demand [22]; DPA retention [I1] | Adopted |
| R18 | Schools: messages go to the parent or guardian, never to the student, and never via class group chats | [I5]; DPA proportionality [19] | Adopted |

## Open questions

1. **Exact text of SEC MC 18 §1 and §4**, including the 15-day/consent exceptions to the hours rule, and whether SEC MC 32-2024 replaced it. Primary PDF not readable this session.
2. **BSP Circular 1160's list of collection acts** and the RA 11765 section number for the abusive-collection ban: not found.
3. **Whether an email or SMS reminder is a valid extrajudicial demand** (RA 8792 functional equivalence, any Supreme Court ruling): not checked; search budget exhausted.
4. **BIR view on reminder tools vs CAS components** (RMO 9-2021 "collection" facilities): no ruling found. Consider a written BIR ruling request before paid launch; cost and timing not found.
5. **NPC Circular 2022-04 "likely to pose a risk" trigger** applied to debt data, and the conclusion of NPC AO 2023-001 on posting delinquents.
6. **NTC broadcast rules for one-to-one transactional A2P SMS**, and the NTC's own sender-ID issuance.
7. **Viber terms for business use of personal accounts; Messenger tags for payment reminders.**
8. **E-money rules (BSP Circular 649) and RA 11967 (Internet Transactions Act)**: not checked.
9. Owner decision: whether any template may mention legal action (e.g. small claims) and on what conditions.

## Sources

All accessed 2026-10-24. "SR" means only the search-result excerpt was seen; WebFetch and direct downloads failed for every host this session. Numbers [21] and [39] are unused.

**Internal (not redone)**
- [I1] Kompleto, `docs/research/p1-kill-checks.md` §2 (BIR CAS/eTSP, RMO 9-2021 primary text read), §4 (data privacy), §5 (channels, NTC link memo), 2026-10-20.
- [I2] Kompleto, `docs/research/p1-launch-prerequisites.md` §1–3 (business registration, NPC registration/declaration, DPO, breach), 2026-10-21.
- [I3] Kompleto, `docs/adr/0004-stay-outside-bir-accounting-system-rules.md`.
- [I4] Kompleto, `docs/legal.md` (draft privacy notice, terms, DPA; not lawyer-reviewed).
- [I5] Kompleto, `docs/research/ph-industries-sectors.md` (RA 11984 schools; debt shaming in group chats).

**Debt collection**
- [1] SEC, MC No. 18 s. 2019 page, https://sec.gov.ph/reportorial-requirements/corporations-with-secondary-licenses/financing-companies/www.sec.gov.ph/mc-2019/mc-no-18-s-2019-prohibition-on-unfair-debt-collection-practices-of-financing-companiesfc-and-lending-companieslc, 2019-08-19, OD, SR, M.
- [2] BusinessWorld, "Corporate regulator cracks down on 'unfair debt collection practices' by financing, lending firms", https://www.bworldonline.com/?p=249677, 2019, N, SR, M.
- [3] Grant Thornton PH, "Prohibition on unfair debt collection practices of financing companies, lending companies", https://www.grantthornton.com.ph/insights/articles-and-updates1/tax-notes/prohibition-on-unfair-debt-collection-practices-of-financing-companies-lending-companies/, 2019, TP, SR, M.
- [4] ACCRALAW, "Stop Debt Collection Harassment", https://accralaw.com/2019/10/08/stop-debt-collection-harassment/, 2019-10-08, TP, SR, M.
- [5] Philstar, "SEC issues memorandum on unfair debt collection practices", https://www.philstar.com/business/2019/08/26/1946404/sec-issues-memorandum-unfair-debt-collection-practices, 2019-08-26, N, SR, M.
- [6] labanph.org, "SEC MC 18 (2019) — Full Text", https://labanph.org/sec-mc-18, n.d., TP (unverified reproduction), SR, L.
- [7] Search excerpts on SEC enforcement: Newsbytes, "SEC orders 6 lending firms to stop operations", https://newsbytes.ph/2023/06/30/sec-orders-6-lending-firms-to-stop-operations/, 2023-06-30; MEXC News, https://www.mexc.com/news/681318, 2026 (Global Dominion fine; excerpt not tied to one URL), N, SR, L–M.
- [8] RA 11765, Supreme Court E-Library, https://elibrary.judiciary.gov.ph/thebookshelf/showdocs/2/95036 ; Lawphil, https://lawphil.net/statutes/repacts/ra2022/ra_11765_2022.html, 2022, OD, SR, M–H.
- [9] Insurance Commission, IMC 2023-01 (IRR of RA 11765), https://elibrary.judiciary.gov.ph/thebookshelf/showdocs/2/96925, 2023, OD, SR, M.
- [10] CDA MC 2023-14 (IRR), https://elibrary.judiciary.gov.ph/thebookshelf/showdocs/2/96961 ; CDA FCPA FAQs, https://cda.gov.ph/wp-content/uploads/2026/03/PPT_FCPA-FAQs.pdf, 2023/2026, OD, SR, M.
- [11] BSP, Circular No. 1160, https://www.bsp.gov.ph/Regulations/Issuances/2022/1160.pdf, 2022-11-28, OD, SR, M.
- [12] ACCRALAW, "Bangko Sentral amends regulations on financial consumer protection", https://accralaw.com/2023/01/13/bangko-sentral-amends-regulations-on-financial-consumer-protection/, 2023-01-13 ; Mondaq, "BSP issues implementing rules of Financial Consumer Protection Act", https://mondaq.com/financial-services/1300890/bsp-issues-implementing-rules-of-financial-consumer-protection-act, 2023, TP, SR, M.
- [13] Ablola, Saribong & Gueco, "BSP Circular 1160: Financial Consumer Protection Rules for Banks and E-Money Issuers", https://asg.ph/insights/bsp-financial-consumer-protection-1160, n.d., TP, SR, M.
- [14] Respicio & Co., collection-hours commentaries, https://www.respicio.ph/commentaries/bsp-rules-on-fair-debt-collection-practices-for-credit-card-companies ; https://www.respicio.ph/commentaries/how-to-report-harassment-by-collection-agencies-in-the-philippines, n.d., TP, SR, L.

**Data privacy**
- [15] NPC, Circular No. 20-01, https://privacy.gov.ph/wp-content/uploads/2020/10/NPC-Circular-No.-20-01.pdf, 2020-09-14, OD, SR (fetch failed), M.
- [16] NPC, Circular No. 2022-02 amending 20-01, https://privacy.gov.ph/wp-content/uploads/2023/05/NPC-Circular-No.-2022-%E2%80%93-02-Amending-20-01-Loan-Related-Transactions.pdf ; NPC news, https://privacy.gov.ph/npc-amends-circular-on-the-processing-of-personal-data-for-loan-related-transactions/, 2022–23, OD, SR, M.
- [17] Inquirer, "Hands off borrowers' contact list, lenders told", https://business.inquirer.net/380868/hands-off-borrowers-contact-list-lenders-told ; Newsbytes, https://newsbytes.ph/2020/10/19/npc-circular-bars-online-lenders-from-getting-phone-social-media-contact-list-of-borrowers/, 2020–23, N, SR, M.
- [18] NPC, Advisory Opinion No. 2023-001, https://privacy.gov.ph/wp-content/uploads/2023/05/Advisory-Opinion-No.-2023-001.pdf, 2023, OD, SR, L–M.
- [19] NPC, Advisory Opinions 2025-003, https://privacy.gov.ph/wp-content/uploads/2025/06/NPC-Advisory-Opinion-No.-2025-003-AFPSLAI_Redacted.pdf, and 2025-002, https://privacy.gov.ph/wp-content/uploads/2025/04/NPC-Advisory-Opinion-No.-2025-002-Philippine-National-Bank.pdf, 2025, OD, SR, M; Respicio, "Privacy Violations by Debt Collectors Under the Data Privacy Act", https://www.respicio.ph/commentaries/privacy-violations-by-debt-collectors-under-the-data-privacy-act, n.d., TP, SR, L–M.
- [20] Baker McKenzie, "NPC issues guidelines on the processing of personal information based on legitimate interest" (NPC Circular 2023-07), https://insightplus.bakermckenzie.com/bm/data-technology/philippines-national-privacy-commission-issues-guidelines-on-the-processing-of-personal-information-based-on-legitimate-interest, 2024-01, TP, SR, M.

**Civil Code**
- [22] Supreme Court, G.R. No. 233774, https://lawphil.net/judjuris/juri2019/apr2019/gr_233774_2019.html, 2019-04, OD, SR, M; Respicio, "Is an Email Demand Letter Valid in the Philippines?", https://www.respicio.ph/commentaries/is-an-email-demand-letter-valid-in-the-philippines-what-to-do-if-details-are-incomplete, n.d., TP, SR, L.
- [23] Respicio, "Legal Interest on Loans Under the Philippine Civil Code", https://www.respicio.ph/commentaries/legal-interest-on-loans-under-the-philippine-civil-code-rules-and-rates ; Divina Law, "The cost of debt 3", https://www.divinalaw.com/dose-of-law/the-cost-of-debt-3/, 2019-08-12, TP, SR, M.
- [24] Supreme Court, *Odiamar v. Valencia*, G.R. No. 213582, https://lawphil.net/judjuris/juri2018/sep2018/pdf/gr_213582_2018.pdf, 2018-09-12, OD, SR, H; lawyerly.ph citing MB Circular 799, https://lawyerly.ph/cited/c7dc0, n.d., TP, SR, M.

**Harassment and defamation**
- [25] Respicio, "Cyber Libel for Debt Shaming", https://www.lawyer-philippines.com/articles/cyber-libel-for-debt-shaming ; "Cyber Libel & Debt-Shaming on Facebook", https://www.respicio.ph/commentaries/cyber-libel-grounds-public-debt-humiliation-philippines, 2025, TP, SR, M.
- [26] Columbia Global Freedom of Expression, *Disini v. Secretary of Justice*, G.R. No. 203335, https://globalfreedomofexpression.columbia.edu/cases/disini-v-the-secretary-of-justice/, decision 2014-02-18, TP, SR, H.
- [27] NDV Law, "Annoying Someone is a Crime", https://ndvlaw.com/annoying-someone-is-a-crime-updated-last-10-june-2020/, 2020-06-10 ; "Can Debt Collection Harassment Lead to Unjust Vexation?", https://ndvlaw.com/can-debt-collection-harassment-lead-to-unjust-vexation/, n.d., TP, SR, M.
- [28] jur.ph, *Unicapital v. Consing*, G.R. No. 175277, https://jur.ph/jurisprudence/summary/unicapital-inc-v-consing-jr ; *Navarro-Banaria v. Banaria*, G.R. No. 217806, https://jur.ph/jurisprudence/summary/navarro-banaria-v-banaria ; *Barons Marketing v. CA*, https://jur.ph/jurisprudence/barons-marketing-corp-v-court-of-appeals, TP (case summaries), SR, M.
- [29] RA 11313 (PRC copy), https://prc.gov.ph/sites/default/files/Republic%20Act%20No.%2011313.pdf ; Batas Natin explainer, https://batasnatin.com/explainers/ra-11313-safe-spaces-act-explained-protection-against-sexual-harassment, 2019, OD/TP, SR, M.

**BIR**
- [30] RR 7-2024 full text (third-party copy), https://www.offshore-management.com.ph/blog/wordpress/wp-content/uploads/2024/05/RR-7-2024_Full.pdf ; Grant Thornton PH, "Clarification for Compliance with Invoicing Requirements: BIR RR No. 7-2024", https://www.grantthornton.com.ph/insights/articles-and-updates1/lets-talk-tax/clarification-for-compliance-with-invoicing-requirements-bir-rr-no-7-2024/ ; BIR, RMC 77-2024 digest, https://bir-cdn.bir.gov.ph/BIR/pdf/RMC%20No.%2077-2024%20Digest.pdf, 2024, OD/TP, SR, M.
- [31] BIR, RR 11-2024, https://bir-cdn.bir.gov.ph/BIR/pdf/RR%2011-2024.pdf, 2024-06-13, OD, SR, M.
- [32] Lawyerly, BIR RR 9-2009, https://dev.lawyerly.ph/laws/48854, 2009-12-23, OD (database copy), SR, M.
- [33] ClearTax PH, "BIR CAS Registration Philippines", https://www.cleartax.com/ph/bir-cas-registration ; Orkids, "Computerized Accounting System & BIR CAS", https://orkids.ph/guides/what-is-a-computerized-accounting-system, 2025–26, VD/TP, SR, L–M.

**Payments**
- [34] BSP, Circular No. 1049, https://www.bsp.gov.ph/Regulations/Issuances/2019/c1049.pdf, 2019, OD, SR, M; Lexology, "BSP Issues Rules on Registration of Operators of Payment Systems", https://www.lexology.com/library/detail.aspx?g=add1bb97-795c-4fca-916b-a6c2e2be5b90, 2019, TP, SR, M; Philstar, "Payment systems operators required to register with BSP", https://www.philstar.com/business/2019/09/13/1951253/payment-systems-operators-required-register-bsp, 2019-09-13, N, SR, M; PIA, BSP halt order, https://pia.gov.ph/press-releases/2021/07/25/bsp-orders-an-operator-of-payment-system-to-halt-pperations-and-invites-it-to-register-with-bsp, 2021-07-25, OD, SR, M.
- [35] BitPinas, "BSP Clarifies Who Should Register as a Bills Payment Service", https://bitpinas.com/news/bsp-clarifies-register-bills-payment-service/, 2020, N, SR, M.

**Messaging**
- [36] NTC MC 03-03-2005, https://elibrary.judiciary.gov.ph/thebookshelf/showdocs/11/42711 ; MC 04-07-2009, https://elibrary.judiciary.gov.ph/thebookshelf/showdocs/10/49116 ; MC 07-08-2018, https://elibrary.judiciary.gov.ph/thebookshelf/showdocs/10/91581, 2005–2018, OD, SR, M.
- [37] Twilio, "Philippines: SMS Guidelines", https://www.twilio.com/en-us/guidelines/ph/sms ; Krispcall, sender-ID registration, https://help.krispcall.com/en/article/documents-required-and-instructions-to-register-your-alphanumeric-sender-id-in-the-philippines-1wxpkd8/ ; Telnyx, https://support.telnyx.com/en/articles/6531682-philippines-sms-guidelines, n.d., VD, SR, M.
- [38] NTC MC 001-12-2022 (IRR of RA 11934), https://elibrary.judiciary.gov.ph/thebookshelf/showdocs/2/96154, 2022-12, OD, SR, M–H.
- [40] Meta, "Messenger Platform and IG Messaging API policy", https://developers.facebook.com/documentation/business-messaging/messenger-platform/policy, n.d., VD, SR, M.
- [41] Rakuten Viber, "Share button", https://developers.viber.com/docs/tools/share-button/, n.d., VD, SR, M.
- [42] RA 12010 (AFASA), https://lawphil.net/statutes/repacts/ra2024/ra_12010_2024.html ; Batas Natin, https://batasnatin.com/laws/ra-12010 ; FNS Law, https://fnslaw.com.ph/insights/the-anti-financial-account-scamming-act/, 2024-07-20, OD/TP, SR, M.
