/// Index into the temporary pool used during block building.
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
