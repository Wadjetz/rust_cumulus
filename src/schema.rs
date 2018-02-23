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
