use errors::*;

// Custom header type
header! { (XPoweredBy, "X-Powered-By") => [String] }



pub fn fetch_readable(url: &str) -> Result<()> {
    //let client = reqwest::Client::new()?;
    Ok(())
}
