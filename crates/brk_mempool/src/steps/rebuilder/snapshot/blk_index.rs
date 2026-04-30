/// Projected-block index in a mempool snapshot. `u8` because the
/// projection horizon is ~8 blocks at typical loads; `BlkIndex::MAX`
/// is reserved as the "not in any projected block" sentinel used by
/// `Snapshot::block_of` for txs below the mempool floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlkIndex(u8);

impl BlkIndex {
    /// Sentinel for "not in any projected block".
    pub const MAX: BlkIndex = BlkIndex(u8::MAX);

    pub fn is_not_in_projected(self) -> bool {
        self == Self::MAX
    }

    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for BlkIndex {
    fn from(v: usize) -> Self {
        debug_assert!(v < u8::MAX as usize, "BlkIndex overflow: {v}");
        Self(v as u8)
    }
}
