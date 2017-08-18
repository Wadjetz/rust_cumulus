use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;

use errors::*;
use migration::{Evolution, sync};

pub fn run(connection: PooledConnection<PostgresConnectionManager>) -> Result<()> {
    let migrations = vec![
        Evolution::new("1", r#"
            CREATE TABLE users (
                uuid UUID PRIMARY KEY,
                login TEXT NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password TEXT NOT NULL,
                created TIMESTAMP,
                updated TIMESTAMP
            );

            CREATE TYPE SourceType AS ENUM (
                'Rss', 'Twitter'
            );

            CREATE TABLE sources (
                uuid UUID PRIMARY KEY,
                source_type SourceType NOT NULL,
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
                url TEXT NOT NULL,
                rss JSONB,
                readable JSONB,
                twitter JSONB,
                created TIMESTAMP,
                updated TIMESTAMP,
                source_uuid UUID NOT NULL REFERENCES sources(uuid)
            );

            CREATE TYPE Reaction AS ENUM (
                'Readed',
                'ReadLater',
                'Viewed',
                'Liked',
                'Disliked',
                'Archived'
            );

            CREATE TABLE users_feeds (
                uuid UUID PRIMARY KEY,
                reaction Reaction NOT NULL,
                feed_uuid UUID NOT NULL REFERENCES feeds(uuid),
                user_uuid UUID NOT NULL REFERENCES users(uuid),
                created TIMESTAMP,
                updated TIMESTAMP
            );

            CREATE TABLE bookmarks (
                uuid UUID PRIMARY KEY,
                url TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                path TEXT,
                created TIMESTAMP,
                updated TIMESTAMP,
                user_uuid UUID NOT NULL REFERENCES users(uuid)
            );

            CREATE TYPE FileType AS ENUM (
                'File', 'Directory'
            );

            CREATE TABLE files (
                uuid UUID PRIMARY KEY,
                hash TEXT,
                name TEXT NOT NULL,
                location TEXT NOT NULL,
                file_type FileType NOT NULL,
                size BIGINT,
                user_uuid UUID NOT NULL REFERENCES users(uuid)
            );
        "#, r#"
            DROP TABLE users_feeds;
            DROP TABLE users_sources;
            DROP TABLE feeds;
            DROP TABLE sources;

            DROP TABLE bookmarks;
            DROP TABLE files;

            DROP TABLE users;

            DROP TYPE "filetype";
            DROP TYPE "sourcetype";
        "#), 
    ];

    sync(connection, migrations)
}
