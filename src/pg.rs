use r2d2::{ Pool, Config };
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use config::Config as AppConfig;

pub fn create_db_pool(app_config: &AppConfig) -> Pool<PostgresConnectionManager> {
    let database_url = app_config.database_url.clone();
    let config = Config::default();
    let manager = PostgresConnectionManager::new(database_url, TlsMode::None).expect("Create PostgresConnectionManager error");
    Pool::new(config, manager).expect("Failed to create pool")
}
