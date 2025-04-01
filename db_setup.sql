-- CREATE DATABASE
DROP DATABASE IF EXISTS "d2d";
CREATE DATABASE "d2d";

-- CREATE TABLE
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(255) PRIMARY KEY NOT NULL,
    account_id VARCHAR(255) UNIQUE,
    username VARCHAR(255),
    email VARCHAR(255) UNIQUE,
    password VARCHAR(255),
    image_url VARCHAR(255),
    provider VARCHAR(20),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
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