use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use serde_json::Value;

use feed_rs::entry::Entry;
use services::mercury::ReadableData;

#[derive(Debug)]
pub struct Feed {
    pub uuid: Uuid,
    pub url: String,
    pub rss: Option<Entry>,
    pub readable: Option<ReadableData>,
    pub twitter: Option<Value>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime
}

impl Feed {
    pub fn new(url: &str, rss: Option<Entry>, readable: Option<ReadableData>, twitter: Option<Value>) -> Self {
        Feed {
            uuid: Uuid::new_v4(),
            url: url.to_string(),
            rss,
            readable,
            twitter,
            created: UTC::now().naive_utc(),
            updated: UTC::now().naive_utc(),
        }
    }
}
