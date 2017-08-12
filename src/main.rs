#![allow(unused_doc_comment)]
#![recursion_limit = "1024"]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]
#![feature(custom_derive)]
#![feature(associated_type_defaults)]

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
#[macro_use]
extern crate juniper;

mod errors;
mod pg;
mod migration;
mod migrations;
mod file_system;
mod token;
mod graphql;
mod app_state;
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

use rocket::response::{NamedFile, content};
use rocket::{Data, State};
use juniper::RootNode;
use juniper::rocket_handlers;

use graphql::query::Query;
use graphql::mutation::Mutation;
use app_state::AppState;
use pg::create_db_pool;
use services::rss_job;
use files::{download_resolver, upload_resolver};
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
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
pub fn upload(auth_data: AuthData, app_state: State<AppState>, file_data: Data, path: PathBuf) -> Result<String, String> {
    let connection: PooledConnection<PostgresConnectionManager> = app_state.connection.clone().get()
                        .map_err(|e| e.description().to_string())?;
    upload_resolver(connection, file_data, path, auth_data)
        .map_err(|err| err.description().to_string())
}

#[get("/download/<file_uuid>")]
pub fn download(_auth_data: AuthData, app_state: State<AppState>, file_uuid: String) -> Result<FsFile, String> {
    let connection: PooledConnection<PostgresConnectionManager> = app_state.connection.clone().get()
        .map_err(|e| e.description().to_string())?;
    download_resolver(connection, &file_uuid)
        .map_err(|e| e.description().to_string())
}

fn main() {
    let connection = create_db_pool(&config::CONFIG);
    println!("Run migrations");
    if let Err(error) = migrations::run(connection.clone().get().unwrap()) {
        println!("Run migrations error {:?}", error);
    }
    let client = reqwest::Client::new().unwrap();
    println!("Run rss_job");
    rss_job::run(client, connection.clone());
    rocket::ignite()
        .manage(Query::new(connection.clone()))
        .manage(AppState::new(connection.clone()))
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
