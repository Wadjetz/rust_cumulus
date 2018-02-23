-- This file should undo anything in `up.sql`

ALTER TABLE sources DROP COLUMN url;
ALTER TABLE sources DROP COLUMN title;
ALTER TABLE sources DROP COLUMN website;

ALTER TABLE sources ADD COLUMN source_type SourceType;
ALTER TABLE sources ADD COLUMN data TEXT;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'sourcetype') THEN
        CREATE TYPE SourceType AS ENUM (
            'Rss', 'Twitter'
        );
    END IF;
END
$$;

ALTER TABLE sources ALTER COLUMN created DROP NOT NULL;
ALTER TABLE sources ALTER COLUMN updated DROP NOT NULL;

ALTER TABLE users ALTER COLUMN created DROP NOT NULL;
ALTER TABLE users ALTER COLUMN updated DROP NOT NULL;
