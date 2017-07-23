use graphql::query::Query;
use models::feed::{Rss, Feed};
use services::mercury::ReadableData;

graphql_object!(Feed: Query as "Feed" |&self| {
    description: "Feed"

    field uuid() -> String as "uuid" {
        self.uuid.hyphenated().to_string()
    }

    field url() -> &String as "url" {
        &self.url
    }

    field rss() -> &Option<Rss> as "rss" {
        &self.rss
    }

    field readable() -> &Option<ReadableData> as "readable" {
        &self.readable
    }
    /*
    field twitter() -> &String as "twitter" {
        &String::from("twitter")
    }*/

    field created() -> String as "created" {
        format!("{}", self.created)
    }

    field updated() -> String as "updated" {
        format!("{}", self.updated)
    }
});
