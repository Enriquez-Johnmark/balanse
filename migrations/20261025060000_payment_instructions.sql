-- The Owner's own payment instructions, shown on the Debtor's page in the
-- Owner's words (R12, G6). Balanse never adds payment details of its own.
alter table businesses add column payment_instructions text not null default '';
