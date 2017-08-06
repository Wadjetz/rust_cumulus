use uuid::Uuid;
use postgres::rows::Row;
use postgres_shared::types::ToSql;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;
use std::path::{Path, PathBuf};
use juniper::Executor;
use std::os::unix::fs::MetadataExt;
use rocket::Data;
use std::fs::File as FsFile;

use errors::*;
use token::AuthData;
use graphql::query::Query;
use pg::{Insertable, PgDatabase};
use users::User;
use file_system;

#[derive(Debug)]
pub struct File {
    pub uuid: Uuid,
    pub hash: Option<String>,
    pub name: String,
    pub location: String,
    pub file_type: FileType,
    pub size: Option<i64>,
    pub user_uuid: Uuid,
}

#[derive(Debug, EnumString, ToString)]
pub enum FileType {
    File,
    Directory,
}

impl File {
    pub fn new(hash: Option<String>, name: &str, location: &str, file_type: FileType, size: Option<i64>, user_uuid: Uuid) -> Self {
        File {
            uuid: Uuid::new_v4(),
            hash,
            name: name.to_string(),
            location: location.to_string(),
            file_type,
            size,
            user_uuid,
        }
    }

    pub fn new_directory(name: &str, location: &str, user_uuid: Uuid) -> Self {
        File {
            uuid: Uuid::new_v4(),
            hash: None,
            name: name.to_string(),
            location: location.to_string(),
            file_type: FileType::Directory,
            size: None,
            user_uuid,
        }
    }
}

#[test]
fn path_learning_test() {
    use std::path::Path;
    let path1 = Path::new("/");
    let path2 = Path::new("/toto/test.txt");
    let path3 = Path::new("/toto");

    assert_eq!(path1.file_name(), None);
    assert_eq!(path2.file_name().and_then(|n| n.to_str()), Some("test.txt"));
    assert_eq!(path3.file_name().and_then(|n| n.to_str()), Some("toto"));
}

graphql_object!(File: Query as "File" |&self| {
    description: "A file"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
    }

    field hash() -> &Option<String> as "hash" {
        &self.hash
    }

    field name() -> &String as "name" {
        &self.name
    }

    field location() -> &String as "location" {
        &self.location
    }

    field file_type() -> String as "file_type" {
        self.file_type.to_string()
    }

    field size() -> Option<String> as "size" {
        self.size.map(|s| s.to_string())
    }

    field user_uuid() -> String as "uuid" {
        self.user_uuid.hyphenated().to_string()
    }
});

impl<'a> From<Row<'a>> for File {
    fn from(row: Row) -> Self {
        File {
            uuid: row.get("uuid"),
            hash: row.get("hash"),
            name: row.get("name"),
            location: row.get("location"),
            file_type: row.get("file_type"),
            size: row.get("size"),
            user_uuid: row.get("user_uuid"),
        }
    }
}

impl Insertable for File {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO files (uuid, hash, name, location, file_type, size, user_uuid)
            VALUES ($1, $2, $3, $4, $5, $6, $7);
        "#.to_owned()
    }

    fn insert_params<'a>(&'a self) -> Box<[&'a ToSql]> {
        Box::new([&self.uuid, &self.hash, &self.name, &self.location, &self.file_type, &self.size, &self.user_uuid])
    }
}

pub fn upload_resolver(connection: PooledConnection<PostgresConnectionManager>, file_data: Data, path: PathBuf, auth_data: AuthData) -> Result<String> {
    let maybe_file_name = path.file_name()
                  .and_then(|os_str| os_str.to_str())
                  .map(|s| s.to_string());
    let (hash, metadata) = file_system::save_file(file_data, path.clone())?;
    let file_name = maybe_file_name.unwrap_or_else(|| hash.clone());
    let file = File::new(
        Some(hash),
        &file_name,
        path.to_str().unwrap_or("/"),
        FileType::File,
        Some(metadata.size() as i64),
        auth_data.uuid,
    );
    let pg = PgDatabase::new(connection);
    pg.insert(&file)?;
    Ok(String::from("Ok"))
}

fn find_files_by_uuid(pg: &PgDatabase, file_uuid: Uuid) -> Result<Option<File>> {
    let query = "SELECT * FROM files WHERE uuid = $1";
    Ok(pg.find_one(query, &[&file_uuid])?)
}

pub fn download_resolver(connection: PooledConnection<PostgresConnectionManager>, file_uuid: &str) -> Result<FsFile> {
    let pg = PgDatabase::new(connection);
    let file_uuid = Uuid::parse_str(&file_uuid)?;
    if let Some(file) = find_files_by_uuid(&pg, file_uuid)? {
        let fs_file = FsFile::open(Path::new("upload/").join(Path::new(&file.location)))?;
        Ok(fs_file)
    } else {
        Err(ErrorKind::NotFound.into())
    }
}

pub fn files_resolver<'a>(executor: &Executor<'a, Query>, limit: i32, offset: i32, user: &User) -> Result<Vec<File>> {
    let connection = executor.context().connection.clone().get()?;
    let pg = PgDatabase::new(connection);
    let query = "SELECT * FROM files WHERE user_uuid = $1::uuid LIMIT $2::int OFFSET $3::int;";
    Ok(pg.find(query, &[&user.uuid, &limit, &offset])?)
}
