use std::str::FromStr;
use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use postgres::rows::Row;
use postgres::types::ToSql;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use pg::{Insertable, PgDatabase};
use user::User;
use feeds::Feed;
use source::Source;
use sources::find_source_by_uuid;
use errors::*;

#[derive(Debug)]
pub struct UserFeed {
    pub uuid: Uuid,
    pub reaction: Reaction,
    pub feed_uuid: Uuid,
    pub user_uuid: Uuid,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Debug, EnumString, ToString, ToSql, FromSql)]
#[postgres(name = "reaction")]
pub enum Reaction {
    Unreaded,
    Readed,
    ReadLater,
    Viewed,
    Liked,
    Disliked,
    Archived,
}

impl UserFeed {
    pub fn new(user_uuid: Uuid, feed_uuid: Uuid, reaction: Reaction) -> Self {
        UserFeed {
            uuid: Uuid::new_v4(),
            reaction: reaction,
            user_uuid,
            feed_uuid,
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
        }
    }
}

impl<'a> From<Row<'a>> for UserFeed {
    fn from(row: Row) -> Self {
        UserFeed {
            uuid: row.get("uuid"),
            reaction: row.get("reaction"),
            user_uuid: row.get("user_uuid"),
            feed_uuid: row.get("feed_uuid"),
            created: row.get("created"),
            updated: row.get("updated"),
        }
    }
}

impl Insertable for UserFeed {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO users_feeds (uuid, reaction, user_uuid, feed_uuid, created, updated) VALUES ($1::uuid, $2, $3::uuid, $4::uuid, $5, $6)
        "#.to_owned()
    }

    fn insert_params(&self) -> Box<[&ToSql]> {
        Box::new([&self.uuid, &self.reaction, &self.user_uuid, &self.feed_uuid, &self.created, &self.updated])
    }
}

pub fn is_user_feed_already_inserted(pg: &PgDatabase, url: &str, user: &User) -> Result<bool> {
    let query = r#"
        SELECT COUNT(*) AS exist FROM feeds
        JOIN users_feeds ON users_feeds.feed_uuid = feeds.uuid
        WHERE users_feeds.user_uuid = $1::uuid
        AND feeds.url = $2
    "#;
    Ok(pg.exist(query, &[&user.uuid, &url])?)
}

pub fn update_feed_reaction(pg: &PgDatabase, feed_uuid: &Uuid, reaction: &Reaction, user: &User) -> Result<u64> {
    let query = r#"
        UPDATE users_feeds SET reaction = $1
        WHERE users_feeds.user_uuid = $2::uuid
        AND users_feeds.feed_uuid = $3::uuid
    "#;
    Ok(pg.update(query, &[reaction, &user.uuid, feed_uuid])?)
}

pub fn reaction_feed_resolver(pool: Pool<PostgresConnectionManager>, feed_uuid: &str, reaction: &str, user: &User) -> Result<u64> {
    let pg = PgDatabase::from_pool(pool)?;
    let feed_uuid = Uuid::parse_str(feed_uuid)?;
    let reaction = Reaction::from_str(reaction)?;
    Ok(update_feed_reaction(&pg, &feed_uuid, &reaction, user)?)
}

pub fn users_feeds_resolver(pool: Pool<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Vec<Feed>> {
    let pg = PgDatabase::from_pool(pool)?;
    let query = r#"
        SELECT feeds.* FROM feeds
        JOIN users_sources ON users_sources.source_uuid = feeds.source_uuid
        WHERE users_sources.user_uuid = $1
        LIMIT $2::int OFFSET $3::int;
    "#;
    Ok(pg.find(query, &[&user.uuid, &limit, &offset])?)
}

pub fn unreaded_feeds(pool: Pool<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Vec<Feed>> {
    let pg = PgDatabase::from_pool(pool)?;
    let query = r#"
        SELECT feeds.* FROM feeds
        JOIN users_feeds ON users_feeds.feed_uuid = feeds.uuid
        WHERE users_feeds.reaction = 'Unreaded'
        AND users_feeds.user_uuid = $1
        ORDER BY feeds.updated DESC
        LIMIT $2::int OFFSET $3::int;
    "#;
    Ok(pg.find(query, &[&user.uuid, &limit, &offset])?)
}

pub fn unreaded_feeds_by_source_resolver(pool: Pool<PostgresConnectionManager>, limit: i32, offset: i32, source_uuid: &str, user: &User) -> Result<Vec<Feed>> {
    let source_uuid = Uuid::parse_str(source_uuid)?;
    let pg = PgDatabase::from_pool(pool)?;
    let source = find_source_by_uuid(&pg, &source_uuid)?;
    let source: Source = source.ok_or(ErrorKind::NotFound)?;
    let query = r#"
        SELECT feeds.* FROM feeds
        JOIN users_feeds ON users_feeds.feed_uuid = feeds.uuid
        WHERE users_feeds.reaction = 'Unreaded'
        AND users_feeds.user_uuid = $1
        AND feeds.source_uuid = $2
        ORDER BY feeds.updated DESC
        LIMIT $3::int OFFSET $4::int;
    "#;
    Ok(pg.find(query, &[&user.uuid, &source.uuid, &limit, &offset])?)
}

pub fn feeds_by_reaction_resolver(pool: Pool<PostgresConnectionManager>, reaction: &str, limit: i32, offset: i32, user: &User) -> Result<Vec<Feed>> {
    let reaction = Reaction::from_str(reaction)?;
    let pg = PgDatabase::from_pool(pool)?;
    let query = r#"
        SELECT feeds.* FROM feeds
        JOIN users_feeds ON users_feeds.feed_uuid = feeds.uuid
        WHERE users_feeds.reaction = $1
        AND users_feeds.user_uuid = $2
        LIMIT $3::int OFFSET $4::int;
    "#;
    Ok(pg.find(query, &[&reaction, &user.uuid, &limit, &offset])?)
}
