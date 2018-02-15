use dotenv::dotenv;
use std::time::Duration;

use migrations;
use config;
use reqwest;
use rocket;
use graphql::query::{Schema, Query};
use graphql::mutation::Mutation;
use pg::create_db_pool;
use mindstream::rss;
use routes;

pub fn run() {
    dotenv().ok();
    let conf = config::Config::from_env();
    let connection = create_db_pool(&conf);
    println!("Run migrations");
    if let Err(error) = migrations::run(connection.clone().get().unwrap()) {
        println!("Run migrations error {:?}", error);
    }
    let client = reqwest::Client::new();
    println!("Run rss_job");
    rss::run_rss_job(Duration::from_secs(&conf.rss_job_interval * 60), client, connection.clone());
    rocket::ignite()
        .manage(Query::new(connection.clone()))
        .manage(create_db_pool(&conf))
        .manage(conf)
        .manage(Schema::new(
            Query::new(connection),
            Mutation,
        ))
        .mount("/", routes![
            routes::index,
            routes::files,
            routes::graphiql,
            routes::post_graphql_handler,
        ])
        .launch();
}
