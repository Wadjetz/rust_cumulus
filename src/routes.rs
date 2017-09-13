use std::path::{Path, PathBuf};
use std::fs::File as FsFile;

use rocket::response::{NamedFile, content};
use rocket::{Data, State};
use rocket::http::RawStr;
use juniper_rocket;

use graphql::query::{Schema, Query};
use pg::DbConn;
use files::{download_resolver, upload_resolver};
use token::AuthData;

#[get("/graphql")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[post("/graphql", data="<request>")]
pub fn post_graphql_handler(
    context: State<Query>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
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
