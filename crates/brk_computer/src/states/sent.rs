use brk_core::{Sats, StoredUsize};

#[derive(Debug, Default)]
pub struct SentState {
    utxos: StoredUsize,
    sats: Sats,
}
