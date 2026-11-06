-- When a Debt was closed (paid, written off or cancelled): its evidence log
-- is kept until a year after this (H3, R17). Every change that closes or
-- reopens a Debt must set or clear it; the check makes sure of that.
alter table debts add column closed_at timestamptz;

update debts set closed_at = updated_at where status in ('paid', 'written_off', 'cancelled');

alter table debts add constraint debts_closed_at_matches_status
    check ((closed_at is not null) = (status in ('paid', 'written_off', 'cancelled')));
