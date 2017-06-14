#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", allow(needless_pass_by_value))]

//#[macro_use] extern crate rocket_contrib;
//#[macro_use] extern crate serde_derive;
extern crate dotenv;
extern crate postgres;
extern crate postgres_shared;
extern crate r2d2;
extern crate r2d2_postgres;

extern crate rocket;
use rocket::response::content;
use rocket::State;

#[macro_use]
extern crate juniper;
use juniper::{EmptyMutation, RootNode};
use juniper::rocket_handlers;
use juniper::Context;

mod pg;

use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;
use pg::create_db_pool;

struct File {
    id: String,
    path: String,
    size: u64,
}

impl File {
    pub fn new(id: String, path: String, size: u64) -> Self {
        File {
            id, path, size
        }
    }
}

struct Repository {
    pub files: Vec<File>,
    pub connection: Pool<PostgresConnectionManager>,
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            files: vec![],
            connection: create_db_pool()
        }
    }
}

impl Context for Repository {}

graphql_object!(File: Repository as "File" |&self| {
    description: "A file"

    field id() -> String as "id" {
        self.id.to_string()
    }

    field path() -> String as "path" {
        self.path.to_string()
    }

    field size() -> String as "size" {
        format!("{}", self.size)
    }
});

graphql_object!(Repository: Repository as "Query" |&self| {
    description: "The root query object of the schema"

    field files() -> Vec<File> as "Files" {
        vec![File::new("lol".to_string(), "toto".to_string(), 5)]
    }

    field test(&executor) -> String as "Test" {
        let c = executor.context().connection.clone().get().expect("Error connection pool");;
        let rows = c.query("SELECT 1 + 2 AS test", &[]).unwrap();
        let r: Vec<i32> = rows.iter().map(|ref row| row.get("test")).collect();
        format!("{:?}", r)
    }
});

type Schema = RootNode<'static, Repository, EmptyMutation<Repository>>;

#[get("/")]
fn graphiql() -> content::HTML<String> {
    rocket_handlers::graphiql_source("/graphql")
}

#[post("/graphql", data="<request>")]
fn post_graphql_handler(
    context: State<Repository>,
    request: rocket_handlers::GraphQLRequest,
    schema: State<Schema>
) -> rocket_handlers::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(Repository::new())
        .manage(Schema::new(Repository::new(), EmptyMutation::<Repository>::new()))
        .mount("/", routes![graphiql, post_graphql_handler])
        .launch();
}
