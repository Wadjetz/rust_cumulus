use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;

pub struct AppState {
    pub connection: Pool<PostgresConnectionManager>,
}

impl AppState {
    pub fn new(connection: Pool<PostgresConnectionManager>) -> Self {
        AppState { connection }
    }
}
