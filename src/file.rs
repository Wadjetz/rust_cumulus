use query::Query;

pub struct File {
    id: String,
    path: String,
    size: u64,
}

impl File {
    pub fn new(id: String, path: String, size: u64) -> Self {
        File {
            id, path, size
        }
    }
}

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
