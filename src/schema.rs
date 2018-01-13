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
    conversations (uuid) {
        uuid -> Uuid,
        level -> Int4,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    use diesel::types::{Nullable, Timestamp, Jsonb, Uuid, Text};
    use mindstream::models::sourcetype::SourceTypeMapper;
    sources (uuid) {
        uuid -> Uuid,
        source_type -> SourceTypeMapper,
        data -> Nullable<Jsonb>,
        error -> Nullable<Text>,
        created -> Nullable<Timestamp>,
        updated -> Nullable<Timestamp>,
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
    conversations,
    sources,
    users,
);
