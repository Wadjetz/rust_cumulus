#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
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
