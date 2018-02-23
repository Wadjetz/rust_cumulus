use uuid::Uuid;
use chrono::prelude::*;
use chrono::NaiveDateTime;

use schema::sources;

#[derive(Debug, PartialEq, Identifiable, Queryable, Insertable, GraphQLObject)]
#[primary_key(uuid)]
#[table_name="sources"]
pub struct Source {
    pub uuid: Uuid,
    pub error: Option<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub url: String,
    pub title: String,
    pub website: String,
}

impl Source {
    pub fn new(url: String, title: String, website: String) -> Self {
        Source {
            uuid: Uuid::new_v4(),
            error: None,
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
            url,
            title,
            website,
        }
    }
}
