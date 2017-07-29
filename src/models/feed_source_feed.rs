use uuid::Uuid;

#[derive(Debug)]
pub struct FeedSourceFeed {
    pub uuid: Uuid,
    pub feed_uuid: Uuid,
    pub feed_source_uuid: Uuid,
}

#[allow(dead_code)]
impl FeedSourceFeed {
    pub fn new(feed_uuid: Uuid, feed_source_uuid: Uuid) -> Self {
        FeedSourceFeed {
            uuid: Uuid::new_v4(),
            feed_uuid,
            feed_source_uuid,
        }
    }
}
