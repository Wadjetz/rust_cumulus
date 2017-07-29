use uuid::Uuid;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use postgres::rows::Row;
use postgres::rows::Rows;
use postgres::error::Error;
use postgres_shared::error::{SqlState};

use models::user::User;
use models::feed_source::FeedSource;
use models::user_feed_source::UserFeedSource;
use errors::*;

impl FeedSource {
    pub fn from(row: &Row) -> Self {
        FeedSource {
            uuid: row.get("uuid"),
            title: row.get("title"),
            xml_url: row.get("xml_url"),
            html_url: row.get("html_url"),
            error: row.get("error"),
            created: row.get("created"),
            updated: row.get("updated"),
        }
    }
}

fn insert_query(connection: &PooledConnection<PostgresConnectionManager>, feed_source: &FeedSource) -> Result<u64> {
    connection.execute(
        "INSERT INTO feeds_sources (uuid, title, xml_url, html_url, error, created, updated) VALUES ($1, $2, $3, $4, $5, $6, $7);",
        &[
            &feed_source.uuid,
            &feed_source.title,
            &feed_source.xml_url,
            &feed_source.html_url,
            &feed_source.error,
            &feed_source.created,
            &feed_source.updated,
        ]
    ).map_err(|e| {
        println!("{:?}", e);
        match e {
            Error::Db(ref e) if e.code == SqlState::UniqueViolation => ErrorKind::AlreadyExist.into(),
            e => e.into(),
        }
    })
}

pub fn insert(connection: &PooledConnection<PostgresConnectionManager>, feed_source: &FeedSource) -> Result<u64> {
    let inerted_rows = insert_query(connection, feed_source)?;
    if inerted_rows == 0 {
        Err(ErrorKind::NotInserted.into())
    } else {
        Ok(inerted_rows)
    }
}

fn find_query(connection: &PooledConnection<PostgresConnectionManager>, limit: i32, offset: i32) -> Result<Rows<'static>> {
    let bookmarks = connection.query(
      "SELECT * FROM feeds_sources LIMIT $1::int OFFSET $2::int;",
      &[&limit, &offset]
    )?;
    Ok(bookmarks)
}

pub fn find(connection: &PooledConnection<PostgresConnectionManager>, limit: i32, offset: i32) -> Result<Vec<FeedSource>> {
    let rows = find_query(connection, limit, offset)?;
    let feeds_sources = rows.iter().map(|row| FeedSource::from(&row)).collect();
    Ok(feeds_sources)
}


fn find_by_uuid_query(connection: &PooledConnection<PostgresConnectionManager>, uuid: &Uuid) -> Result<Rows<'static>> {
    let user = connection.query(
        "SELECT * FROM feeds_sources WHERE uuid = $1::uuid;",
        &[&uuid]
    )?;
    Ok(user)
}

pub fn find_by_uuid(connection: &PooledConnection<PostgresConnectionManager>, uuid: &Uuid) -> Result<Option<FeedSource>> {
    let rows = find_by_uuid_query(connection, uuid)?;
    let mut users: Vec<FeedSource> = rows.iter().map(|row| FeedSource::from(&row)).collect();
    let user = users.pop();
    Ok(user)
}
