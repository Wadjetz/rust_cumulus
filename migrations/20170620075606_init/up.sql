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
    parent TEXT NOT NULL,
    location TEXT NOT NULL,
    file_type TEXT NOT NULL,
    size BIGINT,
    user_uuid UUID NOT NULL REFERENCES users(uuid)
);
