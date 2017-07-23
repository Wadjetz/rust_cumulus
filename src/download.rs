use std::fs::File as FsFile;
use std::error::Error;
use std::path::Path;

use rocket::State;
use uuid::Uuid;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;

use repositories::file_repository;
use app_state::AppState;
use token::AuthData;

#[get("/download/<uuid>", rank=2)]
pub fn download(_auth_data: AuthData, app_state: State<AppState>, uuid: String) -> Result<FsFile, String> {
    let connection: PooledConnection<PostgresConnectionManager> = app_state.connection.clone().get()
                        .map_err(|e| e.description().to_string())?;
    let uuid = Uuid::parse_str(&uuid)
                        .map_err(|e| e.description().to_string())?;
    let file = file_repository::find_by_uuid(&connection, &uuid)
                        .map_err(|e| e.description().to_string())?;
    FsFile::open(Path::new("upload/").join(Path::new(&file.location)))
                        .map_err(|e| e.description().to_string())
}
