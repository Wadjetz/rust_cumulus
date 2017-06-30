use graphql::query::Query;
use file::{File, FileType};

graphql_object!(File: Query as "File" |&self| {
    description: "A file"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
    }

    field hash() -> &String as "hash" {
        &self.hash
    }

    field name() -> &String as "name" {
        &self.name
    }

    field parent() -> &String as "parent" {
        &self.parent
    }

    field location() -> &String as "location" {
        &self.location
    }

    field file_type() -> String as "file_type" {
        match self.file_type {
            FileType::File => "file".to_string(),
            FileType::Directory => "directory".to_string(),
        }
    }

    field size() -> String as "size" {
        format!("{}", self.size)
    }
});
