use brk_types::{Sats, VSize};
use smallvec::SmallVec;

use crate::types::{PoolIndex, TxIndex};

/// A transaction node in the dependency graph.
///
/// Created fresh for each block building cycle, then discarded.
pub struct TxNode {
    /// Index into mempool entries (for final output)
    pub tx_index: TxIndex,

    /// Index in the graph pool
    pub pool_index: PoolIndex,

    /// Transaction fee
    pub fee: Sats,

    /// Transaction virtual size
    pub vsize: VSize,

    /// Parent transactions (dependencies)
    pub parents: SmallVec<[PoolIndex; 4]>,

    /// Child transactions (dependents)
    pub children: SmallVec<[PoolIndex; 8]>,

    /// Cumulative fee (self + all ancestors)
    pub ancestor_fee: Sats,

    /// Cumulative vsize (self + all ancestors)
    pub ancestor_vsize: VSize,

    /// Whether this tx has been selected
    pub selected: bool,

    /// Generation counter for heap staleness detection
    pub generation: u32,
}

impl TxNode {
    pub fn new(
        tx_index: TxIndex,
        pool_index: PoolIndex,
        fee: Sats,
        vsize: VSize,
        ancestor_fee: Sats,
        ancestor_vsize: VSize,
    ) -> Self {
        Self {
            tx_index,
            pool_index,
            fee,
            vsize,
            parents: SmallVec::new(),
            children: SmallVec::new(),
            ancestor_fee,
            ancestor_vsize,
            selected: false,
            generation: 0,
        }
    }
}
