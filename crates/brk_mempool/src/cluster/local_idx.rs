/// Index of a node within a single `Cluster`. Cluster-local; meaningless
/// across clusters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct LocalIdx(u32);

impl LocalIdx {
    pub const ZERO: Self = Self(0);

    #[inline]
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn inner(self) -> u32 {
        self.0
    }
}

impl From<u32> for LocalIdx {
    #[inline]
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl From<usize> for LocalIdx {
    #[inline]
    fn from(v: usize) -> Self {
        debug_assert!(v <= u32::MAX as usize, "LocalIdx overflow: {v}");
        Self(v as u32)
    }
}
