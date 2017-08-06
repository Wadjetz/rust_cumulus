use juniper::Context;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;

use users::{verify_password};
use repositories::user_repository;
use graphql::auth_query_ql::AuthQuery;
use repositories::user_repository::{verify_user};
use sources::Source;
use std::error::Error;
use errors::ErrorKind;
use sources;
use token;

pub struct Query {
    pub connection: Pool<PostgresConnectionManager>,
}

impl Query {
    pub fn new(connection: Pool<PostgresConnectionManager>) -> Self {
        Query { connection }
    }
}

impl Context for Query {}

const DEFAULT_LIMIT: i32 = 10;

graphql_object!(Query: Query as "Query" |&self| {
    description: "The root query object of the schema"

    field auth(
        &executor,
        token: String as "Auth token"
    ) -> Result<AuthQuery, String> as "Auth" {
      let connection = executor.context().connection.clone().get().expect("Error connection pool");
      token::decode_auth(&token)
            .and_then(|auth_data| verify_user(&connection, auth_data))
            .map(AuthQuery::new)
            .map_err(|e| e.description().to_string())
    }

    field login(
        &executor,
        email: String as "Email",
        password: String as "Password"
    ) -> Result<String, String> as "Token" {
        let connection = executor.context().connection.clone().get().map_err(|e| e.description().to_string())?;
        user_repository::find_by_email(&connection, &email)
            .and_then(|user| {
                match verify_password(&password, &user.password) {
                    Ok(true) => token::create_token(user.uuid, email),
                    Ok(false) => Err(ErrorKind::WrongCredentials.into()),
                    Err(e) => Err(ErrorKind::WrongCredentials.into()),
                }
            })
            .map_err(|e| e.description().to_string())
    }

    field sources(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset",
    ) -> Result<Vec<Source>, String> {
        sources::find_resolver(executor, limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0)).map_err(|e| e.description().to_string())
    }
});
