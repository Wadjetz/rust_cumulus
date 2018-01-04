use reqwest::Client;
use config::CONFIG;

use errors::*;

header! { (XApiKey, "x-api-key") => [String] }

#[derive(GraphQLObject, Debug, Clone, Serialize, Deserialize)]
pub struct ReadableData {
    pub url: String,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub date_published: Option<String>,
    pub lead_image_url: Option<String>,
    pub dek: Option<String>,
    pub excerpt: Option<String>,
    pub word_count: Option<i32>,
    pub direction: Option<String>,
    pub total_pages: Option<i32>,
    pub rendered_pages: Option<i32>,
    pub next_page_url: Option<String>,
}

pub fn fetch_readable(client: &Client, url: &str) -> Result<Option<ReadableData>> {
    let url = format!("http://mercury.postlight.com/parser?url={}", url);
    let api_key = &CONFIG.mercury_api_key;
    let mut response = client.get(&url)
        .header(XApiKey(api_key.to_owned()))
        .send()?;
    let readable_data: Option<ReadableData> = response.json()?;
    Ok(readable_data)
}
