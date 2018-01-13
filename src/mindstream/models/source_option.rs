use mindstream::sources::{TwitterSource, RssSource};

#[derive(Debug)]
pub enum SourceOption {
    Rss(RssSource),
    Twitter(TwitterSource)
}
