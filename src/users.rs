use uuid::Uuid;
use bcrypt::verify;
use postgres::rows::Row;
use postgres::types::ToSql;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use config;
use errors::*;
use token;
use graphql::auth_query::AuthQuery;
use graphql::auth_mutation::AuthMutation;
use pg::{Insertable, PgDatabase};
use user::User;

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool> {
    Ok(verify(password, hashed_password)?)
}

impl<'a> From<Row<'a>> for User {
    fn from(row: Row) -> Self {
        User {
            uuid: row.get("uuid"),
            login: row.get("login"),
            email: row.get("email"),
            password: row.get("password"),
            created: row.get("created"),
            updated: row.get("updated"),
        }
    }
}

impl Insertable for User {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO users (uuid, login, email, password, created, updated)
            VALUES ($1, $2, $3, $4, $5, $6);
        "#.to_owned()
    }

    fn insert_params(&self) -> Box<[&ToSql]> {
        Box::new([&self.uuid, &self.login, &self.email, &self.password, &self.created, &self.updated])
    }
}

pub fn signup_resolver(pool: Pool<PostgresConnectionManager>, user: User) -> Result<String> {
    let pg = PgDatabase::from_pool(pool)?;
    pg.insert(&user)?;
    let token = token::create_token(user.uuid, user.email, config::CONFIG.secret_key.as_ref())?;
    Ok(token)
}

fn find_user_by_email(pg: &PgDatabase, email: &str) -> Result<Option<User>> {
    let query = r#"SELECT * FROM users WHERE email = $1;"#;
    Ok(pg.find_one::<User>(query, &[&email])?)
}

#[allow(dead_code)]
pub fn find_user_by_uuid(pg: &PgDatabase, uuid: &Uuid) -> Result<Option<User>> {
    let query = r#"SELECT * FROM users WHERE uuid = $1::uuid;"#;
    Ok(pg.find_one::<User>(query, &[&uuid])?)
}

pub fn login_resolver(pool: Pool<PostgresConnectionManager>, email: String, password: String) -> Result<String> {
    let pg = PgDatabase::from_pool(pool)?;
    if let Some(user) = find_user_by_email(&pg, &email)? {
        if let Ok(true) = verify_password(&password, &user.password) {
            Ok(token::create_token(user.uuid, email, config::CONFIG.secret_key.as_ref())?)
        } else {
            Err(ErrorKind::WrongCredentials.into())
        }
    } else {
        Err(ErrorKind::WrongCredentials.into())
    }
}

impl From<User> for AuthQuery {
    fn from(user: User) -> Self {
        AuthQuery::new(user)
    }
}

impl From<User> for AuthMutation {
    fn from(user: User) -> Self {
        AuthMutation::new(user)
    }
}

pub fn auth_resolver<E>(pool: Pool<PostgresConnectionManager>, token: String) -> Result<E> where E: From<User> {
    let pg = PgDatabase::from_pool(pool)?;
    let auth_data = token::decode_auth(&token, config::CONFIG.secret_key.as_ref())?;
    if let Some(user) = find_user_by_email(&pg, &auth_data.email)? {
        Ok(user.into())
    } else {
        Err(ErrorKind::WrongCredentials.into())
    }
}
