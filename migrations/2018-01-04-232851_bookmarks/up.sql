-- Your SQL goes here

CREATE TABLE users (
    uuid UUID PRIMARY KEY,
    login TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    created TIMESTAMP NOT NULL,
    updated TIMESTAMP NOT NULL
);

CREATE TABLE bookmarks (
    uuid UUID PRIMARY KEY,
    url TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    path TEXT,
    created TIMESTAMP NOT NULL,
    updated TIMESTAMP NOT NULL,
    user_uuid UUID NOT NULL REFERENCES users(uuid)
);
