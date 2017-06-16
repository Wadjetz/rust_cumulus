#![recursion_limit = "1024"]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", allow(needless_pass_by_value))]

#[macro_use] extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate postgres;
extern crate postgres_shared;
extern crate r2d2;
extern crate r2d2_postgres;

extern crate rocket;
use rocket::response::content;
use rocket::State;
use rocket_contrib::{JSON, Value};

#[macro_use]
extern crate juniper;
use juniper::{EmptyMutation, RootNode};
use juniper::rocket_handlers;

extern crate jsonwebtoken;
extern crate uuid;

mod errors;
mod pg;
mod query;
mod file;
mod token;
mod auth;
mod user;
mod user_repository;

use query::Query;
use token::AuthData;

type Schema = RootNode<'static, Query, EmptyMutation<Query>>;

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

#[get("/upload")]
fn upload(auth_data: AuthData) -> String {
    auth_data.email
}

#[error(401)]
fn unauthorized() -> JSON<Value> {
    JSON(json!({ "message": "error" }))
}

fn main() {
    rocket::ignite()
        .manage(Query::new())
        .manage(Schema::new(Query::new(), EmptyMutation::<Query>::new()))
        .mount("/", routes![graphiql, post_graphql_handler, upload])
        .launch();
}
