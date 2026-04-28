use brk_types::{Sats, VSize};
use smallvec::SmallVec;

use super::PoolIndex;
use crate::stores::TxIndex;

/// Built fresh per block-building cycle, then discarded.
pub struct TxNode {
    pub tx_index: TxIndex,
    pub fee: Sats,
    pub vsize: VSize,
    pub parents: SmallVec<[PoolIndex; 4]>,
    pub children: SmallVec<[PoolIndex; 8]>,
}

impl TxNode {
    pub fn new(tx_index: TxIndex, fee: Sats, vsize: VSize) -> Self {
        Self {
            tx_index,
            fee,
            vsize,
            parents: SmallVec::new(),
            children: SmallVec::new(),
        }
    }
}
