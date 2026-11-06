-- Delivery failures and spam complaints the provider reported for reminders
-- Balanse sent (F4). A permanent bounce or a complaint marks the Debtor
-- `bounced` until the Owner changes the Contact's address; a temporary one
-- is only shown.
create table bounces (
    id          uuid primary key,
    tenant_id   uuid not null,
    debtor_id   uuid not null,
    -- The engine's outbox message that failed.
    message_id  uuid not null,
    address     text not null,
    kind        text not null check (kind in ('bounce', 'soft_bounce', 'complaint')),
    reason      text,
    at          timestamptz not null,
    foreign key (debtor_id, tenant_id) references debtors (id, tenant_id) on delete cascade
);

create index bounces_debtor on bounces (tenant_id, debtor_id, at);
