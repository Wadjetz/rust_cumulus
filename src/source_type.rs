use diesel::serialize::{self, ToSql, Output, IsNull};
use diesel::deserialize::{self, FromSql};
use std::io::Write;
use diesel::pg::Pg;

#[derive(Debug, PartialEq, ToSql, FromSql, AsExpression, FromSqlRow, GraphQLEnum)]
#[postgres(name = "sourcetype")]
#[sql_type = "SqlSourceType"]
pub enum SourceType {
    Rss,
    Twitter,
}

#[derive(SqlType, PartialEq)]
#[postgres(type_name = "sourcetype")]
pub struct SqlSourceType;

impl ToSql<SqlSourceType, Pg> for SourceType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            SourceType::Rss => out.write_all(b"Rss")?,
            SourceType::Twitter => out.write_all(b"Twitter")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<SqlSourceType, Pg> for SourceType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"Rss" => Ok(SourceType::Rss),
            b"Twitter" => Ok(SourceType::Twitter),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
