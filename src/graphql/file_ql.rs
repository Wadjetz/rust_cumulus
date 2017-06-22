use graphql::query::Query;
use file::File;

graphql_object!(File: Query as "File" |&self| {
    description: "A file"

    field id() -> String as "id" {
        self.id.to_string()
    }

    field path() -> String as "path" {
        self.path.to_string()
    }

    field size() -> String as "size" {
        format!("{}", self.size)
    }
});
