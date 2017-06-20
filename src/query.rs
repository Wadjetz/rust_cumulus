use juniper::Context;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;

use pg::create_db_pool;
use file::File;
use auth::Auth;
use user::User;
use user_repository::{verify_user, find_by_email};
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

    field files() -> Vec<File> as "Files" {
        vec![File::new("lol".to_string(), "toto".to_string(), 5)]
    }

    field test(&executor) -> String as "Test" {
        let c = executor.context().connection.clone().get().expect("Error connection pool");
        let rows = c.query("SELECT 1 + 2 AS test", &[]).unwrap();
        let r: Vec<i32> = rows.iter().map(|ref row| row.get("test")).collect();
        format!("{:?}", r)
    }

    field user(&executor, email: String as "email") -> Option<User> as "User" {
        let connection = executor.context().connection.clone().get().expect("Error connection pool");
        find_by_email(&connection, &email).ok()
    }

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
