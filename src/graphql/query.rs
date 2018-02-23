use std::error::Error;

use juniper::{FieldResult, Context, RootNode, FieldError};
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use diesel::PgConnection;

use graphql::auth_query::AuthQuery;
use graphql::mutation::Mutation;
use mindstream::sources::{Source, find_sources_resolver};
use users::{auth_resolver, login_resolver};

pub struct Query {
    pub connection: Pool<PostgresConnectionManager>,
    pub diesel_pool: Pool<ConnectionManager<PgConnection>>,
}

impl Query {
    pub fn new(connection: Pool<PostgresConnectionManager>, diesel_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Query { connection, diesel_pool }
    }
}

impl Context for Query {}

pub type Schema = RootNode<'static, Query, Mutation>;

const DEFAULT_LIMIT: i32 = 10;

graphql_object!(Query: Query as "Query" |&self| {
    description: "The root query object of the schema"

    field auth(
        &executor,
        token: String as "Auth token"
    ) -> FieldResult<AuthQuery> as "Auth" {
        auth_resolver(executor.context().connection.clone(), token)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field login(
        &executor,
        email: String as "Email",
        password: String as "Password"
    ) -> FieldResult<String> as "Token" {
        login_resolver(executor.context().connection.clone(), email, password)
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }

    field sources(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset",
    ) -> FieldResult<Vec<Source>> {
        find_sources_resolver(executor.context().connection.clone(), limit.unwrap_or(DEFAULT_LIMIT), offset.unwrap_or(0))
            .map_err(|e| FieldError::from(&e.description().to_string()))
    }
});
