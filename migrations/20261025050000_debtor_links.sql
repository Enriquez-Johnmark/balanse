-- The signed link each reminder carries to the Debtor's page (G1). The
-- database holds only the link's ID; the link itself is the ID signed with
-- a key derived from the master key, so a dump gives away no usable link.
-- A link works for 60 days from the day it is issued.
create table debtor_links (
    id          uuid primary key,
    tenant_id   uuid not null,
    debtor_id   uuid not null,
    created_at  timestamptz not null,
    expires_at  timestamptz not null,
    foreign key (debtor_id, tenant_id) references debtors (id, tenant_id) on delete cascade
);

create index debtor_links_debtor on debtor_links (tenant_id, debtor_id, expires_at);
