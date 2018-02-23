#[derive(Debug)]
pub enum SourceOption {
    Rss(RssSource),
    Twitter(TwitterSource)
}

#[derive(GraphQLObject, Debug, Serialize, Deserialize)]
pub struct RssSource {
    pub title: String,
    pub xml_url: String,
    pub html_url: String,
}

impl RssSource {
    pub fn new(title: &str, xml_url: &str, html_url: &str) -> Self {
        RssSource {
            title: title.to_owned(),
            xml_url: xml_url.to_owned(),
            html_url: html_url.to_owned(),
        }
    }
}

#[derive(GraphQLObject, Debug, Serialize, Deserialize)]
pub struct TwitterSource {
    pub hashtag: Option<String>,
}

impl TwitterSource {
    #[allow(dead_code)]
    pub fn new(hashtag: Option<String>) -> Self {
        TwitterSource {
            hashtag
        }
    }
}
