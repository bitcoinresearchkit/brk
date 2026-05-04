/// Index of a `Cluster` inside `Snapshot::clusters`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ClusterId(u32);

impl ClusterId {
    #[inline]
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn inner(self) -> u32 {
        self.0
    }
}

impl From<u32> for ClusterId {
    #[inline]
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl From<usize> for ClusterId {
    #[inline]
    fn from(v: usize) -> Self {
        debug_assert!(v <= u32::MAX as usize, "ClusterId overflow: {v}");
        Self(v as u32)
    }
}
