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
