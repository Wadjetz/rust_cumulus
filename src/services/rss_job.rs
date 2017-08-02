use std::thread;
use std::time::Duration;

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use reqwest::Client;

use errors::*;
use feeds::Feed;
use sources;
use pg::PgDatabase;
use services::rss;
use services::mercury;

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

// TODO refactor
fn process_rss(client: &Client, pool: &Pool<PostgresConnectionManager>) -> Result<()> {
    let conn = pool.get()?;
    let pg = PgDatabase::new(conn);
    let sources = sources::find_rss_sources(&pg, i32::max_value(), 0)?;
    for source in &sources {
        if let sources::SourceOption::Rss(rss_source) = source.options()? {
            let maybe_feeds_channel = rss::fetch_feeds_channel(&rss_source.xml_url)?;
            if let Some(feeds_channel) = maybe_feeds_channel {
                for rss_feed in &feeds_channel.entries {
                    for link in &rss_feed.alternate {
                        println!("sources {:?} rss_feed {:?}", sources, rss_feed);
                    }
                }
            }
        }
    }
    /*
    let feeds_sources = repositories::feed_source_repository::find(&conn, i32::max_value(), 0)?;
    for feed_source in &feeds_sources {
        let maybe_feeds_channel = rss::fetch_feeds_channel(&feed_source.xml_url)?;
        if let Some(feeds_channel) = maybe_feeds_channel {
            for rss_feed in &feeds_channel.entries {
                for link in &rss_feed.alternate {
                    if let Ok(None) = repositories::feed_repository::find_by_url(&conn, &link.href) {
                        if let Ok(Some(readable)) = mercury::fetch_readable(client, &link.href) {
                            let feed = Feed::new(&readable.url.clone(), Some(rss_feed.clone().into()), Some(readable), None);
                            match repositories::feed_repository::insert(&conn, &feed) {
                                Ok(_) => {
                                    println!("readable inserted {:?}", feed.url);
                                    repositories::feeds_sources_feeds_repository::insert(&conn, &feed_source, &feed)?;
                                },
                                Err(_error) => {},//println!("readable error {:?}", error),
                            }
                        } else {
                            let feed = Feed::new(&link.href, Some(rss_feed.clone().into()), None, None); // TODO remove clone, refactor
                            match repositories::feed_repository::insert(&conn, &feed) {
                                Ok(_) => {
                                    println!("rss inserted {:?}", feed.url);
                                    repositories::feeds_sources_feeds_repository::insert(&conn, &feed_source, &feed)?;
                                },
                                Err(_error) => {},//println!("rss error {:?}", error),
                            }
                        }
                    }
                }
            }
        } else {
            println!("Feed not found for {:?}", &feed_source.xml_url);
        }
    }
    */
    Ok(())
}
