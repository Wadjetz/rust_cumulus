use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use postgres::rows::Row;
use postgres::types::ToSql;

use graphql::query::Query;
use users::User;
use dilem::conversations::Conversation;
use pg::Insertable;

#[derive(Debug)]
pub struct Message {
    pub uuid: Uuid,
    pub content: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub conversations_uuid: Uuid,
    pub user_uuid: Uuid,
}

impl Message {
    pub fn new(content: &str, conversations: &Conversation, user: &User) -> Self {
        Message {
            uuid: Uuid::new_v4(),
            content: content.to_owned(),
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
            conversations_uuid: conversations.uuid.clone(),
            user_uuid: user.uuid.clone(),
        }
    }
}

graphql_object!(Message: Query as "Message" |&self| {
    description: "Message"

    field uuid() -> Uuid as "uuid" {
        self.uuid
    }

    field content() -> &String as "content" {
        &self.content
    }

    field created() -> String as "created" {
        format!("{}", self.created)
    }

    field updated() -> String as "updated" {
        format!("{}", self.updated)
    }

    field conversations_uuid() -> Uuid as "conversations_uuid" {
        self.conversations_uuid
    }

    field user_uuid() -> Uuid as "user_uuid" {
        self.user_uuid
    }
});

impl<'a> From<Row<'a>> for Message {
    fn from(row: Row) -> Self {
        Message {
            uuid: row.get("uuid"),
            content: row.get("content"),
            created: row.get("created"),
            updated: row.get("updated"),
            conversations_uuid: row.get("conversations_uuid"),
            user_uuid: row.get("user_uuid"),
        }
    }
}

impl Insertable for Message {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO messages (uuid, content, created, updated, conversations_uuid, user_uuid)
            VALUES ($1, $2, $3, $4, $5, $6);
        "#.to_owned()
    }

    fn insert_params(&self) -> Box<[&ToSql]> {
        Box::new([&self.uuid, &self.content, &self.created, &self.updated, &self.conversations_uuid, &self.user_uuid])
    }
}
