use std::path::{Path, PathBuf};

use rocket::response::{NamedFile, content};
use rocket::State;
use juniper_rocket;

use graphql::query::{Schema, Query};

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
