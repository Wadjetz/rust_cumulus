pub mod bookmark_repository;
pub mod file_repository;

use models::file::FileType;
use sources::SourceType;

use postgres_shared::types::FromSql;
use postgres_shared::types::ToSql;
use postgres::types::Type;
use postgres_protocol::types;
use postgres_shared::types::IsNull;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
struct WrongEnumType {
    description: String
}

impl WrongEnumType {
    pub fn new(description: String) -> Self {
        WrongEnumType { description }
    }
}

impl fmt::Display for WrongEnumType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.description())
    }
}

impl Error for WrongEnumType {
    fn description(&self) -> &str {
        &self.description
    }
}

impl FromSql for FileType {
    fn from_sql(_: &Type, raw: &[u8]) -> Result<FileType, Box<Error + Sync + Send>> {
        let value = types::text_from_sql(raw).map(|b| b.to_owned())?;
        let file_type = FileType::from_str(&value).map_err(|err| {
            Box::new(WrongEnumType::new(err.description().to_string()))
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

impl FromSql for SourceType {
    fn from_sql(_: &Type, raw: &[u8]) -> Result<SourceType, Box<Error + Sync + Send>> {
        let value = types::text_from_sql(raw).map(|b| b.to_owned())?;
        let file_type = SourceType::from_str(&value).map_err(|err| {
            Box::new(WrongEnumType::new(err.description().to_string()))
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

impl ToSql for SourceType {
    fn to_sql(&self, ty: &Type, w: &mut Vec<u8>) -> Result<IsNull, Box<Error + Sync + Send>> {
        <&str as ToSql>::to_sql(&&*self.to_string(), ty, w)
    }

    fn accepts(ty: &Type) -> bool {
        <&str as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}
