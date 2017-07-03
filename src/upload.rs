use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::error::Error;

use rocket::State;
use rocket::Data;
use uuid::Uuid;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;

use models;
use repositories::file_repository;
use app_state::AppState;
use token::AuthData;
use file_system;

#[post("/upload/<path..>", data = "<file_data>")]
pub fn upload(auth_data: AuthData, app_state: State<AppState>, file_data: Data, path: PathBuf) -> Result<String, String> {
    let connection: PooledConnection<PostgresConnectionManager> = app_state.connection.clone().get()
                        .map_err(|e| e.description().to_string())?;
    let maybe_file_name = path.file_name()
                  .and_then(|os_str| os_str.to_str())
                  .map(|s| s.to_string());
    file_system::save_file(file_data, path.clone()).and_then(|(hash, metadata)| {
        let file_name = maybe_file_name.unwrap_or_else(|| hash.clone());
        let f = models::file::File::new(
            Uuid::new_v4(),
            Some(hash),
            &file_name,
            path.to_str().unwrap_or("/"),
            models::file::FileType::File,
            Some(metadata.size() as i64),
            auth_data.uuid,
        );
        file_repository::insert(&connection, &f).map(|inserted| format!("Ok {}", inserted))
    }).map_err(|err| err.description().to_string())
}