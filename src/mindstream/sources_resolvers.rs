use url::Url;

use errors::*;
use mindstream::models::source::Source;
use mindstream::models::rss_source::RssSource;
use mindstream::sources::insert;
use mindstream::rss::fetch_feeds_channel;
use pg::PgDatabase;
use diesel::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use r2d2_postgres::PostgresConnectionManager;

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

fn source_existe(pg: &PgDatabase, xml_url: &str) -> Result<bool> {
    let exist_query = r#"SELECT COUNT(*) AS exist FROM sources WHERE sources."data" @> $1;"#;
    let json_param = json!({ "xml_url": xml_url });
    Ok(pg.exist(exist_query, &[&json_param])?)
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

