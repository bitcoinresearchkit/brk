use crate::Sats;
use schemars::JsonSchema;
use serde::Serialize;

///
/// Address statistics in the mempool (unconfirmed transactions only)
///
/// Based on mempool.space's format.
///
#[derive(Debug, Serialize, JsonSchema)]
pub struct AddressMempoolStats {
    /// Number of unconfirmed transaction outputs funding this address
    #[schemars(example = 0)]
    pub funded_txo_count: u32,

    /// Total amount in satoshis being received in unconfirmed transactions
    #[schemars(example = Sats::new(0))]
    pub funded_txo_sum: Sats,

    /// Number of unconfirmed transaction inputs spending from this address
    #[schemars(example = 0)]
    pub spent_txo_count: u32,

    /// Total amount in satoshis being spent in unconfirmed transactions
    #[schemars(example = Sats::new(0))]
    pub spent_txo_sum: Sats,

    /// Number of unconfirmed transactions involving this address
    #[schemars(example = 0)]
    pub tx_count: u32,
}
