-- Reminder cycles. A Debtor's state says whether reminders are drafted at
-- all; each reminder drafted is recorded with its cycle and step, so a step
-- is drafted at most once per cycle whatever becomes of the Draft.
alter table businesses add column heads_up boolean not null default false;

alter table debtors
    add column state text not null default 'active' check (state in (
        'active', 'paused', 'promised', 'needs_call', 'payment_claimed', 'disputed', 'opted_out', 'bounced')),
    -- Until when a Promise holds; reminders resume the day after.
    add column promised_until date,
    -- When the current cycle started; set when its first reminder is drafted.
    add column cycle_start date,
    add check ((state = 'promised') = (promised_until is not null));

create table reminders (
    id           uuid primary key,
    tenant_id    uuid not null,
    debtor_id    uuid not null,
    cycle_start  date not null,
    step         text not null check (step in ('heads_up', 'first', 'second', 'third')),
    -- The engine's outbox message carrying it.
    message_id   uuid not null unique,
    created_at   timestamptz not null,
    unique (tenant_id, debtor_id, cycle_start, step),
    foreign key (debtor_id, tenant_id) references debtors (id, tenant_id) on delete cascade
);

-- Messages the Owner sent by hand ("I sent it on …"); they count toward the
-- cap like any other.
create table manual_sends (
    id          uuid primary key,
    tenant_id   uuid not null,
    debtor_id   uuid not null,
    sent_at     timestamptz not null,
    logged_by   text not null,
    created_at  timestamptz not null,
    foreign key (debtor_id, tenant_id) references debtors (id, tenant_id) on delete cascade
);

create index manual_sends_debtor on manual_sends (tenant_id, debtor_id, sent_at);
