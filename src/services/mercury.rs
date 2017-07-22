use errors::*;

// Custom header type
header! { (XPoweredBy, "X-Powered-By") => [String] }


#[derive(Debug, Serialize, Deserialize)]
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

pub fn fetch_readable(url: &str) -> Result<()> {
    //let client = reqwest::Client::new()?;
    Ok(())
}
