use uuid::Uuid;

use schema::users_sources;

#[derive(Debug, PartialEq, Identifiable, Queryable, Insertable)]
#[belongs_to(User)]
#[belongs_to(Source)]
#[primary_key(uuid)]
#[table_name="users_sources"]
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
