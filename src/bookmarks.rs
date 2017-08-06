use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use postgres::rows::Row;
use juniper::Executor;

use errors::*;
use graphql::query::Query;
use users::User;
use repositories::bookmark_repository;

#[derive(Debug)]
pub struct Bookmark {
    pub uuid: Uuid,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub path: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub user_uuid: Uuid,
}
impl Bookmark {
    pub fn new(url: String, title: String, description: Option<String>, path: String, user_uuid: Uuid) -> Self {
        Bookmark {
            uuid: Uuid::new_v4(),
            url,
            title,
            description,
            path,
            created: UTC::now().naive_utc(),
            updated: UTC::now().naive_utc(),
            user_uuid
        }
    }
}

graphql_object!(Bookmark: Query as "Bookmark" |&self| {
    description: "Bookmark"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
    }

    field url() -> &String as "url" {
        &self.url
    }

    field title() -> &String as "title" {
        &self.title
    }

    field description() -> &Option<String> as "description" {
        &self.description
    }

    field path() -> &String as "path" {
        &self.path
    }

    field created() -> String as "created" {
        format!("{}", self.created)
    }

    field updated() -> String as "updated" {
        format!("{}", self.updated)
    }

    field user_uuid() -> String as "user_uuid" {
        self.user_uuid.hyphenated().to_string()
    }
});

impl<'a> From<Row<'a>> for Bookmark {
    fn from(row: Row) -> Self {
        Bookmark {
            uuid: row.get("uuid"),
            url: row.get("url"),
            title: row.get("title"),
            description: row.get("description"),
            path: row.get("path"),
            created: row.get("created"),
            updated: row.get("updated"),
            user_uuid: row.get("user_uuid"),
        }
    }
}

pub fn add_bookmark_resolver<'a>(executor: &Executor<'a, Query>, bookmark: Bookmark, user: &User) -> Result<Bookmark> {
    let connection = executor.context().connection.clone().get()?;
    let maybe_bookmark = bookmark_repository::find_by_url_and_user(&connection, &bookmark.url, user)?;
    maybe_bookmark.ok_or_else(|| ErrorKind::AlreadyExist)?;
    bookmark_repository::insert(&connection, &bookmark)?;
    Ok(bookmark)
}
