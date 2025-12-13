use schemars::JsonSchema;
use serde::Serialize;

use crate::{BlockHash, Height, Timestamp};

#[derive(Debug, Clone, Serialize, JsonSchema)]
/// Transaction confirmation status
pub struct TxStatus {
    /// Whether the transaction is confirmed
    #[schemars(example = true)]
    pub confirmed: bool,

    /// Block height (only present if confirmed)
    #[schemars(example = Some(916656))]
    pub block_height: Option<Height>,

    /// Block hash (only present if confirmed)
    #[schemars(example = Some("000000000000000000012711f7e0d13e586752a42c66e25faf75f159b3d04911".to_string()))]
    pub block_hash: Option<BlockHash>,

    /// Block timestamp (only present if confirmed)
    #[schemars(example = Some(1759000868))]
    pub block_time: Option<Timestamp>,
}

impl TxStatus {
    pub const UNCONFIRMED: Self = Self {
        confirmed: false,
        block_hash: None,
        block_height: None,
        block_time: None,
    };
}
