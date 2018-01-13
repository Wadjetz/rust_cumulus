use postgres::rows::Row;
use r2d2_postgres::PostgresConnectionManager;
use uuid::Uuid;

use mindstream::models::source::Source;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use r2d2::Pool;
use schema::sources;

use errors::*;
use pg::PgDatabase;

impl<'a> From<Row<'a>> for Source {
    fn from(row: Row) -> Self {
        Source {
            uuid: row.get("uuid"),
            source_type: row.get("source_type"),
            data: row.get("data"),
            error: row.get("error"),
            created: row.get("created"),
            updated: row.get("updated"),
        }
    }
}

pub fn insert(connection: &PgConnection, source: &Source) -> Result<Source> {
    Ok(diesel::insert_into(sources::table)
            .values(source)
            .get_result(&*connection)?)
}

pub fn find_sources_resolver(pool: Pool<PostgresConnectionManager>, limit: i32, offset: i32) -> Result<Vec<Source>> {
    let pg = PgDatabase::from_pool(pool)?;
    let find_query = r#"SELECT * FROM sources LIMIT $1::int OFFSET $2::int;"#;
    let sources = pg.find(find_query, &[&limit, &offset])?;
    Ok(sources)
}

pub fn find_rss_sources(pg: &PgDatabase, limit: i32, offset: i32) -> Result<Vec<Source>> {
    let find_rss_query = r#"SELECT * FROM sources WHERE source_type = 'Rss' LIMIT $1::int OFFSET $2::int;"#;
    let sources = pg.find::<Source>(find_rss_query, &[&limit, &offset])?;
    Ok(sources)
}

pub fn find_source_by_uuid(pg: &PgDatabase, uuid: &Uuid) -> Result<Option<Source>> {
    let query = "SELECT * FROM sources WHERE uuid = $1::uuid;";
    let source = pg.find_one(query, &[uuid])?;
    Ok(source)
}