use dotenv::dotenv;
use std::time::Duration;

use config;
use reqwest;
use rocket;
use graphql::query::{Schema, Query};
use graphql::mutation::Mutation;
use pg::{create_db_pool, create_diesel_pool};
use mindstream::rss;
use routes;

pub fn run() {
    dotenv().ok();
    let conf = config::Config::from_env();
    let connection = create_db_pool(&conf);
    let client = reqwest::Client::new();
    println!("Run rss_job");
    rss::run_rss_job(Duration::from_secs(&conf.rss_job_interval * 60), client, connection.clone());

    let diesel_pool = create_diesel_pool(&conf);

    rocket::ignite()
        .manage(Query::new(connection.clone(), diesel_pool.clone()))
        .manage(create_db_pool(&conf))
        .manage(diesel_pool.clone())
        .manage(conf)
        .manage(Schema::new(
            Query::new(connection, diesel_pool),
            Mutation,
        ))
        .mount("/", routes![
            routes::index,
            routes::files,
            routes::graphiql,
            routes::post_graphql_handler,
            routes::upload,
            routes::download,
        ])
        .launch();
}
