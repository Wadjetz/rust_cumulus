use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use postgres::rows::Row;
use postgres::rows::Rows;
use token::AuthData;
use users::User;

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
