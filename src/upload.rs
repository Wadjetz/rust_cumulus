use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

use rocket::State;
use rocket::Data;
use uuid::Uuid;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;

use models;
use pg::create_db_pool;
use app_state::AppState;
use token::AuthData;

#[post("/upload/<path..>", data = "<file_data>")]
pub fn upload(_auth_data: AuthData, app_state: State<AppState>, file_data: Data, path: PathBuf) -> Result<String, String> {
    let connection: PooledConnection<PostgresConnectionManager> = app_state.connection.clone().get().expect("Error connection pool"); // TODO handle error
    let maybe_file_name = path.file_name()
                  .and_then(|os_str| os_str.to_str())
                  .map(|s| s.to_string());
    let maybe_parent_path = path.parent();
    match models::file::save_file(file_data, path.clone()) {
        Ok((hash, metadata)) => {
          let file_name = maybe_file_name.unwrap_or(hash.clone());
          let parent = maybe_parent_path
                          .and_then(|path| path.to_str())
                          .map(|s| s.to_string())
                          .unwrap_or("/".to_string());
          let f = models::file::File::new(
              Uuid::new_v4(),
              hash,
              file_name.clone(),
              parent.clone(),
              format!("{}/{}", parent, file_name),
              models::file::FileType::File,
              metadata.size(),
              Uuid::new_v4(),
          );
          //bookmark_repository::insert(&connection, )
          println!("File -> {:?}", f);
          Ok(format!("Ok"))
        },
        Err(err) => Err(err.description().to_string()),
    }
}