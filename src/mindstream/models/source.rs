use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use serde_json;
use serde_json::Value;

use errors::*;
use graphql::query::Query;
use schema::sources;
use mindstream::models::sourcetype::SourceType;
use mindstream::models::source_option::SourceOption;
use mindstream::models::rss_source::RssSource;
use mindstream::models::twitter_source::TwitterSource;

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
