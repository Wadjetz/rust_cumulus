use postgres::rows::Row;
use r2d2_postgres::PostgresConnectionManager;
use uuid::Uuid;
use url::Url;

use mindstream::models::rss_source::RssSource;
use mindstream::models::source::Source;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use schema::sources;

use errors::*;
use mindstream::rss::fetch_feeds_channel;
use pg::PgDatabase;

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

pub fn insert(connection: &PgConnection, source: &Source) -> Result<Source> {
    Ok(diesel::insert_into(sources::table)
            .values(source)
            .get_result(&*connection)?)
}

pub fn _add_source_resolver(pool: Pool<PostgresConnectionManager>, diesel_pool: &Pool<ConnectionManager<PgConnection>>, title: String, xml_url: String, html_url: String) -> Result<Source> {
    let pg = PgDatabase::from_pool(pool)?;
    let connection = diesel_pool.get()?;
    let rss_source = RssSource::new(&title, &xml_url, &html_url);
    let source = Source::new_rss(rss_source)?;
    if !source_existe(&pg, &xml_url)? {
        let source = insert(&connection, &source)?;
        Ok(source)
    } else {
        Err(ErrorKind::AlreadyExist.into())
    }
}

fn source_existe(pg: &PgDatabase, xml_url: &str) -> Result<bool> {
    let exist_query = r#"SELECT COUNT(*) AS exist FROM sources WHERE sources."data" @> $1;"#;
    let json_param = json!({ "xml_url": xml_url });
    Ok(pg.exist(exist_query, &[&json_param])?)
}

pub fn add_rss_source_resolver(pool: Pool<PostgresConnectionManager>, diesel_pool: &Pool<ConnectionManager<PgConnection>>, xml_url: &str) -> Result<Source> {
    Url::parse(xml_url)?;
    let connection = diesel_pool.get()?;
    let pg = PgDatabase::from_pool(pool)?;
    let maybe_feed = fetch_feeds_channel(xml_url)?;
    let feed = maybe_feed.ok_or_else(|| ErrorKind::NotFound)?;
    let source_title = feed.title.unwrap_or_else(|| xml_url.to_string());
    let html_url = feed.website.unwrap_or_else(|| xml_url.to_string());
    let rss_source = RssSource::new(&source_title, xml_url, &html_url);
    let source = Source::new_rss(rss_source)?;
    if !source_existe(&pg, xml_url)? {
        let source = insert(&connection, &source)?;
        Ok(source)
    } else {
        Err(ErrorKind::AlreadyExist.into())
    }
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