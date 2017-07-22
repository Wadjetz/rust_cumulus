use std::thread;
use std::time::Duration;

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use errors::*;

use repositories;
use services::rss;

pub fn run(pool: Pool<PostgresConnectionManager>) {
    thread::spawn(move || {
        loop {
            if let Err(err) = process_rss(&pool) {
                println!("process_rss error {:?}", err);
            }
            let duration = Duration::from_secs(60);
            thread::sleep(duration);
        }
    });
}

fn process_rss(pool: &Pool<PostgresConnectionManager>) -> Result<()> {
    let conn = pool.get()?;
    let feeds_sources = repositories::feed_source_repository::find(&conn, i32::max_value(), 0)?;
    for feed_source in feeds_sources {
        let maybe_feeds_channel = rss::fetch_feeds_channel(&feed_source.xml_url)?;
        if let Some(feeds_channel) = maybe_feeds_channel {
            for feed in feeds_channel.entries {
                println!("feed {:?}", feed);
            }
        } else {
            println!("Feed not found for {:?}", &feed_source.xml_url);
        }
    }
    Ok(())
}
