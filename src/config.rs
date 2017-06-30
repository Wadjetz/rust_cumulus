use std::env;
use dotenv::dotenv;

pub struct Config {
    pub secret_key: String,
    pub database_url: String,
}

impl Config {
    pub fn new(secret_key: String, database_url: String) -> Self {
        Config { secret_key, database_url }
    }
    pub fn from_env() -> Self {
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Config::new(secret_key, database_url)
    }
}

lazy_static! {
    pub static ref CONFIG: Config = {
        dotenv().ok();
        Config::from_env()
    };
}
