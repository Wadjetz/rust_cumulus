use graphql::query::Query;
use models::feed_source::FeedSource;

graphql_object!(FeedSource: Query as "FeedSource" |&self| {
    description: "Feed Source"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
    }

    field title() -> &String as "title" {
        &self.title
    }

    field xml_url() -> &String as "xml_url" {
        &self.xml_url
    }

    field html_url() -> &String as "html_url" {
        &self.html_url
    }

    field error() -> &Option<String> as "error" {
        &self.error
    }

    field created() -> String as "created" {
        format!("{}", self.created)
    }

    field updated() -> String as "updated" {
        format!("{}", self.updated)
    }
});
