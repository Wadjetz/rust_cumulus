use uuid::Uuid;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

use errors::*;
use schema::sources;
use schema::sources::dsl::*;
use source::Source;

pub fn insert(connection: &PgConnection, source: Source) -> Result<Source> {
    Ok(diesel::insert_into(sources::table).values(&source).get_result(connection)?)
}

#[allow(dead_code)]
pub fn find_by_uuid(connection: &PgConnection, searched_uuid: &Uuid) -> Result<Source> {
    Ok(sources.filter(uuid.eq(searched_uuid)).first::<Source>(&*connection)?)
}

pub fn find(connection: &PgConnection, limit: i64, offset: i64) -> Result<Vec<Source>> {
    Ok(sources.limit(limit).offset(offset).load::<Source>(&*connection)?)
}

#[allow(dead_code)]
pub fn exists(connection: &PgConnection, searched_url: &str) -> Result<bool> {
    use diesel::dsl::exists;
    use diesel::select;
    Ok(select(exists(sources.filter(url.eq(searched_url)))).get_result(&*connection)?)
}
