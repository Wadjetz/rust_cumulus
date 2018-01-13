use mindstream::models::rss_source::RssSource;
use mindstream::models::twitter_source::TwitterSource;

#[derive(Debug, Serialize, Deserialize)]
pub enum SourceOption {
    Rss(RssSource),
    Twitter(TwitterSource)
}
