# Balanse

Balanse helps Philippine service businesses get paid by their clients, politely. It runs on Nudgewright, the engine that drafts messages, holds them for approval, sends each one exactly once and stops when someone replies. Use these words in code, docs and messages.

## People and businesses

**Business**:
A service business that pays for Balanse; one account, owned by one Owner. The engine's Tenant.
_Avoid_: Customer, firm, company, account

**Owner**:
The person who signed the Business up and approves its messages. Their address is the Business's login address.
_Avoid_: User, admin

**Staff**:
Up to 5 people the Owner invites to log in to the Business.
_Avoid_: Team member, user

**Debtor**:
A client of the Business that owes it money for one or more invoices.
_Avoid_: Customer, client (in code), payer

**Contact**:
The person at a Debtor who receives reminders: a name, an email address and optionally a mobile number.
_Avoid_: Recipient (except for one message's address)

## Money owed

**Debt**:
One unpaid invoice the Business already issued: its invoice number and date, the amount as typed, and the due date. Balanse never issues invoices or computes amounts.
_Avoid_: Invoice (Balanse doesn't issue them), receivable, balance, account

**Claim**:
A Debtor saying it has paid, through its page, optionally with proof files. A Claim waits for the Owner to confirm or reject it.
_Avoid_: Payment, receipt

**Dispute**:
A Debtor saying an amount isn't owed. It stops reminders.
_Avoid_: Complaint (that means a spam complaint)

**Promise**:
A Debtor's reply that it will pay by a date, recorded by the Owner. Reminders resume the day after.

## Messages

**Draft**:
A message waiting for the Owner's approval.

**Reminder**:
An approved message to a Debtor's Contact about its open Debts, sent once by email or logged as sent by hand.
_Avoid_: Notice, demand, collection letter

**Reply**:
A human answer from a Contact to a Reminder. Automatic replies are not Replies.

**Debtor page**:
The private page a Reminder links to, where a Contact can claim payment, dispute, or opt out.
_Avoid_: Portal, payment page

**Evidence log**:
The dated record of a Debt's approved messages, sends, delivery events, Replies and status changes, which the Owner can export.
_Avoid_: Ledger, statement, history report
