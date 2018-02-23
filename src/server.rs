use dotenv::dotenv;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use diesel::PgConnection;
use diesel::Connection;

use config::Config;
use reqwest;
use rocket;
use graphql::query::{Schema, Query};
use graphql::mutation::Mutation;
use pg::create_db_pool;
use rss;
use routes;

pub fn create_diesel_pool(config: &Config) -> Pool<ConnectionManager<PgConnection>> {
    let database_url = config.database_url.clone();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool")
}

pub fn establish_connection(config: &Config) -> PgConnection {
    let database_url = config.database_url.clone();
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn run() {
    dotenv().ok();
    let conf = Config::from_env();
    let connection = create_db_pool(&conf);
    let diesel_pool = create_diesel_pool(&conf);
    use embedded_migrations;
    let diesel_connection = establish_connection(&conf);
    embedded_migrations::run(&diesel_connection).expect("Migration Error");

    let client = reqwest::Client::new();
    rss::run_rss_job(conf.rss_job_interval.clone(), client, connection.clone());
    rocket::ignite()
        .manage(Query::new(connection.clone(), diesel_pool.clone()))
        .manage(create_db_pool(&conf))
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
        ])
        .launch();
}
