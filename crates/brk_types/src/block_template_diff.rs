use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{NextBlockHash, Transaction, Txid};

/// Delta between the current `getblocktemplate` projection and a prior
/// one identified by `since`. Returned by
/// `GET /api/v1/mining/block-template/diff/{hash}`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockTemplateDiff {
    /// Current next-block hash. Use as `since` on the next diff call.
    pub hash: NextBlockHash,

    /// Echoed prior hash the diff was computed against.
    pub since: NextBlockHash,

    /// Full bodies of transactions that joined the projected next
    /// block since `since`.
    pub added: Vec<Transaction>,

    /// Txids that left the projected next block since `since`
    /// (confirmed, evicted, replaced, or pushed past block 0).
    pub removed: Vec<Txid>,
}
