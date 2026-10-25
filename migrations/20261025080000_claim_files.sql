-- Proof of payment a Contact attached to a claim (G5): at most 3 files
-- per claim, each a stored (encrypted) file of the engine's. Deleted 30
-- days after the claim is settled and never kept more than 90 days.
create table claim_files (
    id           uuid primary key references stored_files (id) on delete cascade,
    tenant_id    uuid not null,
    claim_id     uuid not null references claims (id) on delete cascade,
    media_type   text not null,
    size_bytes   integer not null,
    uploaded_at  timestamptz not null
);

create index claim_files_claim on claim_files (tenant_id, claim_id, uploaded_at);
