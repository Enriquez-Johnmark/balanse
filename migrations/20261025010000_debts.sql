-- Debtors (the Business's clients) and their Debts (invoices the Business
-- already issued and hasn't been paid for). Balanse never issues invoices or
-- computes amounts: an amount is kept exactly as typed, with its value in
-- centavos only to check it and to list a Debtor's own open Debts.
create table debtors (
    id              uuid primary key,
    tenant_id       uuid not null references businesses (tenant_id) on delete cascade,
    business_name   text not null,
    -- The Contact who receives reminders.
    contact_name    text not null,
    contact_email   text not null,
    contact_mobile  text,
    language        text not null check (language in ('taglish', 'english')),
    created_at      timestamptz not null,
    updated_at      timestamptz not null,
    unique (id, tenant_id)
);

create index debtors_tenant on debtors (tenant_id, business_name);

create table debts (
    id                uuid primary key,
    tenant_id         uuid not null,
    debtor_id         uuid not null,
    invoice_number    text not null,
    invoice_date      date not null,
    amount            text not null,
    amount_centavos   bigint not null check (amount_centavos > 0),
    due_date          date not null,
    note              text,
    status            text not null check (status in (
                          'open', 'payment_claimed', 'paid', 'disputed', 'written_off', 'cancelled')),
    created_at        timestamptz not null,
    updated_at        timestamptz not null,
    foreign key (debtor_id, tenant_id) references debtors (id, tenant_id) on delete cascade
);

create index debts_debtor on debts (tenant_id, debtor_id, due_date);
