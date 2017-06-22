use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Bookmark {
    pub uuid: Uuid,
    pub url: String,
    pub title: String,
    pub description: String,
    pub path: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub user_uuid: Uuid,
}

impl Bookmark {
    pub fn new(uuid: Uuid, url: String, title: String, description: String, path: String, created: NaiveDateTime, updated: NaiveDateTime, user_uuid: Uuid) -> Self {
        Bookmark {
            uuid,
            url,
            title,
            description,
            path,
            created,
            updated,
            user_uuid
        }
    }

    pub fn from_request(url: String, title: String, description: Option<String>, path: String, user_uuid: Uuid) -> Self {
        Bookmark {
            uuid: Uuid::new_v4(),
            url,
            title,
            description: description.unwrap_or("".to_string()),
            path,
            created: UTC::now().naive_utc(),
            updated: UTC::now().naive_utc(),
            user_uuid
        }
    }
}
