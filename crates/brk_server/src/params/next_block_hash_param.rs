use schemars::JsonSchema;
use serde::Deserialize;

use brk_types::NextBlockHash;

/// `since` hash for `/api/v1/mining/block-template/diff/{hash}`.
#[derive(Deserialize, JsonSchema)]
pub struct NextBlockHashParam {
    pub hash: NextBlockHash,
}
