#![allow(dead_code)]
use postgres::rows::Row;
use postgres_shared::types::ToSql;
use juniper::Executor;
use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use serde_json::Value;
use serde_json;

use errors::*;
use graphql::query::Query;
use pg::{Insertable, Existable, PgDatabase};

#[derive(Debug, EnumString, ToString)]
pub enum SourceType {
    Rss,
    Twitter,
}

#[derive(Debug)]
pub enum SourceOption {
    Rss(RssSource),
    Twitter(TwitterSource)
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug)]
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
            created: UTC::now().naive_utc(),
            updated: UTC::now().naive_utc(),
        }
    }
}

graphql_object!(Source: Query as "Source" |&self| {
    description: "Source"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
    }

    field source_type() -> String as "source_type" {
        self.source_type.to_string()
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

graphql_object!(RssSource: Query as "RssSource" |&self| {
    description: "RssSource"

    field title() -> &String as "title" {
        &self.title
    }

    field xml_url() -> &String as "xml_url" {
        &self.xml_url
    }

    field html_url() -> &String as "html_url" {
        &self.html_url
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

impl Insertable for Source {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO sources (uuid, source_type, data, error, created, updated)
            VALUES ($1, $2, $3, $4, $5, $6);
        "#.to_owned()
    }

    fn insert_params<'a>(&'a self) -> Box<[&'a ToSql]> {
        Box::new([&self.uuid, &self.source_type, &self.data, &self.error, &self.created, &self.updated])
    }
}

impl Existable for Source {
    fn exist_query() -> String {
        r#"SELECT COUNT(*) AS exist FROM sources WHERE sources."data" @> $1;"#.to_owned()
    }
}

pub fn add_source_resolver<'a>(executor: &Executor<'a, Query>, title: String, xml_url: String, html_url: String) -> Result<Source> {
    let connection = executor.context().connection.clone().get()?;
    let pg = PgDatabase::new(connection);
    let rss_source = RssSource::new(&title, &xml_url, &html_url);
    let source = Source::new_rss(rss_source)?;
    let json_param = json!({ "xml_url": xml_url });
    let exist = pg.exist(&Source::exist_query(), json_param)?;
    if !exist {
        pg.insert(&source)?;
        Ok(source)
    } else {
        Err(ErrorKind::AlreadyExist.into())
    }
}

pub fn find_resolver<'a>(executor: &Executor<'a, Query>, limit: i32, offset: i32) -> Result<Vec<Source>> {
    let connection = executor.context().connection.clone().get()?;
    let pg = PgDatabase::new(connection);
    let find_query = r#"SELECT * FROM sources LIMIT $1::int OFFSET $2::int;"#;
    let sources = pg.find(find_query, &[&limit, &offset])?;
    Ok(sources)
}
