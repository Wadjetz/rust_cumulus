use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use postgres::rows::Row;
use postgres::rows::Rows;
use postgres::error::Error;
use postgres_shared::error::{SqlState};
use token::AuthData;
use user::User;

use errors::*;

impl User {
    pub fn from(row: &Row) -> Self {
        User::new(
            row.get("uuid"),
            row.get("login"),
            row.get("email"),
            row.get("password"),
        )
    }
}

fn insert_query(connection: &PooledConnection<PostgresConnectionManager>, user: &User) -> Result<u64> {
    connection.execute(
        "INSERT INTO users (uuid, login, email, password) VALUES ($1, $2, $3, $4)",
        &[&user.uuid, &user.login, &user.email, &user.password]
    ).map_err(|e| {
        match e {
            Error::Db(ref e) if e.code == SqlState::UniqueViolation => ErrorKind::AlreadyExist.into(),
            e => e.into(),
        }
    })
}

pub fn insert(connection: &PooledConnection<PostgresConnectionManager>, user: &User) -> Result<u64> {
    let inerted_rows = insert_query(connection, user)?;
    if inerted_rows == 0 {
        Err(ErrorKind::NotInserted.into())
    } else {
        Ok(inerted_rows)
    }
}

fn find_by_email_query(connection: &PooledConnection<PostgresConnectionManager>, searched_email: &str) -> Result<Rows<'static>> {
    let user = connection.query(
        "SELECT * FROM users WHERE email = $1",
        &[&searched_email]
    )?;
    Ok(user)
}

pub fn find_by_email(connection: &PooledConnection<PostgresConnectionManager>, searched_email: &str) -> Result<User> {
    let rows = find_by_email_query(connection, searched_email)?;
    let mut users: Vec<User> = rows.iter().map(|row| User::from(&row)).collect();
    match users.pop() {
        Some(user) => Ok(user),
        _ => Err(ErrorKind::NotFound.into())
    }
}

pub fn verify_user(connection: &PooledConnection<PostgresConnectionManager>, auth_data: AuthData) -> Result<User> {
  let user = find_by_email(connection, &auth_data.email)?;
  Ok(user)
}
