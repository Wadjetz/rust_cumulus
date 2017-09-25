use std::str::FromStr;
use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;
use postgres::rows::Row;
use postgres::types::ToSql;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use pg::{Insertable, PgDatabase};
use users::User;
use mindstream::feeds::Feed;
use mindstream::sources::{Source, find_source_by_uuid};
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

fn is_user_feed_exist(pg: &PgDatabase, user_feed: &UserFeed) -> Result<bool> {
    let exist_query = r#"
        SELECT COUNT(*) AS exist FROM users_feeds WHERE user_uuid = $1::uuid AND feed_uuid = $2::uuid;
    "#;
    Ok(pg.exist(exist_query, &[&user_feed.user_uuid, &user_feed.feed_uuid])?)
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

pub fn reaction_feed_resolver(pool: Pool<PostgresConnectionManager>, feed_uuid: &str, reaction: &str, user: &User) -> Result<u64> {
    let pg = PgDatabase::from_pool(pool)?;
    let feed_uuid = Uuid::parse_str(feed_uuid)?;
    let reaction = Reaction::from_str(reaction)?;
    let user_feed = UserFeed::new(user.uuid.clone(), feed_uuid, reaction);
    if !is_user_feed_exist(&pg, &user_feed)? {
        Ok(pg.insert(&user_feed)?)
    } else {
        Err(ErrorKind::AlreadyExist.into())
    }
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
        JOIN users_sources ON users_sources.source_uuid = feeds.source_uuid
        WHERE 0 = (
            SELECT COUNT(*)
            FROM users_feeds
            WHERE users_feeds.feed_uuid = feeds.uuid
                AND users_feeds.user_uuid = $1
        )
        AND users_sources.user_uuid = $1
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
        JOIN users_sources ON users_sources.source_uuid = feeds.source_uuid
        WHERE 0 = (
            SELECT COUNT(*)
            FROM users_feeds
            WHERE users_feeds.feed_uuid = feeds.uuid
                AND users_feeds.user_uuid = $1
        )
        AND users_sources.user_uuid = $1
        AND users_sources.source_uuid = $2
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
        JOIN users_sources ON users_sources.source_uuid = feeds.source_uuid
        WHERE 0 < (
            SELECT COUNT(*)
            FROM users_feeds
            WHERE users_feeds.feed_uuid = feeds.uuid
                AND users_feeds.user_uuid = $1
                AND users_feeds.reaction = $4
        )
        AND users_sources.user_uuid = $1
        LIMIT $2::int OFFSET $3::int;
    "#;
    Ok(pg.find(query, &[&user.uuid, &limit, &offset, &reaction])?)
}
