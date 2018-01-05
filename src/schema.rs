table! {
    diesel_bookmarks (uuid) {
        uuid -> Uuid,
        url -> Text,
        title -> Text,
        description -> Nullable<Text>,
        path -> Nullable<Text>,
        created -> Timestamp,
        updated -> Timestamp,
        user_uuid -> Uuid,
    }
}
