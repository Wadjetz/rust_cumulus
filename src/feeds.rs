use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use postgres::rows::Row;
use postgres_shared::types::ToSql;
use juniper::Executor;
use serde_json;
use serde_json::Value;
use feed_rs::entry::Entry;

use errors::*;
use graphql::query::Query;
use services::mercury::ReadableData;
use pg::{Insertable, PgDatabase};

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
            created: UTC::now().naive_utc(),
            updated: UTC::now().naive_utc(),
            source_uuid,
        }
    }
}

graphql_object!(Rss: Query as "Rss" |&self| {
    description: "Rss"

    field id() -> &String as "id" {
        &self.id
    }

    field title() -> &Option<String> as "title" {
        &self.title
    }

    field content() -> &Option<String> as "content" {
        &self.content
    }

    field summary() -> &Option<String> as "summary" {
        &self.summary
    }

    field author() -> &Option<String> as "author" {
        &self.author
    }

    field published() -> &String as "published" {
        &self.published
    }

    field updated() -> &Option<String> as "updated" {
        &self.updated
    }

    field url() -> &Option<String> as "url" {
        &self.alternate
    }

    field keywords() -> &Vec<String> as "keywords" {
        &self.keywords
    }

    field enclosure() -> &Option<String> as "enclosure" {
        &self.enclosure
    }

    field fingerprint() -> &String as "fingerprint" {
        &self.fingerprint
    }
});

graphql_object!(ReadableData: Query as "ReadableData" |&self| {
    description: "ReadableData"

    field url() -> &String as "url" {
        &self.url
    }

    field domain() -> &Option<String> as "domain" {
        &self.domain
    }

    field title() -> &Option<String> as "title" {
        &self.title
    }

    field content() -> &Option<String> as "content" {
        &self.content
    }

    field date_published() -> &Option<String> as "date_published" {
        &self.date_published
    }

    field lead_image_url() -> &Option<String> as "lead_image_url" {
        &self.lead_image_url
    }

    field dek() -> &Option<String> as "dek" {
        &self.dek
    }

    field excerpt() -> &Option<String> as "excerpt" {
        &self.excerpt
    }

    field word_count() -> &Option<i32> as "word_count" {
        &self.word_count
    }

    field direction() -> &Option<String> as "direction" {
        &self.direction
    }

    field total_pages() -> &Option<i32> as "total_pages" {
        &self.total_pages
    }

    field rendered_pages() -> &Option<i32> as "rendered_pages" {
        &self.rendered_pages
    }

    field next_page_url() -> &Option<String> as "next_page_url" {
        &self.next_page_url
    }
});

graphql_object!(Feed: Query as "Feed" |&self| {
    description: "Feed"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
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
    /*
    field twitter() -> &String as "twitter" {
        &String::from("twitter")
    }*/

    field created() -> String as "created" {
        format!("{}", self.created)
    }

    field updated() -> String as "updated" {
        format!("{}", self.updated)
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

    fn insert_params<'a>(&'a self) -> Box<[&'a ToSql]> {
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

pub fn exist(pg: &PgDatabase, url: &str) -> Result<bool> {
    let exist_query = "SELECT COUNT(*) AS exist FROM feeds WHERE url = $1;";
    Ok(pg.exist(exist_query, &[&url.to_owned()])?)
}

pub fn find_resolver<'a>(executor: &Executor<'a, Query>, limit: i32, offset: i32) -> Result<Vec<Feed>> {
    let connection = executor.context().connection.clone().get()?;
    let pg = PgDatabase::new(connection);
    let feeds = find_feed(&pg, limit, offset)?;
    Ok(feeds)
}
