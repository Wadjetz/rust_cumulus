#![recursion_limit = "1024"]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]
#![feature(custom_derive)]

#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", allow(needless_pass_by_value))]

//#[macro_use] extern crate validator_derive;
//extern crate validator;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate postgres;
#[macro_use]
extern crate postgres_shared;
extern crate postgres_protocol;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate strum;
#[macro_use]
extern crate strum_macros;

#[macro_use]
extern crate rocket_contrib;
extern crate rocket;
use rocket::response::content;
use rocket::State;
use rocket_contrib::{JSON, Value};

#[macro_use]
extern crate juniper;
use juniper::RootNode;
use juniper::rocket_handlers;

extern crate jsonwebtoken;
extern crate bcrypt;
extern crate crypto;
extern crate uuid;
extern crate chrono;

mod errors;
mod pg;
mod models;
mod repositories;
mod token;
mod graphql;
mod upload;
mod app_state;

use graphql::query::Query;
use graphql::mutation::Mutation;

use app_state::AppState;
use pg::create_db_pool;

type Schema = RootNode<'static, Query, Mutation>;

#[get("/")]
fn graphiql() -> content::HTML<String> {
    rocket_handlers::graphiql_source("/graphql")
}

#[post("/graphql", data="<request>")]
fn post_graphql_handler(
    context: State<Query>,
    request: rocket_handlers::GraphQLRequest,
    schema: State<Schema>
) -> rocket_handlers::GraphQLResponse {
    request.execute(&schema, &context)
}

#[error(401)]
fn unauthorized() -> JSON<Value> {
    JSON(json!({ "message": "error" }))
}

fn main() {
    let connection = create_db_pool();
    rocket::ignite()
        .manage(Query::new(connection.clone()))
        .manage(AppState::new(connection.clone()))
        .manage(RootNode::new(Query::new(connection), Mutation))
        .mount("/", routes![graphiql, post_graphql_handler, upload::upload])
        .launch();
}
