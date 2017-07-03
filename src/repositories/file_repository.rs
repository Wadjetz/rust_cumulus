use uuid::Uuid;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use postgres::rows::Row;
use postgres::rows::Rows;
use postgres::error::Error;
use postgres_shared::error::{SqlState};

use models::user::User;
use models::file::File;
use errors::*;

impl File {
    pub fn from(row: &Row) -> Self {
        File {
            uuid: row.get("uuid"),
            hash: row.get("hash"),
            name: row.get("name"),
            location: row.get("location"),
            file_type: row.get("file_type"),
            size: row.get("size"),
            user_uuid: row.get("user_uuid"),
        }
    }
}

fn insert_query(connection: &PooledConnection<PostgresConnectionManager>, file: &File) -> Result<u64> {
    connection.execute(
        "INSERT INTO files (uuid, hash, name, location, file_type, size, user_uuid) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[
          &file.uuid,
          &file.hash,
          &file.name,
          &file.location,
          &file.file_type,
          &file.size,
          &file.user_uuid
        ]
    ).map_err(|e| {
        println!("{:?}", e);
        match e {
            Error::Db(ref e) if e.code == SqlState::UniqueViolation => ErrorKind::AlreadyExist.into(),
            e => e.into(),
        }
    })
}

pub fn insert(connection: &PooledConnection<PostgresConnectionManager>, file: &File) -> Result<u64> {
    let inerted_rows = insert_query(connection, file)?;
    if inerted_rows == 0 {
        Err(ErrorKind::NotInserted.into())
    } else {
        Ok(inerted_rows)
    }
}

fn find_query(connection: &PooledConnection<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Rows<'static>> {
    let files = connection.query(
      "SELECT * FROM files WHERE user_uuid = $1::uuid LIMIT $2::int OFFSET $3::int;",
      &[&user.uuid, &limit, &offset]
    )?;
    Ok(files)
}

pub fn find(connection: &PooledConnection<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Vec<File>> {
    let rows = find_query(connection, limit, offset, user)?;
    let files = rows.iter().map(|row| File::from(&row)).collect();
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
    let mut files: Vec<File> = rows.iter().map(|row| File::from(&row)).collect();
    match files.pop() {
        Some(file) => Ok(file),
        _ => Err(ErrorKind::NotFound.into())
    }
}
