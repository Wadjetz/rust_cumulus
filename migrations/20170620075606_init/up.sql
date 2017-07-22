CREATE TABLE users (
    uuid UUID PRIMARY KEY,
    login TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);

CREATE TABLE bookmarks (
    uuid UUID PRIMARY KEY,
    url TEXT NOT NULL,
    title TEXT,
    description TEXT,
    path TEXT,
    created TIMESTAMP,
    updated TIMESTAMP,
    user_uuid UUID NOT NULL REFERENCES users(uuid)
);

CREATE TABLE files (
    uuid UUID PRIMARY KEY,
    hash TEXT,
    name TEXT NOT NULL,
    location TEXT NOT NULL,
    file_type TEXT NOT NULL,
    size BIGINT,
    user_uuid UUID NOT NULL REFERENCES users(uuid)
);

CREATE TABLE feeds_sources (
    uuid UUID PRIMARY KEY,
    title TEXT NOT NULL,
    xml_url TEXT UNIQUE NOT NULL,
    html_url TEXT NOT NULL,
    error TEXT,
    created TIMESTAMP,
    updated TIMESTAMP
);

CREATE TABLE users_feeds_sources (
    uuid UUID PRIMARY KEY,
    user_uuid UUID NOT NULL REFERENCES users(uuid),
    feeds_sources_uuid UUID NOT NULL REFERENCES feeds_sources(uuid)
);

CREATE TABLE feeds (
    uuid UUID PRIMARY KEY,
    url TEXT UNIQUE NOT NULL,
    rss JSONB,
    readable JSONB,
    twitter JSONB,
    created TIMESTAMP,
    updated TIMESTAMP
)

/*
CREATE TABLE feeds (
    uuid UUID PRIMARY KEY,
    rss_url TEXT,
    rss_guid TEXT,
    rss_title TEXT,
    rss_description TEXT,
    rss_summary TEXT,
    rss_pubdate TEXT,
    rss_image_url TEXT,
    rss_image_title TEXT,
    readable_url TEXT,
    readable_domain TEXT,
    readable_title TEXT,
    readable_content TEXT,
    readable_date_published TEXT,
    readable_lead_image_url TEXT,
    readable_dek TEXT,
    readable_excerpt TEXT,
    readable_word_count BIGINT,
    readable_direction TEXT,
    readable_total_pages BIGINT,
    readable_rendered_pages BIGINT,
    readable_next_page_url TEXT,
    tweet_id Text
);

CREATE TABLE feeds_users (
    uuid UUID PRIMARY KEY,
    feed_uuid UUID NOT NULL REFERENCES feeds(uuid),
    user_uuid UUID NOT NULL REFERENCES users(uuid)
);

CREATE TABLE feeds_feeds_sources (
    uuid UUID PRIMARY KEY,
    feed_uuid UUID NOT NULL REFERENCES feeds(uuid),
    feed_source_uuid UUID NOT NULL REFERENCES feeds_sources(uuid)
);
*/