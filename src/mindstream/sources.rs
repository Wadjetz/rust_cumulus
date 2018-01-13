#![allow(dead_code)]
use postgres::rows::Row;
use postgres::types::ToSql;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use serde_json::Value;
use serde_json;
use url::Url;

use mindstream::models::sourcetype::SourceType;
use mindstream::models::source_option::SourceOption;
use schema::sources;

use errors::*;
use mindstream::rss::fetch_feeds_channel;
use graphql::query::Query;
use pg::{PgInsertable, PgDatabase};

#[derive(GraphQLObject, Debug, Serialize, Deserialize)]
pub struct RssSource {
    pub title: String,
    pub xml_url: String,
    pub html_url: String,
}

impl RssSource {
    pub fn new(title: &str, xml_url: &str, html_url: &str) -> Self {
        RssSource {
            title: title.to_owned(),
            xml_url: xml_url.to_owned(),
            html_url: html_url.to_owned(),
        }
    }
}

#[derive(GraphQLObject, Debug, Serialize, Deserialize)]
pub struct TwitterSource {
    pub hashtag: Option<String>,
}

impl TwitterSource {
    pub fn new(hashtag: Option<String>) -> Self {
        TwitterSource {
            hashtag
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="sources"]
pub struct Source {
    pub uuid: Uuid,
    pub source_type: SourceType,
    pub data: Value,
    pub error: Option<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

impl Source {
    pub fn options(&self) -> Result<SourceOption> {
        match self.source_type {
            SourceType::Rss => {
                let rss_source = serde_json::from_value::<RssSource>(self.data.clone())?;
                Ok(SourceOption::Rss(rss_source))
            },
            SourceType::Twitter => {
                let twitter_source = serde_json::from_value::<TwitterSource>(self.data.clone())?;
                Ok(SourceOption::Twitter(twitter_source))
            }
        }
    }

    pub fn new_rss(rss_source: RssSource) -> Result<Self> {
        let data = serde_json::to_value(rss_source)?;
        Ok(Source::new(SourceType::Rss, data))
    }

    pub fn new_twitter(twitter_source: TwitterSource) -> Result<Self> {
        let data = serde_json::to_value(twitter_source)?;
        Ok(Source::new(SourceType::Twitter, data))
    }

    fn new(source_type: SourceType, data: Value) -> Self {
        Source {
            uuid: Uuid::new_v4(),
            source_type,
            data,
            error: None,
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
        }
    }
}

graphql_object!(Source: Query as "Source" |&self| {
    description: "Source"

    field uuid() -> Uuid as "uuid" {
        self.uuid
    }

    field source_type() -> &SourceType as "source_type" {
        &self.source_type
    }

    field rss_source() -> Option<RssSource> as "rss_source" {
        match self.source_type {
            SourceType::Rss => serde_json::from_value::<RssSource>(self.data.clone()).ok(),
            _ => None
        }
    }

    field error() -> &Option<String> as "error" {
        &self.error
    }

    field created() -> String as "created" {
        format!("{}", self.created)
    }

    field updated() -> String as "updated" {
        format!("{}", self.updated)
    }
});

impl<'a> From<Row<'a>> for Source {
    fn from(row: Row) -> Self {
        Source {
            uuid: row.get("uuid"),
            source_type: row.get("source_type"),
            data: row.get("data"),
            error: row.get("error"),
            created: row.get("created"),
            updated: row.get("updated"),
        }
    }
}

impl PgInsertable for Source {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO sources (uuid, source_type, data, error, created, updated)
            VALUES ($1, $2, $3, $4, $5, $6);
        "#.to_owned()
    }

    fn insert_params(&self) -> Box<[&ToSql]> {
        Box::new([&self.uuid, &self.source_type, &self.data, &self.error, &self.created, &self.updated])
    }
}

pub fn add_source_resolver(pool: Pool<PostgresConnectionManager>, title: String, xml_url: String, html_url: String) -> Result<Source> {
    let pg = PgDatabase::from_pool(pool)?;
    let rss_source = RssSource::new(&title, &xml_url, &html_url);
    let source = Source::new_rss(rss_source)?;
    if !source_existe(&pg, &xml_url)? {
        pg.insert(&source)?;
        Ok(source)
    } else {
        Err(ErrorKind::AlreadyExist.into())
    }
}

fn source_existe(pg: &PgDatabase, xml_url: &str) -> Result<bool> {
    let exist_query = r#"SELECT COUNT(*) AS exist FROM sources WHERE sources."data" @> $1;"#;
    let json_param = json!({ "xml_url": xml_url });
    Ok(pg.exist(exist_query, &[&json_param])?)
}

pub fn add_rss_source_resolver(pool: Pool<PostgresConnectionManager>, xml_url: &str) -> Result<Source> {
    Url::parse(xml_url)?;
    let pg = PgDatabase::from_pool(pool)?;
    let maybe_feed = fetch_feeds_channel(xml_url)?;
    let feed = maybe_feed.ok_or_else(|| ErrorKind::NotFound)?;
    let source_title = feed.title.unwrap_or_else(|| xml_url.to_string());
    let html_url = feed.website.unwrap_or_else(|| xml_url.to_string());
    let rss_source = RssSource::new(&source_title, xml_url, &html_url);
    let source = Source::new_rss(rss_source)?;
    if !source_existe(&pg, xml_url)? {
        pg.insert(&source)?;
        Ok(source)
    } else {
        Err(ErrorKind::AlreadyExist.into())
    }
}

pub fn find_sources_resolver(pool: Pool<PostgresConnectionManager>, limit: i32, offset: i32) -> Result<Vec<Source>> {
    let pg = PgDatabase::from_pool(pool)?;
    let find_query = r#"SELECT * FROM sources LIMIT $1::int OFFSET $2::int;"#;
    let sources = pg.find(find_query, &[&limit, &offset])?;
    Ok(sources)
}

pub fn find_rss_sources(pg: &PgDatabase, limit: i32, offset: i32) -> Result<Vec<Source>> {
    let find_rss_query = r#"SELECT * FROM sources WHERE source_type = 'Rss' LIMIT $1::int OFFSET $2::int;"#;
    let sources = pg.find::<Source>(find_rss_query, &[&limit, &offset])?;
    Ok(sources)
}

pub fn find_source_by_uuid(pg: &PgDatabase, uuid: &Uuid) -> Result<Option<Source>> {
    let query = "SELECT * FROM sources WHERE uuid = $1::uuid;";
    let source = pg.find_one(query, &[uuid])?;
    Ok(source)
}