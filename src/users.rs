use uuid::Uuid;
use bcrypt::{DEFAULT_COST, hash, verify};
use postgres::rows::Row;
use postgres_shared::types::ToSql;
use juniper::Executor;
use chrono::NaiveDateTime;
use chrono::prelude::*;

use errors::*;
use token;
use graphql::query::Query;
use graphql::auth_query::AuthQuery;
use graphql::auth_mutation::AuthMutation;
use pg::{Insertable, PgDatabase};

#[derive(Debug)]
pub struct User {
    pub uuid: Uuid,
    pub login: String,
    pub email: String,
    pub password: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

impl User {
    pub fn new_secure(login: String, email: String, password: String) -> Result<User> {
        let hashed_password = hash_password(&password)?;
        let user = User {
            uuid: Uuid::new_v4(),
            login,
            email,
            password: hashed_password,
            created: UTC::now().naive_utc(),
            updated: UTC::now().naive_utc(),
        };
        Ok(user)
    }
}

pub fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password, DEFAULT_COST)?)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool> {
    Ok(verify(password, hashed_password)?)
}

graphql_object!(User: Query as "User" |&self| {
    description: "User"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
    }

    field email() -> &String as "email" {
        &self.email
    }

    field login() -> &String as "login" {
        &self.login
    }

    field created() -> String as "created" {
        format!("{}", self.created)
    }

    field updated() -> String as "updated" {
        format!("{}", self.updated)
    }
});

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

    fn insert_params<'a>(&'a self) -> Box<[&'a ToSql]> {
        Box::new([&self.uuid, &self.login, &self.email, &self.password, &self.created, &self.updated])
    }
}

pub fn signup_resolver<'a>(executor: &Executor<'a, Query>, login: String, email: String, password: String) -> Result<String> {
    let connection = executor.context().connection.clone().get()?;
    let pg = PgDatabase::new(connection);
    let user = User::new_secure(login, email, password)?;
    pg.insert(&user)?;
    let token = token::create_token(user.uuid, user.email)?;
    Ok(token)
}

fn find_user_by_email(pg: &PgDatabase, email: &str) -> Result<Option<User>> {
    let query = r#"SELECT * FROM users WHERE email = $1;"#;
    Ok(pg.find_one::<User>(query, &[&email])?)
}

pub fn login_resolver<'a>(executor: &Executor<'a, Query>, email: String, password: String) -> Result<String> {
    let connection = executor.context().connection.clone().get()?;
    let pg = PgDatabase::new(connection);
    if let Some(user) = find_user_by_email(&pg, &email)? {
        if let Ok(true) = verify_password(&password, &user.password) {
            Ok(token::create_token(user.uuid, email)?)
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

pub fn auth_resolver<'a, E>(executor: &Executor<'a, Query>, token: String) -> Result<E> where E: From<User> {
    let connection = executor.context().connection.clone().get()?;
    let pg = PgDatabase::new(connection);
    let auth_data = token::decode_auth(&token)?;
    if let Some(user) = find_user_by_email(&pg, &auth_data.email)? {
        Ok(user.into())
    } else {
        Err(ErrorKind::WrongCredentials.into())
    }
}
