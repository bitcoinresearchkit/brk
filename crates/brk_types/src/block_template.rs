use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{MempoolBlock, NextBlockHash, Transaction};

/// Projected next-block contents from Bitcoin Core's `getblocktemplate`
/// (block 0 of the snapshot). Returned by
/// `GET /api/v1/mempool/block-template`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockTemplate {
    /// Pass back as `<hash>` on
    /// `/api/v1/mempool/block-template/diff/{hash}` to fetch deltas.
    pub hash: NextBlockHash,

    /// Aggregate stats for this block (size, vsize, fee range, ...).
    pub stats: MempoolBlock,

    /// Full transaction bodies in `getblocktemplate` order.
    pub transactions: Vec<Transaction>,
}
