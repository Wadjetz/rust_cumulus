use uuid::Uuid;
use graphql::query::Query;
use juniper::Executor;

use services::rss;
use models::feed_source::FeedSource;
use models::user::User;
use repositories::{users_feeds_sources_repository, feed_source_repository};
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

pub fn fallow_feed_source<'a>(executor: &Executor<'a, Query>, uuid: &str, user: &User) -> Result<FeedSource> {
    let connection = executor.context().connection.clone().get()?;
    let uuid = Uuid::parse_str(uuid)?;
    if let Some(feed_source) = feed_source_repository::find_by_uuid(&connection, &uuid)? {
        users_feeds_sources_repository::follow_feed_source(&connection, &feed_source, user)?;
        Ok(feed_source)
    } else {
        Err(ErrorKind::NotFound.into())
    }
}
