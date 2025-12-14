use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct BlockCountPath {
    /// Number of blocks to include in the stats
    #[schemars(example = 100)]
    pub block_count: usize,
}
