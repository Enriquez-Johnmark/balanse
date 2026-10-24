-- The notice telling the Owner about a human reply, so a redelivered reply
-- can finish approving it if a crash stopped that the first time.
alter table replies add column notice_id uuid;
