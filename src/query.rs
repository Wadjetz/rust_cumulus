use juniper::Context;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;

use pg::create_db_pool;
use auth::Auth;
use user_repository::{verify_user};
use token;

pub struct Query {
    pub connection: Pool<PostgresConnectionManager>,
}

impl Query {
    pub fn new() -> Self {
        Query {
            connection: create_db_pool()
        }
    }
}

impl Context for Query {}

graphql_object!(Query: Query as "Query" |&self| {
    description: "The root query object of the schema"

    field auth(
        &executor,
        token: String as "Auth token"
    ) -> Result<Auth, String> as "Auth" {
      let connection = executor.context().connection.clone().get().expect("Error connection pool");
      token::decode_auth(&token)
            .and_then(|auth_data| verify_user(&connection, auth_data))
            .map(Auth::new)
            .map_err(|e| e.description().to_string())
    }
});
