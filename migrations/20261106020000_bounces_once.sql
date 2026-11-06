-- The provider retries a delivery it isn't sure arrived: each event about a
-- message is kept once (F4).
create unique index bounces_once on bounces (message_id, kind);
