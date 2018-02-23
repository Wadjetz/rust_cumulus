use postgres::rows::Row;
use postgres::types::ToSql;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use uuid::Uuid;

use errors::*;
use pg::{Insertable, PgDatabase};
use source_option::RssSource;
use source::Source;

impl<'a> From<Row<'a>> for Source {
    fn from(row: Row) -> Self {
        Source {
            uuid: row.get("uuid"),
            source_type: row.get("source_type"),
            data: row.get("data"),
            error: row.get("error"),
            created: row.get("created"),
            updated: row.get("updated"),
        }
    }
}

impl Insertable for Source {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO sources (uuid, source_type, data, error, created, updated)
            VALUES ($1, $2, $3, $4, $5, $6);
        "#.to_owned()
    }

    fn insert_params(&self) -> Box<[&ToSql]> {
        Box::new([&self.uuid, &self.source_type, &self.data, &self.error, &self.created, &self.updated])
    }
}

#[allow(dead_code)]
pub fn add_source_resolver(pool: Pool<PostgresConnectionManager>, title: String, xml_url: String, html_url: String) -> Result<Source> {
    let pg = PgDatabase::from_pool(pool)?;
    let rss_source = RssSource::new(&title, &xml_url, &html_url);
    let source = Source::new_rss(rss_source)?;
    if !source_existe(&pg, &xml_url)? {
        pg.insert(&source)?;
        Ok(source)
    } else {
        Err(ErrorKind::AlreadyExist.into())
    }
}

pub fn source_existe(pg: &PgDatabase, xml_url: &str) -> Result<bool> {
    let exist_query = r#"SELECT COUNT(*) AS exist FROM sources WHERE sources."data" @> $1;"#;
    let json_param = json!({ "xml_url": xml_url });
    Ok(pg.exist(exist_query, &[&json_param])?)
}

pub fn find_sources_resolver(pool: Pool<PostgresConnectionManager>, limit: i32, offset: i32) -> Result<Vec<Source>> {
    let pg = PgDatabase::from_pool(pool)?;
    let find_query = r#"SELECT * FROM sources LIMIT $1::int OFFSET $2::int;"#;
    let sources = pg.find(find_query, &[&limit, &offset])?;
    Ok(sources)
}

pub fn find_rss_sources(pg: &PgDatabase, limit: i32, offset: i32) -> Result<Vec<Source>> {
    let find_rss_query = r#"SELECT * FROM sources WHERE source_type = 'Rss' LIMIT $1::int OFFSET $2::int;"#;
    let sources = pg.find::<Source>(find_rss_query, &[&limit, &offset])?;
    Ok(sources)
}

pub fn find_source_by_uuid(pg: &PgDatabase, uuid: &Uuid) -> Result<Option<Source>> {
    let query = "SELECT * FROM sources WHERE uuid = $1::uuid;";
    let source = pg.find_one(query, &[uuid])?;
    Ok(source)
}