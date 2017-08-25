#![recursion_limit = "1024"]
#![feature(plugin, custom_attribute, custom_derive, associated_type_defaults)]
#![plugin(clippy, rocket_codegen)]
#![allow(unused_doc_comment, op_ref, unused_io_amount, clone_on_copy, double_parens, needless_pass_by_value)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate dotenv;
extern crate jsonwebtoken;
extern crate bcrypt;
extern crate crypto;
extern crate uuid;
extern crate chrono;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate postgres;
#[macro_use]
extern crate postgres_shared;
#[macro_use]
extern crate postgres_derive;
extern crate postgres_protocol;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate feed_rs;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate rocket;
extern crate url;
#[macro_use]
extern crate juniper;
extern crate geo;

mod errors;
mod pg;
mod migration;
mod migrations;
mod file_system;
mod token;
mod graphql;
mod config;
mod services;
mod sources;
mod feeds;
mod users;
mod users_sources;
mod users_feeds;
mod bookmarks;
mod files;

use std::path::{Path, PathBuf};
use std::fs::File as FsFile;
use std::error::Error;

use dotenv::dotenv;
use rocket::response::{NamedFile, content};
use rocket::{Data, State};
use rocket::http::RawStr;
use juniper::RootNode;
use juniper::rocket_handlers;

use graphql::query::Query;
use graphql::mutation::Mutation;
use pg::{DbConn, create_db_pool};
use services::rss_job;
use files::{download_resolver, upload_resolver};
use token::AuthData;

type Schema = RootNode<'static, Query, Mutation>;

#[get("/graphql")]
fn graphiql() -> content::Html<String> {
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

#[get("/")]
fn index() -> Option<NamedFile> {
    let path = Path::new("./static/index.html");
    NamedFile::open(path).ok()
}

#[get("/assets/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("./static/").join(file);
    NamedFile::open(path).ok()
}

#[post("/upload/<path..>", data = "<file_data>")]
pub fn upload(auth_data: AuthData, conn: DbConn, file_data: Data, path: PathBuf) -> Result<String, String> {
    upload_resolver(conn.into(), file_data, path, auth_data)
        .map_err(|err| err.description().to_string())
}

#[get("/download/<file_uuid>")]
pub fn download(_auth_data: AuthData, conn: DbConn, file_uuid: &RawStr) -> Result<FsFile, String> {
    download_resolver(conn.into(), file_uuid)
        .map_err(|e| e.description().to_string())
}

fn main() {
    dotenv().ok();
    let conf = config::Config::from_env();
    let connection = create_db_pool(&conf);
    println!("Run migrations");
    if let Err(error) = migrations::run(connection.clone().get().unwrap()) {
        println!("Run migrations error {:?}", error);
    }
    let client = reqwest::Client::new().unwrap();
    println!("Run rss_job");
    rss_job::run(client, connection.clone());
    rocket::ignite()
        .manage(Query::new(connection.clone()))
        .manage(create_db_pool(&conf))
        .manage(conf)
        .manage(Schema::new(
            Query::new(connection),
            Mutation,
        ))
        .mount("/", routes![
            index,
            files,
            graphiql,
            post_graphql_handler,
            upload,
            download,
        ])
        .launch();
}
