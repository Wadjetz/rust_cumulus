#![recursion_limit = "1024"]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]
#![feature(custom_derive)]

#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", allow(needless_pass_by_value))]

#[macro_use] extern crate rocket_contrib;
extern crate serde;
//#[macro_use] extern crate validator_derive;
//extern crate validator;
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
use rocket::Data;
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

use uuid::Uuid;

mod errors;
mod pg;
mod file;
mod token;
mod user;
mod user_repository;
mod graphql;
mod bookmark;
mod bookmark_repository;

use graphql::query::Query;
use graphql::mutation::Mutation;
use token::AuthData;

use std::path::PathBuf;

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

use std::os::unix::fs::MetadataExt;

#[post("/upload/<path..>", data = "<file_data>")]
fn upload(_auth_data: AuthData, file_data: Data, path: PathBuf) -> Result<String, String> {
    let maybe_file_name = path.file_name()
                  .and_then(|os_str| os_str.to_str())
                  .map(|s| s.to_string());
    let maybe_parent_path = path.parent();
    match file::save_file(file_data, path.clone()) {
        Ok((hash, metadata)) => {
          let file_name = maybe_file_name.unwrap_or(hash.clone());
          let parent = maybe_parent_path
                          .and_then(|path| path.to_str())
                          .map(|s| s.to_string())
                          .unwrap_or("/".to_string());
          let f = file::File::new(
              Uuid::new_v4(),
              hash,
              file_name.clone(),
              parent.clone(),
              format!("{}/{}", parent, file_name),
              file::FileType::File,
              metadata.size(),
          );
          println!("File -> {:?}", f);
          Ok(format!("Ok"))
        },
        Err(err) => Err(err.description().to_string()),
    }
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
