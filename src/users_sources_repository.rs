use uuid::Uuid;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

use errors::*;
use schema::users_sources;
use schema::users_sources::dsl::*;
use user_source::UserSource;
use user::User;

pub fn insert(connection: &PgConnection, user_source: &UserSource) -> Result<UserSource> {
    Ok(diesel::insert_into(users_sources::table).values(user_source).get_result(connection)?)
}

pub fn exists(connection: &PgConnection, searched_source_uuid: &Uuid, user: &User) -> Result<bool> {
    use diesel::dsl::*;
    use diesel::select; 
    Ok(select(exists(users_sources.filter(source_uuid.eq(&searched_source_uuid)).filter(user_uuid.eq(&user.uuid)))).get_result(connection)?) 
}

