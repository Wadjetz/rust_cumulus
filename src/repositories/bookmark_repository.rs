use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use postgres::rows::Rows;
use postgres::error::Error;
use postgres_shared::error::{SqlState};
use users::User;
use bookmarks::Bookmark;

use errors::*;

fn insert_query(connection: &PooledConnection<PostgresConnectionManager>, bookmark: &Bookmark) -> Result<u64> {
    connection.execute(
        "INSERT INTO bookmarks (uuid, title, url, description, path, created, updated, user_uuid) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        &[&bookmark.uuid, &bookmark.title, &bookmark.url, &bookmark.description, &bookmark.path, &bookmark.created, &bookmark.updated, &bookmark.user_uuid]
    ).map_err(|e| {
        println!("{:?}", e);
        match e {
            Error::Db(ref e) if e.code == SqlState::UniqueViolation => ErrorKind::AlreadyExist.into(),
            e => e.into(),
        }
    })
}

pub fn insert(connection: &PooledConnection<PostgresConnectionManager>, bookmark: &Bookmark) -> Result<u64> {
    let inerted_rows = insert_query(connection, bookmark)?;
    if inerted_rows == 0 {
        Err(ErrorKind::NotInserted.into())
    } else {
        Ok(inerted_rows)
    }
}

fn find_query(connection: &PooledConnection<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Rows<'static>> {
    let bookmarks = connection.query(
      "SELECT * FROM bookmarks WHERE user_uuid = $1::uuid LIMIT $2::int OFFSET $3::int;",
      &[&user.uuid, &limit, &offset]
    )?;
    Ok(bookmarks)
}

pub fn find(connection: &PooledConnection<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Vec<Bookmark>> {
    let rows = find_query(connection, limit, offset, user)?;
    let bookmarks = rows.iter().map(|row| row.into()).collect();
    Ok(bookmarks)
}


fn find_by_url_and_user_query(connection: &PooledConnection<PostgresConnectionManager>, url: &str, user: &User) -> Result<Rows<'static>> {
    let user = connection.query(
        "SELECT * FROM bookmarks WHERE url = $1 AND user_uuid = $2::uuid",
        &[&url, &user.uuid]
    )?;
    Ok(user)
}

pub fn find_by_url_and_user(connection: &PooledConnection<PostgresConnectionManager>, url: &str, user: &User) -> Result<Option<Bookmark>> {
    let rows = find_by_url_and_user_query(connection, url, user)?;
    let mut users: Vec<Bookmark> = rows.iter().map(|row| row.into()).collect();
    let user = users.pop();
    Ok(user)
}
