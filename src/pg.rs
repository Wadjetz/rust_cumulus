use std::env;
use dotenv::dotenv;
use r2d2::{ Pool, Config };
use r2d2_postgres::{TlsMode, PostgresConnectionManager};

pub fn create_db_pool() -> Pool<PostgresConnectionManager> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = Config::default();
    let manager = PostgresConnectionManager::new(database_url, TlsMode::None).expect("Create PostgresConnectionManager error");
    Pool::new(config, manager).expect("Failed to create pool")
}
