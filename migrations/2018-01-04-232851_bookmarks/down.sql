-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS bookmarks;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS conversations;

DROP TYPE IF EXISTS "sourcetype";
