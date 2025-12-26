use std::cmp::Ordering;

use brk_types::{Sats, VSize};

use super::tx_node::TxNode;
use crate::types::PoolIndex;

/// Entry in the priority heap for transaction selection.
///
/// Stores a snapshot of the score at insertion time.
#[derive(Clone, Copy)]
pub struct HeapEntry {
    pub pool_index: PoolIndex,
    ancestor_fee: Sats,
    ancestor_vsize: VSize,
}

impl HeapEntry {
    pub fn new(node: &TxNode) -> Self {
        Self {
            pool_index: node.pool_index,
            ancestor_fee: node.ancestor_fee,
            ancestor_vsize: node.ancestor_vsize,
        }
    }

    /// Compare fee rates: self > other?
    #[inline]
    fn has_higher_fee_rate_than(&self, other: &Self) -> bool {
        // Cross multiply to avoid division:
        // fee_a/vsize_a > fee_b/vsize_b  ⟺  fee_a * vsize_b > fee_b * vsize_a
        let self_score =
            u64::from(self.ancestor_fee) as u128 * u64::from(other.ancestor_vsize) as u128;
        let other_score =
            u64::from(other.ancestor_fee) as u128 * u64::from(self.ancestor_vsize) as u128;
        self_score > other_score
    }
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.pool_index == other.pool_index
    }
}

impl Eq for HeapEntry {}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher fee rate = higher priority
        if self.has_higher_fee_rate_than(other) {
            Ordering::Greater
        } else if other.has_higher_fee_rate_than(self) {
            Ordering::Less
        } else {
            // Tiebreaker: lower index first (deterministic)
            other.pool_index.cmp(&self.pool_index)
        }
    }
}
