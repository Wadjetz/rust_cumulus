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

CREATE TABLE feeds_sources (
    uuid UUID PRIMARY KEY,
    title TEXT NOT NULL,
    xml_url TEXT UNIQUE NOT NULL,
    html_url TEXT NOT NULL,
    error TEXT,
    created TIMESTAMP,
    updated TIMESTAMP
);

CREATE TABLE users_feeds_sources (
    uuid UUID PRIMARY KEY,
    user_uuid UUID NOT NULL REFERENCES users(uuid),
    feeds_sources_uuid UUID NOT NULL REFERENCES feeds_sources(uuid)
);

CREATE TABLE feeds (
    uuid UUID PRIMARY KEY,
    url TEXT UNIQUE NOT NULL,
    rss JSONB,
    readable JSONB,
    twitter JSONB,
    created TIMESTAMP,
    updated TIMESTAMP
)

CREATE TABLE users_feeds (
    uuid UUID PRIMARY KEY,
    feed_uuid UUID NOT NULL REFERENCES feeds(uuid),
    user_uuid UUID NOT NULL REFERENCES users(uuid)
);

CREATE TABLE feeds_sources_feeds (
    uuid UUID PRIMARY KEY,
    feed_uuid UUID NOT NULL REFERENCES feeds(uuid),
    feed_source_uuid UUID NOT NULL REFERENCES feeds_sources(uuid)
);
