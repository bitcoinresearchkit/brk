use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Chunk {
    pub id: usize,
    pub previous: Option<String>,
    pub next: Option<String>,
}
