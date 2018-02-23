use uuid::Uuid;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use diesel::PgConnection;

use errors::*;
use source::Source;
use user_source::UserSource;
use user::User;
use sources_repository;
use users_sources_repository;

pub fn fallow_source_resolver(pool: &Pool<ConnectionManager<PgConnection>>, source_uuid: &str, user: &User) -> Result<Source> {
    let connection = pool.get()?;
    let source_uuid = Uuid::parse_str(source_uuid)?;
    let source = sources_repository::find_by_uuid(&connection, &source_uuid)?;
    if !users_sources_repository::exists(&connection, &source_uuid, user)? {
        let user_source = UserSource::new(user.uuid.clone(), source.uuid.clone());
        let _ = users_sources_repository::insert(&connection, &user_source)?;
        Ok(source)
    } else {
        Err(ErrorKind::AlreadyExist.into())
    }
}
