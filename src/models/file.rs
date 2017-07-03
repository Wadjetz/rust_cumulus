#![allow(dead_code)]
use uuid::Uuid;

#[derive(Debug)]
pub struct File {
    pub uuid: Uuid,
    pub hash: Option<String>,
    pub name: String,
    pub parent: String,
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
    pub fn new(uuid: Uuid, hash: Option<String>, name: &str, parent: &str, location: &str, file_type: FileType, size: Option<i64>, user_uuid: Uuid) -> Self {
        File {
            uuid,
            hash,
            name: name.to_string(),
            parent: parent.to_string(),
            location: location.to_string(),
            file_type,
            size,
            user_uuid,
        }
    }

    pub fn new_directory(name: &str, parent: &str, location: &str, user_uuid: Uuid) -> Self {
        File {
            uuid: Uuid::new_v4(),
            hash: None,
            name: name.to_string(),
            parent: parent.to_string(),
            location: location.to_string(),
            file_type: FileType::Directory,
            size: None,
            user_uuid,
        }
    }
}
