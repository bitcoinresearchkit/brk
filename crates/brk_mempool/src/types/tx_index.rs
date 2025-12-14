/// Index into the mempool entries storage.
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
