use graphql::query::Query;
use models::feed::Rss;

graphql_object!(Rss: Query as "Rss" |&self| {
    description: "Rss"

    field id() -> &String as "id" {
        &self.id
    }

    field title() -> &Option<String> as "title" {
        &self.title
    }

    field content() -> &Option<String> as "content" {
        &self.content
    }

    field summary() -> &Option<String> as "summary" {
        &self.summary
    }

    field author() -> &Option<String> as "author" {
        &self.author
    }

    field published() -> &String as "published" {
        &self.published
    }

    field updated() -> &Option<String> as "updated" {
        &self.updated
    }

    field url() -> &Option<String> as "url" {
        &self.alternate
    }

    field keywords() -> &Vec<String> as "keywords" {
        &self.keywords
    }

    field enclosure() -> &Option<String> as "enclosure" {
        &self.enclosure
    }

    field fingerprint() -> &String as "fingerprint" {
        &self.fingerprint
    }
});
