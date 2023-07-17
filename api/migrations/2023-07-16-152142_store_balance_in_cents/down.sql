ALTER TABLE accounts 
DROP COLUMN balance_cents,
ADD COLUMN balance integer not null