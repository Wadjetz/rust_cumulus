use uuid::Uuid;
use postgres::rows::Row;

struct UserFeed {
    pub uuid: Uuid,
    pub feed_uuid: Uuid,
    pub user_uuid: Uuid,
}

impl<'a> From<Row<'a>> for UserFeed {
    fn from(row: Row) -> Self {
        UserFeed {
            uuid: row.get("uuid"),
            user_uuid: row.get("user_uuid"),
            feed_uuid: row.get("feed_uuid"),
        }
    }
}
