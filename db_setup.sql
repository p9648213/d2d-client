-- CREATE DATABASE
DROP DATABASE IF EXISTS "d2d";
CREATE DATABASE "d2d";

-- CREATE TYPE
CREATE TYPE transaction_type AS ENUM ('DEPOSIT', 'WITHDRAWAL', 'BET', 'WIN', 'BONUS', 'ADJUSTMENT', 'AFFILIATE_PAYOUT', 'CASH_OUT_SETTLE'); -- Added more examples
CREATE TYPE transaction_status AS ENUM ('PENDING', 'COMPLETED', 'FAILED', 'CANCELLED');

-- CREATE TABLE
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(255) PRIMARY KEY NOT NULL,
    account_id VARCHAR(255) UNIQUE,
    username VARCHAR(255),
    email VARCHAR(255) UNIQUE,
    password VARCHAR(255),
    image_url VARCHAR(255),
    provider VARCHAR(20),
    created_at TIMESTAMPZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS wallets (
    id VARCHAR(255) PRIMARY KEY NOT NULL,
    user_id VARCHAR(255) NOT NULL REFERENCES users(id) ON DELETE CASCADE UNIQUE,
    balance_sc DECIMAL(20, 8) NOT NULL DEFAULT 0.0, -- Balance IN SiteCoin
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP -- Add trigger
);

CREATE TABLE IF NOT EXISTS transactions (
    id VARCHAR(255) PRIMARY KEY NOT NULL, -- UUID
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    wallet_id VARCHAR(255) NOT NULL REFERENCES wallets(id), -- Refers to the user's single SC wallet
    type transaction_type NOT NULL,
    status transaction_status NOT NULL DEFAULT 'COMPLETED', -- Default to completed for simple types initially

    -- Core Transaction Amount (in SiteCoin)
    amount DECIMAL(20, 8) NOT NULL, -- Change in SC balance (+ credit, - debit)
    balance_sc_before DECIMAL(20, 8) NOT NULL,
    balance_sc_after DECIMAL(20, 8) NOT NULL,

    -- Conversion/Reference Details (Populated where applicable)
    original_currency VARCHAR(10),          -- For DEPOSIT: e.g., 'BTC', 'USD'.
    original_amount DECIMAL(20, 8),         -- For DEPOSIT: Amount received in original_currency.
    target_currency VARCHAR(10),            -- For WITHDRAWAL: e.g., 'BTC', 'USD'.
    target_amount DECIMAL(20, 8),           -- For WITHDRAWAL: Amount sent in target_currency.
    exchange_rate_xxx_usd DECIMAL(20, 10),  -- Market rate (e.g., BTC/USD) used.
    exchange_rate_sc_usd DECIMAL(20, 10),   -- Internal rate (SC/USD) used.

    reference_id VARCHAR(255),              -- Link to deposit_req_id, withdrawal_req_id, game_round_id, sports_bet_id etc. (Can be UUID)
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP -- Add trigger
);

-- FUNCTION
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- TRIGGER
DO
$$
DECLARE
    tbl RECORD;
BEGIN
    FOR tbl IN
        SELECT table_name
        FROM information_schema.columns
        WHERE table_schema = 'public' 
          AND column_name = 'updated_at'
    LOOP
        -- Drop the trigger if it already exists to avoid duplication
        EXECUTE format('
            DROP TRIGGER IF EXISTS trigger_update_timestamp ON %I
        ', tbl.table_name);

        -- Create the trigger to update `updated_at` on each update
        EXECUTE format('
            CREATE TRIGGER trigger_update_timestamp
            BEFORE UPDATE ON %I
            FOR EACH ROW
            EXECUTE FUNCTION update_timestamp()
        ', tbl.table_name);
    END LOOP;
END
$$;
