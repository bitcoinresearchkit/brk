/// Index into the mempool entries Vec.
/// NOT the global TxIndex for confirmed transactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MempoolTxIndex(pub(crate) u32);

impl MempoolTxIndex {
    #[inline]
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for MempoolTxIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

/// Index into the temporary pool Vec used during block building.
/// Distinct from MempoolTxIndex to prevent mixing up index spaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PoolIndex(u32);

impl PoolIndex {
    #[inline]
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for PoolIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

/// A selected transaction with its effective mining score at selection time.
/// The effective_fee_rate is the ancestor score when this tx was selected,
/// which may differ from the original ancestor score (if ancestors were already mined).
#[derive(Debug, Clone, Copy)]
pub struct SelectedTx {
    pub entries_idx: MempoolTxIndex,
    /// Fee rate at selection time (ancestor_fee / ancestor_vsize)
    pub effective_fee_rate: brk_types::FeeRate,
}
