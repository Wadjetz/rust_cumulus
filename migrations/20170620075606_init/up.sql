CREATE TABLE users (
    uuid UUID PRIMARY KEY,
    login TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);

CREATE TABLE bookmarks (
    uuid UUID PRIMARY KEY,
    url TEXT NOT NULL,
    title TEXT,
    description TEXT,
    path TEXT,
    created TIMESTAMP,
    updated TIMESTAMP,
    user_uuid UUID NOT NULL REFERENCES users(uuid)
);

CREATE TABLE files (
    uuid UUID PRIMARY KEY,
    hash TEXT,
    name TEXT NOT NULL,
    location TEXT NOT NULL,
    file_type TEXT NOT NULL,
    size BIGINT,
    user_uuid UUID NOT NULL REFERENCES users(uuid)
);

CREATE TABLE sources (
    uuid UUID PRIMARY KEY,
    source_type TEXT NOT NULL,
    data JSONB,
    error TEXT,
    created TIMESTAMP,
    updated TIMESTAMP
);

CREATE TABLE users_sources (
    uuid UUID PRIMARY KEY,
    user_uuid UUID NOT NULL REFERENCES users(uuid),
    source_uuid UUID NOT NULL REFERENCES sources(uuid)
);

CREATE TABLE feeds (
    uuid UUID PRIMARY KEY,
    url TEXT UNIQUE NOT NULL,
    rss JSONB,
    readable JSONB,
    twitter JSONB,
    created TIMESTAMP,
    updated TIMESTAMP,
    source_uuid UUID NOT NULL REFERENCES sources(uuid)
);

CREATE TABLE users_feeds (
    uuid UUID PRIMARY KEY,
    reaction TEXT NOT NULL,
    feed_uuid UUID NOT NULL REFERENCES feeds(uuid),
    user_uuid UUID NOT NULL REFERENCES users(uuid),
    created TIMESTAMP,
    updated TIMESTAMP
);
