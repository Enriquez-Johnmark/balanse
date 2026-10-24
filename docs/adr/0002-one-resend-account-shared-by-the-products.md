# One Resend account is shared by the products

Kompleto and Balanse send through one Resend account, each from its own domain, with a daily send budget per product set in configuration: Kompleto 60 and Balanse 30 while on Resend Free, leaving about 10 of the 100 a day for login emails and inbound mail, which also count toward the quota. Resend Pro (US$20/month, no daily cap) comes out of the first customer payment.

A second free account would double the free quota, but Resend's acceptable-use policy forbids "creating or using an account or multiple accounts with the aim of circumventing any quotas or limits imposed by our service" (resend.com/legal/acceptable-use, read 2026-10-24). Losing the account would stop both products' email at once.

## Consequences

- Neither product can use the whole free quota. If one product's budget runs out, its remaining reminders wait until the next day, as Kompleto's already do.
- Kompleto's budget, fixed at 90 today, becomes configurable before both products run live.
- Webhooks and inbound mail for both domains arrive at one account. Each product verifies events and matches them to its own messages, so one product never acts on the other's events.
