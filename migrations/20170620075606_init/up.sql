CREATE TABLE users (
    uuid UUID PRIMARY KEY,
    login TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);