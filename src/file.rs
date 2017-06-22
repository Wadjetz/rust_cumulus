#![allow(dead_code)]
//use errors::*;
//use rocket::Data;
//use std::path::Path;
//use std::path::PathBuf;
//use std::fs::File as FsFile;
//use std::io::prelude::*;
//use uuid::Uuid;

#[derive(Debug)]
pub struct File {
    pub id: String,
    pub name: String,
    pub file_type: FileType,
    pub path: String,
    pub size: u64,
}

#[derive(Debug)]
pub enum FileType {
    File,
    Directory,
}

impl File {
    pub fn new(id: String, name: String, file_type: FileType, path: String, size: u64) -> Self {
        File {
            id, path, name, file_type, size
        }
    }

    pub fn new_directory(id: String, name: String, path: String) -> Self {
        File {
          id,
          path,
          name,
          file_type: FileType::Directory,
          size: 0
        }
    }
}

/*
pub fn save_file(data: Data, path: PathBuf) -> Result<File> {
    let p = Path::new("upload/").join(path);
    let pa = path.clone().to_string();
    let f = FsFile::create(p.clone())?;
    let metadata = f.metadata()?;
    let cfile = File::new(hash_file(&f), path.to_string(), FileType::File, pa, 0);
    data.stream_to_file(p)?;
    Ok(cfile)
}

pub fn hash_file(file: &FsFile) -> Result<String> {
    Ok(Uuid::new_v4().hyphenated().to_string())
}
*/