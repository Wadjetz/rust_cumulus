use uuid::Uuid;

#[derive(Debug)]
pub struct UserFeedSource {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub feeds_sources_uuid: Uuid,
}

impl UserFeedSource {
    pub fn new(user_uuid: Uuid, feeds_sources_uuid: Uuid) -> Self {
        UserFeedSource {
            uuid: Uuid::new_v4(),
            user_uuid,
            feeds_sources_uuid,
        }
    }
}
