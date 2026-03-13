use crate::{Address, AddressChainStats, AddressMempoolStats};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Address information compatible with mempool.space API format
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AddressStats {
    /// Bitcoin address string
    #[schemars(
        example = "04678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5f"
    )]
    pub address: Address,

    /// Statistics for confirmed transactions on the blockchain
    pub chain_stats: AddressChainStats,

    /// Statistics for unconfirmed transactions in the mempool
    pub mempool_stats: Option<AddressMempoolStats>,
}
