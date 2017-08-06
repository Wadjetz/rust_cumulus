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
use files::download_resolver;

