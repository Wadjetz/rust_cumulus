#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::response::content;
use rocket::State;

extern crate juniper;
use juniper::{EmptyMutation, RootNode};
use juniper::rocket_handlers;
use juniper::tests::model::Database;

type Schema = RootNode<'static, Database, EmptyMutation<Database>>;

#[get("/")]
fn graphiql() -> content::HTML<String> {
    rocket_handlers::graphiql_source("/graphql")
}

#[post("/graphql", data="<request>")]
fn post_graphql_handler(
    context: State<Database>,
    request: rocket_handlers::GraphQLRequest,
    schema: State<Schema>
) -> rocket_handlers::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(Database::new())
        .manage(Schema::new(Database::new(), EmptyMutation::<Database>::new()))
        .mount("/", routes![graphiql, post_graphql_handler])
        .launch();
}
