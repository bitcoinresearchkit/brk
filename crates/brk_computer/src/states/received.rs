use brk_core::{Sats, StoredUsize};

#[derive(Debug, Default)]
pub struct ReceivedState {
    utxos: StoredUsize,
    sats: Sats,
    unspendable: Sats,
}
