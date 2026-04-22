use brk_types::{Sats, VSize};
use smallvec::SmallVec;

use crate::types::{PoolIndex, TxIndex};

/// A transaction node in the dependency graph.
///
/// Created fresh for each block building cycle, then discarded.
pub struct TxNode {
    /// Index into mempool entries (carried into the final `Package`).
    pub tx_index: TxIndex,

    /// Transaction fee.
    pub fee: Sats,

    /// Transaction virtual size.
    pub vsize: VSize,

    /// Parent transactions (dependencies).
    pub parents: SmallVec<[PoolIndex; 4]>,

    /// Child transactions (dependents).
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
