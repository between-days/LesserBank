DO $$ BEGIN
    CREATE TYPE account_type AS ENUM ('savings', 'transaction', 'term deposit');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

ALTER TABLE accounts 
ADD account_type account_type NOT NULL