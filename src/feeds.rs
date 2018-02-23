use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use postgres::rows::Row;
use postgres::types::ToSql;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde_json;
use serde_json::Value;
use feed_rs::entry::Entry;

use errors::*;
use graphql::query::Query;
use source::Source;
use mercury::ReadableData;
use pg::{Insertable, PgDatabase};

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
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
    pub rss: Option<Value>,
    pub readable: Option<Value>,
    pub twitter: Option<Value>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub source_uuid: Uuid,
}

impl Feed {
    pub fn new(url: &str, rss: Option<Rss>, readable: Option<ReadableData>, twitter: Option<Value>, source_uuid: Uuid) -> Self {
        Feed {
            uuid: Uuid::new_v4(),
            url: url.to_string(),
            rss: serde_json::to_value(rss).ok(),
            readable: serde_json::to_value(readable).ok(),
            twitter,
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
            source_uuid,
        }
    }
}

graphql_object!(Feed: Query as "Feed" |&self| {
    description: "Feed"

    field uuid() -> Uuid as "uuid" {
        self.uuid
    }

    field url() -> &String as "url" {
        &self.url
    }

    field rss() -> Option<Rss> as "rss" {
        self.rss.clone().and_then(|r| serde_json::from_value::<Rss>(r).ok())
    }

    field readable() -> Option<ReadableData> as "readable" {
        self.readable.clone().and_then(|r| serde_json::from_value::<ReadableData>(r).ok())
        
    }
    
    field twitter() -> Option<String> as "twitter" {
        None
    }

    field created() -> String as "created" {
        format!("{}", self.created)
    }

    field updated() -> String as "updated" {
        format!("{}", self.updated)
    }

    field source_uuid() -> String as "source_uuid" {
        self.source_uuid.hyphenated().to_string()
    }
});

impl<'a> From<Row<'a>> for Feed {
    fn from(row: Row) -> Self {
        Feed {
            uuid: row.get("uuid"),
            url: row.get("url"),
            rss: row.get("rss"),
            readable: row.get("readable"),
            twitter: row.get("twitter"),
            created: row.get("created"),
            updated: row.get("updated"),
            source_uuid: row.get("source_uuid"),
        }
    }
}

impl Insertable for Feed {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO feeds (uuid, url, rss, readable, twitter, created, updated, source_uuid)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8);
        "#.to_owned()
    }

    fn insert_params(&self) -> Box<[&ToSql]> {
        Box::new([
            &self.uuid,
            &self.url,
            &self.rss,
            &self.readable,
            &self.twitter,
            &self.created,
            &self.updated,
            &self.source_uuid,
        ])
    }
}

pub fn insert_feed(pg: &PgDatabase, feed: &Feed) -> Result<u64> {
    Ok(pg.insert(feed)?)
}

pub fn find_feed(pg: &PgDatabase, limit: i32, offset: i32) -> Result<Vec<Feed>> {
    let find_query = r#"SELECT * FROM feeds ORDER BY created DESC LIMIT $1::int OFFSET $2::int;"#;
    pg.find(find_query, &[&limit, &offset])
}

pub fn is_feed_exist(pg: &PgDatabase, url: &str, source: &Source) -> Result<bool> {
    let exist_query = "SELECT COUNT(*) AS exist FROM feeds WHERE url = $1 AND source_uuid = $2;";
    Ok(pg.exist(exist_query, &[&url.to_owned(), &source.uuid])?)
}

pub fn find_resolver(pool: Pool<PostgresConnectionManager>, limit: i32, offset: i32) -> Result<Vec<Feed>> {
    let pg = PgDatabase::from_pool(pool)?;
    let feeds = find_feed(&pg, limit, offset)?;
    Ok(feeds)
}
