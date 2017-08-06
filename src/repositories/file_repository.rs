use uuid::Uuid;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use postgres::rows::Rows;

use users::User;
use files::File;
use errors::*;

fn find_query(connection: &PooledConnection<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Rows<'static>> {
    let files = connection.query(
      "SELECT * FROM files WHERE user_uuid = $1::uuid LIMIT $2::int OFFSET $3::int;",
      &[&user.uuid, &limit, &offset]
    )?;
    Ok(files)
}

pub fn find(connection: &PooledConnection<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Vec<File>> {
    let rows = find_query(connection, limit, offset, user)?;
    let files = rows.iter().map(|row| row.into()).collect();
    Ok(files)
}

fn find_by_uuid_query(connection: &PooledConnection<PostgresConnectionManager>, uuid: &Uuid) -> Result<Rows<'static>> {
    let files = connection.query(
        "SELECT * FROM files WHERE uuid = $1",
        &[uuid]
    )?;
    Ok(files)
}

pub fn find_by_uuid(connection: &PooledConnection<PostgresConnectionManager>, uuid: &Uuid) -> Result<File> {
    let rows = find_by_uuid_query(connection, uuid)?;
    let mut files: Vec<File> = rows.iter().map(|row| row.into()).collect();
    match files.pop() {
        Some(file) => Ok(file),
        _ => Err(ErrorKind::NotFound.into())
    }
}
