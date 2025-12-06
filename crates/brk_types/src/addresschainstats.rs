use crate::{Sats, TypeIndex};
use schemars::JsonSchema;
use serde::Serialize;

/// Address statistics on the blockchain (confirmed transactions only)
///
/// Based on mempool.space's format with type_index extension.
#[derive(Debug, Default, Serialize, JsonSchema)]
pub struct AddressChainStats {
    /// Total number of transaction outputs that funded this address
    #[schemars(example = 5)]
    pub funded_txo_count: u32,

    /// Total amount in satoshis received by this address across all funded outputs
    #[schemars(example = Sats::new(15007599040))]
    pub funded_txo_sum: Sats,

    /// Total number of transaction outputs spent from this address
    #[schemars(example = 5)]
    pub spent_txo_count: u32,

    /// Total amount in satoshis spent from this address
    #[schemars(example = Sats::new(15007599040))]
    pub spent_txo_sum: Sats,

    /// Total number of confirmed transactions involving this address
    #[schemars(example = 10)]
    pub tx_count: u32,

    /// Index of this address within its type on the blockchain
    #[schemars(example = TypeIndex::new(0))]
    pub type_index: TypeIndex,
}
