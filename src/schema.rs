table! {
    sources (uuid) {
        uuid -> Uuid,
        error -> Nullable<Text>,
        created -> Timestamp,
        updated -> Timestamp,
        url -> Text,
        title -> Text,
        website -> Text,
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

table! {
    users_sources (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        source_uuid -> Uuid,
    }
}

joinable!(users_sources -> sources (source_uuid));
joinable!(users_sources -> users (user_uuid));

allow_tables_to_appear_in_same_query!(
    sources,
    users,
    users_sources,
);
