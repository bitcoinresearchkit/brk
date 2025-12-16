use schemars::JsonSchema;
use serde::Deserialize;

use crate::BlockHash;

#[derive(Deserialize, JsonSchema)]
pub struct BlockHashParam {
    pub hash: BlockHash,
}
