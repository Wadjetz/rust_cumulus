use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;

use errors::*;
use migration::{Evolution, sync};

pub fn run(connection: PooledConnection<PostgresConnectionManager>) -> Result<()> {
    let cumulus_evolutions = Evolution::new("1", r#"
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
            'Unreaded',
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
    "#, r#"
        DROP TABLE users_feeds;
        DROP TABLE users_sources;
        DROP TABLE feeds;
        DROP TABLE sources;
        DROP TABLE users;
        DROP TYPE "sourcetype";
        DROP TYPE "reaction";
    "#);

    let migrations = vec![cumulus_evolutions];

    sync(connection, migrations)
}
