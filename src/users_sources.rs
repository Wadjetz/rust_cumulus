use uuid::Uuid;
use postgres::rows::Row;
use postgres::types::ToSql;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use errors::*;
use graphql::query::Query;
use user::User;
use source::Source;
use users_feeds::Reaction;
use pg::{Insertable, PgDatabase};

#[derive(Debug)]
pub struct UserSource {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub source_uuid: Uuid,
}

impl UserSource {
    #[allow(dead_code)]
    pub fn new(user_uuid: Uuid, source_uuid: Uuid) -> Self {
        UserSource {
            uuid: Uuid::new_v4(),
            user_uuid,
            source_uuid,
        }
    }
}

impl<'a> From<Row<'a>> for UserSource {
    fn from(row: Row) -> Self {
        UserSource {
            uuid: row.get("uuid"),
            user_uuid: row.get("user_uuid"),
            source_uuid: row.get("source_uuid"),
        }
    }
}

impl Insertable for UserSource {
    fn insert_query(&self) -> String {
        r#"
            INSERT INTO users_sources (uuid, user_uuid, source_uuid) VALUES ($1::uuid, $2::uuid, $3::uuid)
        "#.to_owned()
    }

    fn insert_params(&self) -> Box<[&ToSql]> {
        Box::new([&self.uuid, &self.user_uuid, &self.source_uuid])
    }
}

pub fn find_user_source_by_uuid(pg: &PgDatabase, uuid: Uuid) -> Result<Option<Source>> {
    let find_query = r#"SELECT * FROM sources WHERE uuid = $1::uuid;"#;
    Ok(pg.find_one::<Source>(find_query, &[&uuid])?)
}

pub fn user_source_exist(pg: &PgDatabase, uuid: &Uuid, user: &User) -> Result<bool> {
    let exist_query = r#"
        SELECT COUNT(*) AS exist FROM users_sources WHERE user_uuid = $1::uuid AND source_uuid = $2::uuid;
    "#;
    Ok(pg.exist(exist_query, &[&user.uuid, &uuid])?)
}

pub fn fallow_source_resolver(pool: Pool<PostgresConnectionManager>, uuid: &str, user: &User) -> Result<Source> {
    let pg = PgDatabase::from_pool(pool)?;
    let uuid = Uuid::parse_str(uuid)?;
    let maybe_source = find_user_source_by_uuid(&pg, uuid)?;
    if let Some(source) = maybe_source {
        let exist = user_source_exist(&pg, &uuid, user)?;
        if !exist {
            let user_source = UserSource::new(user.uuid.clone(), source.uuid.clone());
            pg.insert(&user_source)?;
            Ok(source)
        } else {
            Err(ErrorKind::AlreadyExist.into())
        }
    } else {
        Err(ErrorKind::NotFound.into())
    }
}

pub fn users_sources_resolver(pool: Pool<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Vec<Source>> {
    let pg = PgDatabase::from_pool(pool)?;
    let query = r#"
        SELECT sources.* FROM sources
        JOIN users_sources ON users_sources.source_uuid = sources.uuid
        WHERE users_sources.user_uuid = $1
        LIMIT $2::int OFFSET $3::int;
    "#;
    Ok(pg.find(query, &[&user.uuid, &limit, &offset])?)
}

pub fn unfollowed_sources_resolver(pool: Pool<PostgresConnectionManager>, limit: i32, offset: i32, user: &User) -> Result<Vec<Source>> {
    let pg = PgDatabase::from_pool(pool)?;
    let query = r#"
        SELECT sources.* FROM sources
        WHERE 0 = (
            SELECT COUNT(*)
            FROM users_sources
            WHERE sources.uuid = users_sources.source_uuid
                AND users_sources.user_uuid = $1
        )
        LIMIT $2::int OFFSET $3::int;
    "#;
    Ok(pg.find(query, &[&user.uuid, &limit, &offset])?)
}

pub fn total_my_rss_sources_resolver(pool: Pool<PostgresConnectionManager>, user: &User) -> Result<i32> {
    let pg = PgDatabase::from_pool(pool)?;
    let find_rss_query = r#"
        SELECT COUNT(*) AS total FROM sources
        JOIN users_sources ON users_sources.source_uuid = sources.uuid
        WHERE users_sources.user_uuid = $1;
    "#;
    let total = pg.total(find_rss_query, &[&user.uuid])? as i32;
    Ok(total)
}

pub fn find_users_by_source(pg: &PgDatabase, source: &Source) -> Result<Vec<User>> {
    let query = r#"
    SELECT users.* FROM users
    JOIN users_sources ON users_sources.user_uuid = users.uuid
    WHERE users_sources.source_uuid = $1;
    "#;
    Ok(pg.find(query, &[&source.uuid])?)
}

#[derive(Debug)]
pub struct SourceStat {
    pub uuid: Uuid,
    pub count: i64,
}

impl<'a> From<Row<'a>> for SourceStat {
    fn from(row: Row) -> Self {
        SourceStat {
            uuid: row.get("uuid"),
            count: row.get("count"),
        }
    }
}

graphql_object!(SourceStat: Query as "SourceStat" |&self| {
    description: "SourceStat"

    field uuid() -> Uuid as "Source Uuid" {
        self.uuid
    }

    field count() -> i32 as "Count" {
        self.count as i32
    }
});

fn sources_stats(pg: &PgDatabase, user: &User, reaction: &Reaction) -> Result<Vec<SourceStat>> {
    let query = r#"
        SELECT sources.uuid, (
            SELECT COUNT(feeds.uuid)
            FROM feeds
            JOIN users_feeds ON users_feeds.feed_uuid = feeds.uuid
            WHERE feeds.source_uuid = sources.uuid
                AND users_feeds.reaction = $1
                AND users_feeds.user_uuid = $2::uuid
        ) AS count FROM sources
        JOIN users_sources ON users_sources.source_uuid = sources.uuid
        WHERE users_sources.user_uuid = $2::uuid
    "#;
    Ok(pg.find(query, &[reaction, &user.uuid])?)
}

pub fn sources_stats_resolver(pool: Pool<PostgresConnectionManager>, user: &User) -> Result<Vec<SourceStat>> {
    let pg = PgDatabase::from_pool(pool)?;
    Ok(sources_stats(&pg, user, &Reaction::Unreaded)?)
}
