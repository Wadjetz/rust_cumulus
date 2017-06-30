use juniper::Context;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;


use graphql::auth_query_ql::AuthQuery;
use repositories::user_repository::{verify_user};
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
});
