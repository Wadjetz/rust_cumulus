use graphql::query::Query;
use services::mercury::ReadableData;

graphql_object!(ReadableData: Query as "ReadableData" |&self| {
    description: "ReadableData"

    field url() -> &String as "url" {
        &self.url
    }

    field domain() -> &Option<String> as "domain" {
        &self.domain
    }

    field title() -> &Option<String> as "title" {
        &self.title
    }

    field content() -> &Option<String> as "content" {
        &self.content
    }

    field date_published() -> &Option<String> as "date_published" {
        &self.date_published
    }

    field lead_image_url() -> &Option<String> as "lead_image_url" {
        &self.lead_image_url
    }

    field dek() -> &Option<String> as "dek" {
        &self.dek
    }

    field excerpt() -> &Option<String> as "excerpt" {
        &self.excerpt
    }

    field word_count() -> &Option<i32> as "word_count" {
        &self.word_count
    }

    field direction() -> &Option<String> as "direction" {
        &self.direction
    }

    field total_pages() -> &Option<i32> as "total_pages" {
        &self.total_pages
    }

    field rendered_pages() -> &Option<i32> as "rendered_pages" {
        &self.rendered_pages
    }

    field next_page_url() -> &Option<String> as "next_page_url" {
        &self.next_page_url
    }
});
