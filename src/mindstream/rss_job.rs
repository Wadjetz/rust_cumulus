use std::thread;
use std::time::Duration;

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use reqwest;
use reqwest::Client;
use feed_rs::parser;
use feed_rs::feed::Feed as RssFeed;

use errors::*;
use mindstream::feeds::{is_feed_exist, insert_feed, Feed};
use mindstream::sources::{find_rss_sources, SourceOption};
use mindstream::mercury::{fetch_readable};
use pg::PgDatabase;

pub fn fetch_feeds_channel(url: &str) -> Result<Option<RssFeed>> {
    let mut response = reqwest::get(url)?;
    let feed = parser::parse(&mut response);
    Ok(feed)
}

pub fn run(client: Client, pool: Pool<PostgresConnectionManager>) {
    thread::spawn(move || {
        loop {
            if let Err(err) = process_rss(&client, &pool) {
                println!("process_rss error {:?}", err);
            }
            let duration = Duration::from_secs(5 * 60);
            thread::sleep(duration);
        }
    });
}

fn process_rss(client: &Client, pool: &Pool<PostgresConnectionManager>) -> Result<()> {
    let conn = pool.get()?;
    let pg = PgDatabase::new(conn);
    let sources = find_rss_sources(&pg, i32::max_value(), 0)?;
    for source in &sources {
        match source.options()? {
            SourceOption::Rss(rss_source) => {
                let maybe_feeds_channel = fetch_feeds_channel(&rss_source.xml_url)?;
                if let Some(feeds_channel) = maybe_feeds_channel {
                    for rss_feed in &feeds_channel.entries {
                        for link in &rss_feed.alternate {
                            if !is_feed_exist(&pg, &link.href, source)? {
                                if let Ok(Some(readable)) = fetch_readable(client, &link.href) {
                                    let feed = Feed::new(&link.href, Some(rss_feed.clone().into()), Some(readable), None, source.uuid);
                                    if insert_feed(&pg, &feed).is_ok() {
                                        println!("readable inserted {:?} from {:?}", feed.url, &rss_source.xml_url);
                                    }
                                } else {
                                    let feed = Feed::new(&link.href, Some(rss_feed.clone().into()), None, None, source.uuid); // TODO remove clone, refactor
                                    if insert_feed(&pg, &feed).is_ok() {
                                        println!("rss inserted {:?} from {:?}", feed.url, &rss_source.xml_url);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            SourceOption::Twitter(_) => {}
        }
    }
    Ok(())
}
