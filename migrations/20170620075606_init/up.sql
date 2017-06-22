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
