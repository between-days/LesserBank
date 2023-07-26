ALTER TABLE accounts
DROP COLUMN date_opened,
DROP COLUMN account_status,
DROP COLUMN account_name,
DROP COLUMN available_balance_cents,
DROP COLUMN account_number,
DROP COLUMN bsb;

DROP TYPE account_status;

-- TODO: can't get below to work
-- DROP INDEX accounts."accounts_account_number_key";