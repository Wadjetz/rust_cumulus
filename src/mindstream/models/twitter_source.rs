#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct TwitterSource {
    pub hashtag: Option<String>,
}

impl TwitterSource {
    pub fn new(hashtag: Option<String>) -> Self {
        TwitterSource {
            hashtag
        }
    }
}
