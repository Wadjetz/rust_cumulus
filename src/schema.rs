table! {
    use source_type::SqlSourceType;
    use diesel::sql_types::{Nullable, Text, Timestamp, Jsonb, Uuid};
    sources (uuid) {
        uuid -> Uuid,
        source_type -> SqlSourceType,
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
        created -> Nullable<Timestamp>,
        updated -> Nullable<Timestamp>,
    }
}
