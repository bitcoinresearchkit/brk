/// Index of a `Chunk` inside a `Cluster.chunks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ChunkId(u32);

impl ChunkId {
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

impl From<u32> for ChunkId {
    #[inline]
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl From<usize> for ChunkId {
    #[inline]
    fn from(v: usize) -> Self {
        debug_assert!(v <= u32::MAX as usize, "ChunkId overflow: {v}");
        Self(v as u32)
    }
}
