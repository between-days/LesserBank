DO $$ BEGIN
    CREATE TYPE account_status AS ENUM ('active', 'inactive');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

ALTER TABLE accounts
ADD COLUMN date_opened TIMESTAMP WITH TIME ZONE not null default current_timestamp,
ADD COLUMN account_status account_status not null default 'active',
ADD COLUMN name varchar(40),
ADD COLUMN available_balance_cents bigint not null,
ADD COLUMN account_number varchar(9) not null,
ADD COLUMN bsb integer not null default 123456,
ADD CONSTRAINT account_number_valid CHECK (account_number ~ '[0-9]{9}');

CREATE UNIQUE INDEX accounts_account_number_key ON accounts (account_number);