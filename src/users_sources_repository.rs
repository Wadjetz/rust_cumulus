use uuid::Uuid;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

use errors::*;
use schema::users_sources;
use schema::users_sources::dsl::*;
use user_source::UserSource;
use user::User;
use source::Source;

pub fn insert(connection: &PgConnection, user_source: &UserSource) -> Result<UserSource> {
    Ok(diesel::insert_into(users_sources::table).values(user_source).get_result(connection)?)
}

pub fn exists(connection: &PgConnection, searched_source_uuid: &Uuid, user: &User) -> Result<bool> {
    use diesel::dsl::*;
    use diesel::select; 
    Ok(select(exists(users_sources.filter(source_uuid.eq(&searched_source_uuid)).filter(user_uuid.eq(&user.uuid)))).get_result(connection)?) 
}

pub fn my_sources(connection: &PgConnection, limit: i64, offset: i64, user: &User) -> Result<Vec<Source>> {
    use schema::sources::dsl::*;
    let sources_and_users_sources: Vec<(Source, UserSource)> = sources
            .inner_join(users_sources)
            .filter(user_uuid.eq(user.uuid))
            .limit(limit)
            .offset(offset)
            .load::<(Source, UserSource)>(connection)?;

    let s: Vec<Source> = sources_and_users_sources.into_iter().map(|tuple| tuple.0).collect();
    Ok(s)
}

pub fn unfollowed_sources(connection: &PgConnection, limit: i32, offset: i32, user: &User) -> Result<Vec<Source>> {
    // Ok(diesel::sql_query("SELECT sources.* FROM sources WHERE NOT EXISTS (SELECT uuid FROM users_sources WHERE sources.uuid = users_sources.source_uuid AND users_sources.user_uuid = ?) LIMIT ? OFFSET ?")
    Ok(diesel::sql_query("SELECT sources.* FROM sources LIMIT ?")
    .bind::<diesel::sql_types::Integer, _>(limit)
    .bind::<diesel::sql_types::Uuid, _>(&user.uuid)
    .bind::<diesel::sql_types::Integer, _>(offset)
    .get_results::<Source>(connection)?)
}
