use uuid::Uuid;

#[derive(Debug)]
pub struct UserFeedSource {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub feed_source_uuid: Uuid,
}

impl UserFeedSource {
    #[allow(dead_code)]
    pub fn new(user_uuid: Uuid, feed_source_uuid: Uuid) -> Self {
        UserFeedSource {
            uuid: Uuid::new_v4(),
            user_uuid,
            feed_source_uuid,
        }
    }
}
