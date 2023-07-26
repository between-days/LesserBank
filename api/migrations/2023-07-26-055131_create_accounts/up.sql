DO $$ BEGIN
    CREATE TYPE account_status AS ENUM ('active', 'inactive');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE account_type AS ENUM ('savings', 'transaction', 'term_deposit');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER NOT NULL,
    account_number VARCHAR(9) UNIQUE NOT NULL,
    bsb VARCHAR(6) NOT NULL DEFAULT '123456',
    account_name VARCHAR(40),
    account_status account_status NOT NULL DEFAULT 'active',
    account_type account_type NOT NULL,
    date_opened TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
    balance_cents BIGINT NOT NULL,
    available_balance_cents BIGINT NOT NULL,
    CONSTRAINT account_number_valid CHECK (account_number ~ '[0-9]{9}'),
    CONSTRAINT bsb_valid CHECK (account_number ~ '[0-9]{6}')
);

