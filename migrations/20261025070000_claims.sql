-- A Contact's claim, from the Debtor's page, that they have paid (G2). It
-- stands until the Owner confirms or rejects it; proof files, if any, are
-- kept 30 days after that (G5).
create table claims (
    id          uuid primary key,
    tenant_id   uuid not null,
    debtor_id   uuid not null,
    claimed_at  timestamptz not null,
    settled_at  timestamptz,
    -- 'confirmed' or 'rejected' once settled.
    outcome     text check (outcome in ('confirmed', 'rejected')),
    check ((settled_at is null) = (outcome is null)),
    foreign key (debtor_id, tenant_id) references debtors (id, tenant_id) on delete cascade
);

create index claims_debtor on claims (tenant_id, debtor_id, claimed_at);
