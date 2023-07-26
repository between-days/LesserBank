DO $$ BEGIN CREATE TYPE transaction_type AS ENUM ('internal', 'external');

EXCEPTION
WHEN duplicate_object THEN null;

END $$;

DO $$ BEGIN CREATE TYPE transaction_status AS ENUM ('pending', 'success', 'error');

EXCEPTION
WHEN duplicate_object THEN null;

END $$;

CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER NOT NULL,
    transaction_type transaction_type NOT NULL,
    from_us bool NOT NULL,
    amount_cents BIGINT NOT NULL,
    from_number VARCHAR(9) NOT NULL,
    from_bsb VARCHAR(6) NOT NULL,
    from_name VARCHAR(40),
    to_number VARCHAR(9) NOT NULL,
    to_bsb VARCHAR(6) NOT NULL,
    to_name VARCHAR(40),
    available_balance_cents BIGINT NOT NULL,
    date_start TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
    date_end TIMESTAMP WITH TIME ZONE,
    transaction_status transaction_status NOT NULL,
    CONSTRAINT to_number_valid CHECK (to_number ~ '[0-9]{9}'),
    CONSTRAINT from_number_valid CHECK (from_number ~ '[0-9]{9}')
)