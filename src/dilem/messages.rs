use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use postgres::rows::Row;
use postgres::types::ToSql;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use errors::*;
use graphql::query::Query;
use users::User;
use dilem::conversations::{Conversation, find_conversation, is_user_belong_to_conversation};
use pg::{Insertable, PgDatabase};

#[derive(Debug)]
pub struct Message {
    pub uuid: Uuid,
    pub content: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub conversation_uuid: Uuid,
    pub user_uuid: Uuid,
}

impl Message {
    pub fn new(content: &str, conversation: &Conversation, user: &User) -> Self {
        Message {
            uuid: Uuid::new_v4(),
            content: content.to_owned(),
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
            conversation_uuid: conversation.uuid.clone(),
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

    field conversation_uuid() -> Uuid as "Conversation uuid" {
        self.conversation_uuid
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
            conversation_uuid: row.get("conversation_uuid"),
            user_uuid: row.get("user_uuid"),
        }
    }
}

impl Insertable for Message {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO messages (uuid, content, created, updated, conversation_uuid, user_uuid)
            VALUES ($1, $2, $3, $4, $5, $6);
        "#.to_owned()
    }

    fn insert_params(&self) -> Box<[&ToSql]> {
        Box::new([&self.uuid, &self.content, &self.created, &self.updated, &self.conversation_uuid, &self.user_uuid])
    }
}

pub fn send_message_resolver(pool: Pool<PostgresConnectionManager>, content: &str, conversation_uuid: &str, sender: &User) -> Result<Message> {
    let pg = PgDatabase::from_pool(pool)?;
    let conversation_uuid = Uuid::parse_str(conversation_uuid)?;
    let conversation = find_conversation(&pg, &conversation_uuid)?;
    let conversation = conversation.ok_or_else(|| ErrorKind::NotFound)?;
    if is_user_belong_to_conversation(&pg, &conversation, &sender)? {
        let message = Message::new(content, &conversation, sender);
        pg.insert(&message)?;
        Ok(message)
    } else {
        Err(ErrorKind::Unauthorized.into())
    }
}

fn find_messages(pg: &PgDatabase, limit: i32, offset: i32, conversation_uuid: &Uuid) -> Result<Vec<Message>> {
    let query = r#"
        SELECT messages.* FROM messages
        WHERE messages.conversation_uuid = $1::uuid
        LIMIT $2::int OFFSET $3::int;
    "#;
    Ok(pg.find(query, &[&conversation_uuid, &limit, &offset])?)
}

pub fn find_messages_resolver(pool: Pool<PostgresConnectionManager>, limit: i32, offset: i32, conversation_uuid: &str, user: &User) -> Result<Vec<Message>> {
    let pg = PgDatabase::from_pool(pool)?;
    let conversation_uuid = Uuid::parse_str(conversation_uuid)?;
    let conversation = find_conversation(&pg, &conversation_uuid)?;
    let conversation = conversation.ok_or_else(|| ErrorKind::NotFound)?;
    if is_user_belong_to_conversation(&pg, &conversation, &user)? {
        Ok(find_messages(&pg, limit, offset, &conversation_uuid)?)
    } else {
        Err(ErrorKind::Unauthorized.into())
    }
}
