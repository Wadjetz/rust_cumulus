use uuid::Uuid;
use postgres::rows::Row;
use postgres_shared::types::ToSql;

use errors::*;
use graphql::query::Query;
use pg::{Insertable, PgDatabase};

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
    pub fn new(uuid: Uuid, hash: Option<String>, name: &str, location: &str, file_type: FileType, size: Option<i64>, user_uuid: Uuid) -> Self {
        File {
            uuid,
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
