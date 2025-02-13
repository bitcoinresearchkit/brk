use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryS {
    pub i: String,
    pub v: String,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub format: Option<String>,
}
