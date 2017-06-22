#![recursion_limit = "1024"]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]
#![feature(custom_derive)]

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
use juniper::RootNode;
use juniper::rocket_handlers;

extern crate jsonwebtoken;
extern crate bcrypt;
extern crate uuid;

mod errors;
mod pg;
mod query;
mod mutation;
mod file;
mod file_ql;
mod token;
mod auth;
mod user;
mod user_repository;
mod user_ql;

use query::Query;
use mutation::Mutation;
use token::AuthData;

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

use rocket::Data;
use std::path::PathBuf;

#[derive(FromForm)]
struct FileForm {
    path: String
}

#[post("/upload/<path..>", data = "<file_data>")]
fn upload(auth_data: AuthData, file_data: Data, path: PathBuf) -> Result<String, String> {
    println!("{:?}", path);
    match file::save_file(file_data, path.clone()) {
        Ok(_) => Ok(format!("Ok")),
        Err(err) => Err(err.description().to_string()),
    }
    //let p = Path::new("upload/").join(path);
    //let f = File::create(p).unwrap();
    //file_data.stream_to_file(f.path())?;
}

#[error(401)]
fn unauthorized() -> JSON<Value> {
    JSON(json!({ "message": "error" }))
}

fn main() {
    rocket::ignite()
        .manage(Query::new())
        .manage(RootNode::new(Query::new(), Mutation))
        .mount("/", routes![graphiql, post_graphql_handler, upload])
        .launch();
}
