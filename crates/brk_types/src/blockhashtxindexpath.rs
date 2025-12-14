use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct BlockHashTxIndexPath {
    /// Bitcoin block hash
    #[schemars(example = &"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f")]
    pub hash: String,

    /// Transaction index within the block (0-based)
    #[schemars(example = 0)]
    pub index: usize,
}
