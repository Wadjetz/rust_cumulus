use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use postgres::rows::Row;
use postgres::rows::Rows;
use postgres::error::Error;
use postgres_shared::error::{SqlState};

use models::user::User;
use models::feed::Feed;
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
        }
    }
}

fn insert_query(connection: &PooledConnection<PostgresConnectionManager>, feed: &Feed) -> Result<u64> {
    connection.execute(
        "INSERT INTO feeds (uuid, url, rss, readable, twitter, created, updated) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[&feed.uuid, &feed.url, &serde_json::to_value(&feed.rss).ok(), &serde_json::to_value(&feed.readable).ok(), &feed.twitter, &feed.created, &feed.updated]
    ).map_err(|e| {
        match e {
            Error::Db(ref e) if e.code == SqlState::UniqueViolation => ErrorKind::AlreadyExist.into(),
            e => e.into(),
        }
    })
}

pub fn insert(connection: &PooledConnection<PostgresConnectionManager>, feed: &Feed) -> Result<u64> {
    let inerted_rows = insert_query(connection, feed)?;
    if inerted_rows == 0 {
        Err(ErrorKind::NotInserted.into())
    } else {
        Ok(inerted_rows)
    }
}

fn find_query(connection: &PooledConnection<PostgresConnectionManager>, limit: i32, offset: i32, _user: &User) -> Result<Rows<'static>> {
    let feeds = connection.query(
      "SELECT * FROM feeds ORDER BY created DESC LIMIT $1::int OFFSET $2::int;",
      &[&limit, &offset]
    )?;
    Ok(feeds)
}

pub fn find(connection: &PooledConnection<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Vec<Feed>> {
    let rows = find_query(connection, limit, offset, user)?;
    let feeds = rows.iter().map(|row| Feed::from(&row)).collect();
    Ok(feeds)
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

