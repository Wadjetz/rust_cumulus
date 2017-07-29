use uuid::Uuid;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use postgres::rows::Row;

use models::feed_source::FeedSource;
use models::feed::Feed;
use models::feed_source_feed::FeedSourceFeed;
use errors::*;

impl FeedSourceFeed {
    #[allow(dead_code)]
    pub fn from(row: &Row) -> Self {
        FeedSourceFeed {
            uuid: row.get("uuid"),
            feed_uuid: row.get("feed_uuid"),
            feed_source_uuid: row.get("feed_source_uuid"),
        }
    }
}

fn insert_query(connection: &PooledConnection<PostgresConnectionManager>, feed_source: &FeedSource, feed: &Feed) -> Result<u64> {
    Ok(connection.execute(
        "INSERT INTO feeds_sources_feeds (uuid, feed_uuid, feed_source_uuid) VALUES ($1, $2, $3);",
        &[&Uuid::new_v4(), &feed.uuid, &feed_source.uuid]
    )?)
}

pub fn insert(connection: &PooledConnection<PostgresConnectionManager>, feed_source: &FeedSource, feed: &Feed) -> Result<u64> {
    let inerted_rows = insert_query(connection, feed_source, feed)?;
    if inerted_rows == 0 {
        Err(ErrorKind::NotInserted.into())
    } else {
        Ok(inerted_rows)
    }
}
