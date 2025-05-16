#![allow(unused)]

use brk_core::{Sats, StoredU32};

pub struct BlockState {
    utxos: StoredU32,
    value: Sats,
}
