use std::path::{Path, PathBuf};
use std::fs::File as FsFile;
use std::error::Error;

use dotenv::dotenv;

use migrations;
use config;
use reqwest;
use rocket;
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

pub fn run() {
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
