use uuid::Uuid;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use postgres::rows::Row;
use postgres::rows::Rows;
use postgres::error::Error;
use postgres_shared::error::{SqlState};

use models::user::User;
use models::feed_source::FeedSource;
use errors::*;

struct UserFeedSource {
    uuid: Uuid,
    user_uuid: Uuid,
    feeds_sources_uuid: Uuid,
}

impl UserFeedSource {
    pub fn from(row: &Row) -> Self {
        UserFeedSource {
            uuid: row.get("uuid"),
            user_uuid: row.get("user_uuid"),
            feeds_sources_uuid: row.get("feeds_sources_uuid"),
        }
    }
}

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

fn find_user_feed_source_query(connection: &PooledConnection<PostgresConnectionManager>, feed_source: &FeedSource, user: &User) -> Result<Rows<'static>> {
    let users_feeds_sources_number = connection.query(
        "SELECT * FROM users_feeds_sources WHERE user_uuid = $1::uuid AND feeds_sources_uuid = $2::uuid;",
        &[&user.uuid, &feed_source.uuid]
    )?;
    Ok(users_feeds_sources_number)
}

fn find_user_feed_source(connection: &PooledConnection<PostgresConnectionManager>, feed_source: &FeedSource, user: &User) -> Result<Option<UserFeedSource>> {
    let rows = find_user_feed_source_query(connection, feed_source, &user)?;
    let mut user_feeds_sources: Vec<UserFeedSource> = rows.iter().map(|row| UserFeedSource::from(&row)).collect();
    Ok(user_feeds_sources.pop())
}

#[allow(dead_code)]
fn follow_feed_source_query(connection: &PooledConnection<PostgresConnectionManager>, feed_source: &FeedSource, user: &User) -> Result<u64> {
    Ok(connection.execute(
        "INSERT INTO users_feeds_sources (uuid, user_uuid, feeds_sources_uuid) VALUES ($1::uuid, $2::uuid, $3::uuid)",
        &[&Uuid::new_v4(), &user.uuid, &feed_source.uuid]
    )?)
}

pub fn follow_feed_source(connection: &PooledConnection<PostgresConnectionManager>, feed_source: &FeedSource, user: &User) -> Result<u64> {
    if let None = find_user_feed_source(connection, feed_source, user)? {
        let inerted_rows = follow_feed_source_query(connection, feed_source, user)?;
        if inerted_rows == 0 {
            Err(ErrorKind::NotInserted.into())
        } else {
            Ok(inerted_rows)
        }
    } else {
        Err(ErrorKind::AlreadyExist.into())
    }
}

fn find_by_user_query(connection: &PooledConnection<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Rows<'static>> {
    let bookmarks = connection.query(r#"
            SELECT * FROM feeds_sources
	            JOIN users_feeds_sources ON users_feeds_sources.feeds_sources_uuid = feeds_sources.uuid
            WHERE users_feeds_sources.user_uuid = $3::uuid
            LIMIT $1::int OFFSET $2::int;
        "#,
        &[&limit, &offset, &user.uuid]
    )?;
    Ok(bookmarks)
}

pub fn find_by_user(connection: &PooledConnection<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Vec<FeedSource>> {
    let rows = find_by_user_query(connection, limit, offset, user)?;
    let feeds_sources = rows.iter().map(|row| FeedSource::from(&row)).collect();
    Ok(feeds_sources)
}
