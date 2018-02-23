use uuid::Uuid;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde_json::Value;

use errors::*;
use source_type::SourceType;
use source_option::{SourceOption, RssSource, TwitterSource };
use serde_json;

use schema::sources;

#[derive(Debug, PartialEq, Identifiable, Queryable, Insertable)]
#[primary_key(uuid)]
#[table_name="sources"]
pub struct Source {
    pub uuid: Uuid,
    pub source_type: SourceType,
    pub data: Option<Value>,
    pub error: Option<String>,
    pub created: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

impl Source {
    pub fn options(&self) -> Result<SourceOption> {
        match self.source_type {
            SourceType::Rss => {
                // TODO remove unwrap
                let rss_source = serde_json::from_value::<RssSource>(self.data.clone().unwrap())?;
                Ok(SourceOption::Rss(rss_source))
            },
            SourceType::Twitter => {
                // TODO remove unwrap
                let twitter_source = serde_json::from_value::<TwitterSource>(self.data.clone().unwrap())?;
                Ok(SourceOption::Twitter(twitter_source))
            }
        }
    }

    pub fn new_rss(rss_source: RssSource) -> Result<Self> {
        let data = serde_json::to_value(rss_source)?;
        Ok(Source::new(SourceType::Rss, data))
    }

    #[allow(dead_code)]
    pub fn new_twitter(twitter_source: TwitterSource) -> Result<Self> {
        let data = serde_json::to_value(twitter_source)?;
        Ok(Source::new(SourceType::Twitter, data))
    }

    fn new(source_type: SourceType, data: Value) -> Self {
        Source {
            uuid: Uuid::new_v4(),
            source_type,
            data: Some(data),
            error: None,
            created: Some(Utc::now().naive_utc()),
            updated: Some(Utc::now().naive_utc()),
        }
    }
}

graphql_object!(Source: () as "Source" |&self| {
    description: "Source"

    field uuid() -> Uuid as "uuid" {
        self.uuid
    }

    field source_type() -> &SourceType as "source_type" {
        &self.source_type
    }

    field rss_source() -> Option<RssSource> as "rss_source" {
        match self.source_type {
            SourceType::Rss => self.data.clone().and_then(|data| serde_json::from_value::<RssSource>(data.clone()).ok()),
            _ => None
        }
    }

    field error() -> &Option<String> as "error" {
        &self.error
    }

    field created() -> &Option<NaiveDateTime> {
        &self.created
    }

    field updated() -> &Option<NaiveDateTime> {
        &self.updated
    }
});
