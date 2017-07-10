use reqwest;
use feed_rs::parser;
use feed_rs::feed::Feed;

use errors::*;

pub fn fetch_feeds_channel(url: &str) -> Result<Option<Feed>> {
    let mut response = reqwest::get(url)?;
    let feed = parser::parse(&mut response);
    Ok(feed)
}
