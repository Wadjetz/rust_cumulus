use url::Url;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use diesel::PgConnection;

use rss_service::fetch_feeds_channel;
use source::Source;
use sources_repository;
use errors::*;

pub fn add_rss_source_resolver(pool: &Pool<ConnectionManager<PgConnection>>, url: &str) -> Result<Source> {
    let _ = Url::parse(url)?;
    let connection = pool.get()?;
    let maybe_feed = fetch_feeds_channel(url)?;
    let feed = maybe_feed.ok_or_else(|| ErrorKind::NotFound)?;
    let title = feed.title.unwrap_or_else(|| url.to_string());
    let website = feed.website.unwrap_or_else(|| url.to_string());
    let source = Source::new(url.to_owned(), title, website);
    Ok(sources_repository::insert(&connection, source)?)
}

pub fn find_sources_resolver(pool: &Pool<ConnectionManager<PgConnection>>, limit: i32, offset: i32) -> Result<Vec<Source>> {
    let connection = pool.get()?;
    Ok(sources_repository::find(&connection, limit as i64, offset as i64)?)
}
