-- Balanse's settings for each Business. The Business itself is the engine's
-- account (accounts.tenant_id); this row and everything else of Balanse's
-- goes with it when the account is erased.
create table businesses (
    tenant_id   uuid primary key references accounts (tenant_id) on delete cascade,
    created_at  timestamptz not null
);
