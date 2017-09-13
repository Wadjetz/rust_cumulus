use uuid::Uuid;
use postgres::rows::Row;
use postgres::types::ToSql;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use errors::*;
use users::User;
use mindstream::sources::Source;
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
