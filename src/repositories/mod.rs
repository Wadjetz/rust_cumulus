pub mod bookmark_repository;
pub mod user_repository;
pub mod file_repository;
pub mod feed_source_repository;
pub mod feed_repository;

use models::file::FileType;

use postgres_shared::types::FromSql;
use postgres_shared::types::ToSql;
use postgres::types::Type;
use postgres_protocol::types;
use postgres_shared::types::IsNull;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct NotFileType {
    description: String
}

impl NotFileType {
    pub fn new(description: String) -> Self {
        NotFileType { description }
    }
}

impl fmt::Display for NotFileType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.description())
    }
}

impl Error for NotFileType {
    fn description(&self) -> &str {
        &self.description
    }
}

impl FromSql for FileType {
    fn from_sql(_: &Type, raw: &[u8]) -> Result<FileType, Box<Error + Sync + Send>> {
        let value = types::text_from_sql(raw).map(|b| b.to_owned())?;
        let file_type = FileType::from_str(&value).map_err(|err| {
            Box::new(NotFileType::new(err.description().to_string()))
        })?;
        Ok(file_type)
    }

    fn accepts(ty: &Type) -> bool {
        match *ty {
            Type::Varchar | Type::Text | Type::Bpchar | Type::Name | Type::Unknown => true,
            Type::Other(ref u) if u.name() == "citext" => true,
            _ => false,
        }
    }
}

impl ToSql for FileType {
    fn to_sql(&self, ty: &Type, w: &mut Vec<u8>) -> Result<IsNull, Box<Error + Sync + Send>> {
        <&str as ToSql>::to_sql(&&*self.to_string(), ty, w)
    }

    fn accepts(ty: &Type) -> bool {
        <&str as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}
