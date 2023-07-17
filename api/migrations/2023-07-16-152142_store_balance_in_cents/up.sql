ALTER TABLE accounts 
DROP COLUMN balance,
ADD COLUMN balance_cents bigint not null