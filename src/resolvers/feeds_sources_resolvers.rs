use graphql::query::Query;
use juniper::Executor;

use services::rss;
use models::feed_source::FeedSource;
use repositories::feed_source_repository;
use errors::*;

pub fn add_feed_source<'a>(executor: &Executor<'a, Query>, xml_url: &str, title: Option<String>) -> Result<FeedSource> {
    let connection = executor.context().connection.clone().get()?;
    let maybe_feed = rss::fetch_feeds_channel(&xml_url)?;
    let feed = maybe_feed.ok_or_else(|| ErrorKind::NotFound)?;
    let feed_source_title = feed.title.unwrap_or_else(|| xml_url.to_string());
    let feed_source_title = title.unwrap_or_else(|| feed_source_title);
    let html_url = feed.website.unwrap_or_else(|| xml_url.to_string());
    let feed_source = FeedSource::new(&feed_source_title, &xml_url, &html_url);
    Ok(feed_source_repository::insert(&connection, &feed_source).map(|_| feed_source)?)
}
