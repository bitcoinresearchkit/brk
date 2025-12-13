use std::ops::{Index, IndexMut};

use brk_types::{Sats, VSize};
use smallvec::SmallVec;

use crate::mempool::{MempoolTxIndex, PoolIndex};

/// Compare ancestor fee rates using cross-multiplication (avoids f64 division).
/// Returns true if (fee_a / vsize_a) > (fee_b / vsize_b).
#[inline]
fn has_higher_fee_rate(fee_a: Sats, vsize_a: VSize, fee_b: Sats, vsize_b: VSize) -> bool {
    // Cross multiply: fee_a/vsize_a > fee_b/vsize_b  ⟺  fee_a * vsize_b > fee_b * vsize_a
    let score_a = u64::from(fee_a) as u128 * u64::from(vsize_b) as u128;
    let score_b = u64::from(fee_b) as u128 * u64::from(vsize_a) as u128;
    score_a > score_b
}

/// Type-safe wrapper around Vec<AuditTx> that only allows PoolIndex access.
pub struct Pool(Vec<AuditTx>);

impl Pool {
    pub fn new(txs: Vec<AuditTx>) -> Self {
        Self(txs)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<PoolIndex> for Pool {
    type Output = AuditTx;

    #[inline]
    fn index(&self, idx: PoolIndex) -> &Self::Output {
        &self.0[idx.as_usize()]
    }
}

impl IndexMut<PoolIndex> for Pool {
    #[inline]
    fn index_mut(&mut self, idx: PoolIndex) -> &mut Self::Output {
        &mut self.0[idx.as_usize()]
    }
}

/// Lightweight transaction for block building.
/// Created fresh each rebuild, discarded after.
pub struct AuditTx {
    /// Original entries index (for final output)
    pub entries_idx: MempoolTxIndex,
    /// Pool index (for internal graph traversal)
    pub pool_idx: PoolIndex,
    pub fee: Sats,
    pub vsize: VSize,
    /// In-mempool parent pool indices
    pub parents: SmallVec<[PoolIndex; 4]>,
    /// In-mempool child pool indices
    pub children: SmallVec<[PoolIndex; 8]>,
    /// Cumulative fee (self + all ancestors)
    pub ancestor_fee: Sats,
    /// Cumulative vsize (self + all ancestors)
    pub ancestor_vsize: VSize,
    /// Already selected into a block
    pub used: bool,
    /// Generation counter for invalidating stale heap entries
    pub generation: u32,
}

impl AuditTx {
    /// Create AuditTx with pre-computed ancestor values from Bitcoin Core.
    pub fn new_with_ancestors(
        entries_idx: MempoolTxIndex,
        pool_idx: PoolIndex,
        fee: Sats,
        vsize: VSize,
        ancestor_fee: Sats,
        ancestor_vsize: VSize,
    ) -> Self {
        Self {
            entries_idx,
            pool_idx,
            fee,
            vsize,
            parents: SmallVec::new(),
            children: SmallVec::new(),
            ancestor_fee,
            ancestor_vsize,
            used: false,
            generation: 0,
        }
    }
}

/// Priority queue entry. Stores snapshot of score at insertion time.
#[derive(Clone, Copy)]
pub struct TxPriority {
    pub pool_idx: PoolIndex,
    /// Score snapshot for heap ordering
    ancestor_fee: Sats,
    ancestor_vsize: VSize,
    /// Generation at insertion (detects stale entries)
    pub generation: u32,
}

impl TxPriority {
    pub fn new(tx: &AuditTx) -> Self {
        Self {
            pool_idx: tx.pool_idx,
            ancestor_fee: tx.ancestor_fee,
            ancestor_vsize: tx.ancestor_vsize,
            generation: tx.generation,
        }
    }

    /// Check if this entry is stale (tx was updated since insertion).
    #[inline]
    pub fn is_stale(&self, tx: &AuditTx) -> bool {
        self.generation != tx.generation
    }

    #[inline]
    fn has_higher_score_than(&self, other: &Self) -> bool {
        has_higher_fee_rate(
            self.ancestor_fee,
            self.ancestor_vsize,
            other.ancestor_fee,
            other.ancestor_vsize,
        )
    }
}

impl PartialEq for TxPriority {
    fn eq(&self, other: &Self) -> bool {
        self.pool_idx == other.pool_idx
    }
}

impl Eq for TxPriority {}

impl PartialOrd for TxPriority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TxPriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher score = higher priority (for max-heap)
        if self.has_higher_score_than(other) {
            std::cmp::Ordering::Greater
        } else if other.has_higher_score_than(self) {
            std::cmp::Ordering::Less
        } else {
            // Tiebreaker: lower index first (deterministic)
            other.pool_idx.cmp(&self.pool_idx)
        }
    }
}
