use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use serde_json::Value;

use feed_rs::entry::Entry;
use services::mercury::ReadableData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rss {
    pub id:          String,
    pub title:       Option<String>,
    pub content:     Option<String>,
    pub summary:     Option<String>,
    pub author:      Option<String>,
    pub published:   String,
    pub updated:     Option<String>,
    pub alternate:   Option<String>,
    pub keywords:    Vec<String>,
    pub enclosure:   Option<String>,
    pub fingerprint: String,
}

impl From<Entry> for Rss {
    fn from(entry: Entry) -> Self {
        Rss {
            id: entry.id,
            title: entry.title,
            content: entry.content,
            summary: entry.summary,
            author: entry.author,
            published: entry.published.to_string(),
            updated: entry.updated.map(|updated| updated.to_string()),
            alternate: entry.alternate.iter().map(|link| link.href.clone()).collect::<Vec<String>>().pop(),
            keywords: entry.keywords,
            enclosure: entry.alternate.iter().map(|link| link.href.clone()).collect::<Vec<String>>().pop(),
            fingerprint: entry.fingerprint,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Feed {
    pub uuid: Uuid,
    pub url: String,
    pub rss: Option<Rss>,
    pub readable: Option<ReadableData>,
    pub twitter: Option<Value>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime
}

impl Feed {
    pub fn new(url: &str, rss: Option<Rss>, readable: Option<ReadableData>, twitter: Option<Value>) -> Self {
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
