use uuid::Uuid;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use postgres::rows::Row;
use postgres::rows::Rows;
use postgres::error::Error;
use postgres_shared::error::{SqlState};

use models::user::User;
use feeds::Feed;
use errors::*;

use serde_json;

impl Feed {
    pub fn from(row: &Row) -> Self {
        Feed {
            uuid: row.get("uuid"),
            url: row.get("url"),
            rss: serde_json::from_value(row.get("rss")).ok(),
            readable: serde_json::from_value(row.get("readable")).ok(),
            twitter: row.get("twitter"),
            created: row.get("created"),
            updated: row.get("updated"),
            source_uuid: row.get("source_uuid"),
        }
    }
}

struct UserFeed {
    pub uuid: Uuid,
    pub feed_uuid: Uuid,
    pub user_uuid: Uuid,
}

impl UserFeed {
    #[allow(dead_code)]
    pub fn from(row: &Row) -> Self {
        UserFeed {
            uuid: row.get("uuid"),
            feed_uuid: row.get("feed_uuid"),
            user_uuid: row.get("user_uuid"),
        }
    }
}

fn find_by_url_query(connection: &PooledConnection<PostgresConnectionManager>, url: &str) -> Result<Rows<'static>> {
    let feed = connection.query(
        "SELECT * FROM feeds WHERE url = $1",
        &[&url]
    )?;
    Ok(feed)
}

pub fn find_by_url(connection: &PooledConnection<PostgresConnectionManager>, url: &str) -> Result<Option<Feed>> {
    let rows = find_by_url_query(connection, url)?;
    let mut feeds: Vec<Feed> = rows.iter().map(|row| Feed::from(&row)).collect();
    let feed = feeds.pop();
    Ok(feed)
}

