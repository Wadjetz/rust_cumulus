use uuid::Uuid;
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::NaiveDateTime;
use chrono::prelude::*;
use postgres::rows::Row;
use r2d2::Pool;
use validator::Validate;

use config;
use errors::*;
use token;
use graphql::auth_query::AuthQuery;
use graphql::auth_mutation::AuthMutation;
use diesel::PgConnection;
use r2d2_diesel::ConnectionManager;
use diesel;
use schema::users;
use diesel::prelude::*;

#[derive(Debug, Queryable, Insertable, GraphQLObject, Validate)]
#[table_name="users"]
pub struct User {
    pub uuid: Uuid,
    #[validate(length(min = "1"))]
    pub login: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "6"))]
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
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
        };
        Ok(user)
    }

    pub fn insert(connection: &PgConnection, user: &User) -> Result<User> {
        Ok(diesel::insert_into(users::table).values(user).get_result(connection)?)
    }

    pub fn find_by_email(connection: &PgConnection, searched_email: &str) -> Result<User> {
        use schema::users::dsl::*;
        Ok(users.filter(email.eq(searched_email)).first::<User>(&*connection)?)
    }

    pub fn find_by_uuid(connection: &PgConnection, searched_uuid: &Uuid) -> Result<User> {
        use schema::users::dsl::*;
        Ok(users.filter(uuid.eq(searched_uuid)).first::<User>(&*connection)?)
    }
}

pub fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password, DEFAULT_COST)?)
}

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

pub fn signup_resolver(pool: &Pool<ConnectionManager<PgConnection>>, user: User) -> Result<String> {
    let connection = pool.get()?;
    let _ = user.validate()?;
    let _ = User::insert(&connection, &user)?;
    let token = token::create_token(user.uuid, user.email, config::CONFIG.secret_key.as_ref())?;
    Ok(token)
}

pub fn login_resolver(pool: &Pool<ConnectionManager<PgConnection>>, email: String, password: String) -> Result<String> {
    let connection = pool.get()?;
    let user = User::find_by_email(&connection, &email)?;
    if let Ok(true) = verify_password(&password, &user.password) {
        Ok(token::create_token(user.uuid, email, config::CONFIG.secret_key.as_ref())?)
    } else {
        Err(ErrorKind::WrongCredentials.into())
    }
}

pub fn auth_resolver<E>(pool: &Pool<ConnectionManager<PgConnection>>, token: String) -> Result<E> where E: From<User> {
    let connection = pool.get()?;
    let auth_data = token::decode_auth(&token, config::CONFIG.secret_key.as_ref())?;
    let user = User::find_by_email(&connection, &auth_data.email)?;
    Ok(user.into())
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
