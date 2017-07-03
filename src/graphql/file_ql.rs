use graphql::query::Query;
use models::file::File;

graphql_object!(File: Query as "File" |&self| {
    description: "A file"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
    }

    field hash() -> &Option<String> as "hash" {
        &self.hash
    }

    field name() -> &String as "name" {
        &self.name
    }

    field location() -> &String as "location" {
        &self.location
    }

    field file_type() -> String as "file_type" {
        self.file_type.to_string()
    }

    field size() -> Option<String> as "size" {
        self.size.map(|s| s.to_string())
    }

    field user_uuid() -> String as "uuid" {
        self.user_uuid.hyphenated().to_string()
    }
});
