use uuid::Uuid;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

use errors::*;
use schema::sources;
use schema::sources::dsl::*;
use source::Source;

#[allow(dead_code)]
pub fn insert(connection: &PgConnection, source: Source) -> Result<Source> {
    Ok(diesel::insert_into(sources::table).values(&source).get_result(connection)?)
}

#[allow(dead_code)]
pub fn find_by_uuid(connection: &PgConnection, searched_uuid: &Uuid) -> Result<Source> {
    Ok(sources.filter(uuid.eq(searched_uuid)).first::<Source>(&*connection)?)
}
