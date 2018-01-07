table! {
    bookmarks (uuid) {
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

table! {
    users (uuid) {
        uuid -> Uuid,
        login -> Text,
        email -> Text,
        password -> Text,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

joinable!(bookmarks -> users (user_uuid));

allow_tables_to_appear_in_same_query!(
    bookmarks,
    users,
);
