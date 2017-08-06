use juniper::Context;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;

use graphql::auth_query::AuthQuery;
use sources::Source;
use std::error::Error;
use sources::find_sources_resolver;
use users::{auth_resolver, login_resolver};

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
        auth_resolver(executor, token).map_err(|e| e.description().to_string())
    }

    field login(
        &executor,
        email: String as "Email",
        password: String as "Password"
    ) -> Result<String, String> as "Token" {
        login_resolver(executor, email, password).map_err(|e| e.description().to_string())
    }

    field sources(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset",
    ) -> Result<Vec<Source>, String> {
        find_sources_resolver(executor, limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0)).map_err(|e| e.description().to_string())
    }
});
