-- Your SQL goes here

CREATE TABLE diesel_bookmarks (
    uuid UUID PRIMARY KEY,
    url TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    path TEXT,
    created TIMESTAMP NOT NULL,
    updated TIMESTAMP NOT NULL,
    user_uuid UUID NOT NULL-- REFERENCES users(uuid)
);
