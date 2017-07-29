use uuid::Uuid;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use postgres::rows::Row;
use postgres::rows::Rows;

use models::user::User;
use models::feed_source::FeedSource;
use models::user_feed_source::UserFeedSource;
use errors::*;

impl UserFeedSource {
    pub fn from(row: &Row) -> Self {
        UserFeedSource {
            uuid: row.get("uuid"),
            user_uuid: row.get("user_uuid"),
            feed_source_uuid: row.get("feed_source_uuid"),
        }
    }
}

fn find_user_feed_source_query(connection: &PooledConnection<PostgresConnectionManager>, feed_source: &FeedSource, user: &User) -> Result<Rows<'static>> {
    let users_feeds_sources_number = connection.query(
        "SELECT * FROM users_feeds_sources WHERE user_uuid = $1::uuid AND feed_source_uuid = $2::uuid;",
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
        "INSERT INTO users_feeds_sources (uuid, user_uuid, feed_source_uuid) VALUES ($1::uuid, $2::uuid, $3::uuid)",
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
	            JOIN users_feeds_sources ON users_feeds_sources.feed_source_uuid = feeds_sources.uuid
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

