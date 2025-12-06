use crate::{Sats, TxOut};
use schemars::JsonSchema;
use serde::Serialize;

///
/// Address statistics in the mempool (unconfirmed transactions only)
///
/// Based on mempool.space's format.
///
#[derive(Debug, Default, Clone, Serialize, JsonSchema)]
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

impl AddressMempoolStats {
    pub fn receiving(&mut self, txout: &TxOut) {
        self.funded_txo_count += 1;
        self.funded_txo_sum += txout.value;
    }

    pub fn received(&mut self, txout: &TxOut) {
        self.funded_txo_count -= 1;
        self.funded_txo_sum -= txout.value;
    }

    pub fn sending(&mut self, txout: &TxOut) {
        self.spent_txo_count += 1;
        self.spent_txo_sum += txout.value;
    }

    pub fn sent(&mut self, txout: &TxOut) {
        self.spent_txo_count -= 1;
        self.spent_txo_sum -= txout.value;
    }

    pub fn update_tx_count(&mut self, tx_count: u32) {
        self.tx_count = tx_count
    }
}
