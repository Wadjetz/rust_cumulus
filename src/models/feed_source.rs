use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;

#[derive(Debug)]
pub struct FeedSource {
    pub uuid: Uuid,
    pub title: String,
    pub xml_url: String,
    pub html_url: String,
    pub error: Option<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

impl FeedSource {
    pub fn new(title: &str, xml_url: &str, html_url: &str) -> Self {
        FeedSource {
            uuid: Uuid::new_v4(),
            title: title.to_string(),
            xml_url: xml_url.to_string(),
            html_url: html_url.to_string(),
            error: None,
            created: UTC::now().naive_utc(),
            updated: UTC::now().naive_utc(),
        }
    }
}
