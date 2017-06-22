use graphql::query::Query;
use bookmark::Bookmark;

graphql_object!(Bookmark: Query as "Bookmark" |&self| {
    description: "Bookmark"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
    }

    field url() -> &String as "url" {
        &self.url
    }

    field title() -> &String as "title" {
        &self.title
    }

    field description() -> &String as "description" {
        &self.description
    }

    field path() -> &String as "path" {
        &self.path
    }

    field created() -> String as "created" {
        format!("{}", self.created)
    }

    field updated() -> String as "updated" {
        format!("{}", self.updated)
    }

    field user_uuid() -> String as "user_uuid" {
        self.user_uuid.hyphenated().to_string()
    }
});
