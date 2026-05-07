/// Compact index into a `Snapshot`'s dense `txs` vec. Snapshot-internal:
/// rebuilt fresh each tick from the live `TxStore`, so consumers
/// can't hold one across rebuilds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TxIndex(u32);

impl TxIndex {
    #[inline]
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for TxIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<TxIndex> for u64 {
    #[inline]
    fn from(value: TxIndex) -> Self {
        u64::from(value.0)
    }
}
