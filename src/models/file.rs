#![allow(dead_code)]
use uuid::Uuid;

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
