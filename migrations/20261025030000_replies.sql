-- Replies to reminders, matched by their signed reply address. The text is
-- untrusted and shown only inside Balanse, never copied into an email; it is
-- deleted after 90 days (F5).
create table replies (
    id            uuid primary key,
    tenant_id     uuid not null,
    debtor_id     uuid not null,
    reminder_id   uuid not null references reminders (id) on delete cascade,
    from_address  text not null,
    subject       text not null,
    body_text     text,
    automatic     boolean not null,
    received_at   timestamptz not null,
    -- The provider's message ID, so a message delivered twice is kept once.
    message_key   text not null,
    unique (tenant_id, message_key),
    foreign key (debtor_id, tenant_id) references debtors (id, tenant_id) on delete cascade
);

create index replies_debtor on replies (tenant_id, debtor_id, received_at);
