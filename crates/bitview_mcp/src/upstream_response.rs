pub struct UpstreamResponse {
    pub url: String,
    pub status: u16,
    pub content_type: String,
    pub body: Vec<u8>,
    pub cache_status: Option<String>,
    pub cache_age: Option<String>,
}
