use url::Url;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use pg::PgDatabase;

use rss::fetch_feeds_channel;
use sources;
use source_option::RssSource;
use source::Source;
use errors::*;

pub fn add_rss_source_resolver(pool: Pool<PostgresConnectionManager>, xml_url: &str) -> Result<Source> {
    Url::parse(xml_url)?;
    let pg = PgDatabase::from_pool(pool)?;
    let maybe_feed = fetch_feeds_channel(xml_url)?;
    let feed = maybe_feed.ok_or_else(|| ErrorKind::NotFound)?;
    let source_title = feed.title.unwrap_or_else(|| xml_url.to_string());
    let html_url = feed.website.unwrap_or_else(|| xml_url.to_string());
    let rss_source = RssSource::new(&source_title, xml_url, &html_url);
    let source = Source::new_rss(rss_source)?;
    if !sources::source_existe(&pg, xml_url)? {
        pg.insert(&source)?;
        Ok(source)
    } else {
        Err(ErrorKind::AlreadyExist.into())
    }
}
