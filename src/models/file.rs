#![allow(dead_code)]
use uuid::Uuid;

#[derive(Debug)]
pub struct File {
    pub uuid: Uuid,
    pub hash: String,
    pub name: String,
    pub parent: String,
    pub location: String,
    pub file_type: FileType,
    pub size: i64,
    pub user_uuid: Uuid,
}

#[derive(Debug, EnumString, ToString)]
pub enum FileType {
    File,
    Directory,
}

impl File {
    pub fn new(uuid: Uuid, hash: String, name: String, parent: String, location: String, file_type: FileType, size: i64, user_uuid: Uuid) -> Self {
        File {
            uuid, hash, name, parent, location, file_type, size, user_uuid
        }
    }
}
