use juniper::Context;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;


use graphql::auth_query_ql::AuthQuery;
use repositories::user_repository::{verify_user};
use repositories::feed_source_repository;
use models::feed_source::FeedSource;
use sources::Source;
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

    field feeds_sources(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset",
    ) -> Result<Vec<FeedSource>, String> {
        let connection = executor.context().connection.clone().get().expect("Error connection pool");
        feed_source_repository::find(&connection, limit.unwrap_or(50), offset.unwrap_or(0))
            .map_err(|e| e.description().to_string())
    }

    field sources(
        &executor,
        limit: Option<i32> as "Limit",
        offset: Option<i32> as "Offset",
    ) -> Result<Vec<Source>, String> {
        sources::find_resolver(executor, limit.unwrap_or(50), offset.unwrap_or(0)).map_err(|e| e.description().to_string())
    }
});
