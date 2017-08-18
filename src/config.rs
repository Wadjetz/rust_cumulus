use std::env;

#[derive(Debug)]
pub struct Config {
    pub secret_key: String,
    pub database_url: String,
    pub mercury_api_key: String,
}

impl Config {
    pub fn new(secret_key: String, database_url: String, mercury_api_key: String) -> Self {
        Config { secret_key, database_url, mercury_api_key }
    }
    pub fn from_env() -> Self {
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mercury_api_key = env::var("MERCURY_API_KEY").expect("MERCURY_API_KEY must be set");
        Config::new(secret_key, database_url, mercury_api_key)
    }
}

lazy_static! {
    pub static ref CONFIG: Config = {
        Config::from_env()
    };
}
