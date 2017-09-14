use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;

use postgres::rows::Row;
use postgres::types::ToSql;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use errors::*;
use graphql::query::Query;
use users::{User, find_user_by_uuid};
use dilem::conversations_users::ConversationUser;
use dilem::messages::Message;
use pg::{Insertable, PgDatabase};

#[derive(Debug)]
pub struct Conversation {
    pub uuid: Uuid,
    pub level: i32,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

impl Conversation {
    pub fn new() -> Self {
        Conversation {
            uuid: Uuid::new_v4(),
            level: 0,
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
        }
    }
}

graphql_object!(Conversation: Query as "Conversation" |&self| {
    description: "Conversation"

    field uuid() -> Uuid as "uuid" {
        self.uuid
    }

    field level() -> i32 as "level" {
        self.level
    }

    field created() -> String as "created" {
        format!("{}", self.created)
    }

    field updated() -> String as "updated" {
        format!("{}", self.updated)
    }
});

impl<'a> From<Row<'a>> for Conversation {
    fn from(row: Row) -> Self {
        Conversation {
            uuid: row.get("uuid"),
            level: row.get("level"),
            created: row.get("created"),
            updated: row.get("updated"),
        }
    }
}

impl Insertable for Conversation {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO conversations (uuid, level, created, updated)
            VALUES ($1, $2, $3, $4);
        "#.to_owned()
    }

    fn insert_params(&self) -> Box<[&ToSql]> {
        Box::new([&self.uuid, &self.level, &self.created, &self.updated])
    }
}

pub fn find_conversation(pg: &PgDatabase, conversation_uuid: &Uuid) -> Result<Option<Conversation>> {
    let query = "SELECT * FROM conversations WHERE uuid = $1::uuid;";
    Ok(pg.find_one(query, &[conversation_uuid])?)
}

pub fn create_conversation_resolver(pool: Pool<PostgresConnectionManager>, target_user_uuid: &str, user: &User) -> Result<()> {
    let pg = PgDatabase::from_pool(pool)?;
    let target_user_uuid = Uuid::parse_str(target_user_uuid)?;
    let target_user = find_user_by_uuid(&pg, &target_user_uuid)?;
    let target_user = target_user.ok_or_else(|| ErrorKind::NotFound)?;
    let conversation = Conversation::new();
    pg.insert(&conversation)?;
    pg.insert(&ConversationUser::new(target_user.uuid, conversation.uuid))?; // TODO handle rollback
    pg.insert(&ConversationUser::new(user.uuid, conversation.uuid))?;
    Ok(())
}

pub fn send_message_resolver(pool: Pool<PostgresConnectionManager>, content: &str, conversation_uuid: &str, sender: &User) -> Result<()> {
    let pg = PgDatabase::from_pool(pool)?;
    let conversation_uuid = Uuid::parse_str(conversation_uuid)?;
    let conversation = find_conversation(&pg, &conversation_uuid)?;
    let conversation = conversation.ok_or_else(|| ErrorKind::NotFound)?;
    if is_user_belong_to_conversation(&pg, &conversation, &sender)? {
        let message = Message::new(content, &conversation, sender);
        pg.insert(&message)?;
        Ok(())
    } else {
        Err(ErrorKind::Unauthorized.into())
    }
}

pub fn is_user_belong_to_conversation(pg: &PgDatabase, conversation: &Conversation, user: &User) -> Result<bool> {
    let query = r#"
        SELECT COUNT(*) AS exist
        FROM conversations_users
        WHERE conversations_users.conversation_uuid = $1::uuid
        AND conversations_users.user_uuid = $2::uuid;
    "#;
    Ok(pg.exist(query, &[&conversation.uuid, &user.uuid])?)
}
