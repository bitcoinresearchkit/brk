use schemars::JsonSchema;
use serde::Deserialize;

use crate::{BlockHash, TxIndex};

#[derive(Deserialize, JsonSchema)]
pub struct BlockHashStartIndex {
    /// Bitcoin block hash
    pub hash: BlockHash,

    /// Starting transaction index within the block (0-based)
    #[schemars(example = 0)]
    pub start_index: TxIndex,
}
