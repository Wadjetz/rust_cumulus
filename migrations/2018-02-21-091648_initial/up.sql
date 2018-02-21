CREATE TABLE IF NOT EXISTS users (
    uuid UUID PRIMARY KEY,
    login TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    created TIMESTAMP,
    updated TIMESTAMP
);

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'sourcetype') THEN
        CREATE TYPE SourceType AS ENUM (
            'Rss', 'Twitter'
        );
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS sources (
    uuid UUID PRIMARY KEY,
    source_type SourceType NOT NULL,
    data JSONB,
    error TEXT,
    created TIMESTAMP,
    updated TIMESTAMP
);

CREATE TABLE IF NOT EXISTS users_sources (
    uuid UUID PRIMARY KEY,
    user_uuid UUID NOT NULL REFERENCES users(uuid),
    source_uuid UUID NOT NULL REFERENCES sources(uuid)
);

CREATE TABLE IF NOT EXISTS feeds (
    uuid UUID PRIMARY KEY,
    url TEXT NOT NULL,
    rss JSONB,
    readable JSONB,
    twitter JSONB,
    created TIMESTAMP,
    updated TIMESTAMP,
    source_uuid UUID NOT NULL REFERENCES sources(uuid)
);

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'reaction') THEN
        CREATE TYPE Reaction AS ENUM (
            'Unreaded',
            'Readed',
            'ReadLater',
            'Viewed',
            'Liked',
            'Disliked',
            'Archived'
        );
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS users_feeds (
    uuid UUID PRIMARY KEY,
    reaction Reaction NOT NULL,
    feed_uuid UUID NOT NULL REFERENCES feeds(uuid),
    user_uuid UUID NOT NULL REFERENCES users(uuid),
    created TIMESTAMP,
    updated TIMESTAMP
);
