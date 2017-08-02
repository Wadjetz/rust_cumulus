use postgres::rows::Row;
use postgres_shared::types::ToSql;
use juniper::Executor;
use uuid::Uuid;

use errors::*;
use graphql::query::Query;
use models::user::User;
use sources::Source;
use pg::{Insertable, Existable, PgDatabase};

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

    fn insert_params<'a>(&'a self) -> Box<[&'a ToSql]> {
        Box::new([&self.uuid, &self.user_uuid, &self.source_uuid])
    }
}

impl Existable for UserSource {
    fn exist_query() -> String {
        r#"
            SELECT COUNT(*) AS exist FROM users_sources WHERE user_uuid = $1::uuid AND source_uuid = $2::uuid;
        "#.to_owned()
    }
}

pub fn fallow_source_resolver<'a>(executor: &Executor<'a, Query>, uuid: &str, user: &User) -> Result<Source> {
    let connection = executor.context().connection.clone().get()?;
    let pg = PgDatabase::new(connection);
    let uuid = Uuid::parse_str(uuid)?;
    let find_query = r#"SELECT * FROM sources WHERE uuid = $1::uuid;"#;
    let maybe_source = pg.find_one::<Source>(find_query, &[&uuid])?;
    if let Some(source) = maybe_source {
        let exist = pg.exist(&UserSource::exist_query(), &[&user.uuid, &uuid])?;
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
