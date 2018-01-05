pub mod feeds;
pub mod sources;
pub mod users_feeds;
pub mod users_sources;
pub mod mercury;
pub mod rss;

use uuid::Uuid;
use errors::*;
use rocket_contrib::Json;
use pg::DbConn;
use mindstream;
use pg::PgDatabase;
use users;

pub fn export_resolver(conn: DbConn, _user_uuid: Uuid) -> Result<Json> {
    let pg: PgDatabase = conn.into();
    let sources = mindstream::sources::find_all(&pg)?;
    let users_sources = mindstream::users_sources::find_all(&pg)?;
    let users = users::find_all(&pg)?;
    let feeds = mindstream::feeds::find_all(&pg)?;
    let users_feeds = mindstream::users_feeds::find_all(&pg)?;
    Ok(Json(json!({
        "users": users,
        "sources": sources,
        "feeds": feeds,
        "users_sources": users_sources,
        "users_feeds": users_feeds,
    })))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Import {
    pub users: Vec<users::User>,
    pub sources: Vec<mindstream::sources::Source>,
    pub feeds: Vec<mindstream::feeds::Feed>,
    pub users_sources: Vec<mindstream::users_sources::UserSource>,
    pub users_feeds: Vec<mindstream::users_feeds::UserFeed>,
}

pub fn import_resolver(conn: DbConn, import: Import) -> Result<String> {
    let _pg: PgDatabase = conn.into();
    Ok(format!("{:?}", import))
}
