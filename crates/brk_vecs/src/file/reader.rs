use memmap2::MmapMut;
use parking_lot::RwLockReadGuard;

use crate::file::layout::Layout;

use super::Region;

pub struct Reader<'a, 'b>
where
    'a: 'b,
{
    layout: RwLockReadGuard<'a, Layout>,
    mmap: RwLockReadGuard<'a, MmapMut>,
    region: RwLockReadGuard<'b, Region>,
}

impl<'a, 'b> Reader<'a, 'b>
where
    'a: 'b,
{
    pub fn new(
        mmap: RwLockReadGuard<'a, MmapMut>,
        layout: RwLockReadGuard<'a, Layout>,
        region: RwLockReadGuard<'b, Region>,
    ) -> Self {
        Self {
            mmap,
            layout,
            region,
        }
    }

    pub fn read(&self, offset: usize, len: usize) -> &[u8] {
        assert!(offset + len < self.region.length());

        let start = self.region.start() + offset;
        let end = start + len;
        &self.mmap[start..end]
    }

    pub fn region(&self) -> &Region {
        &self.region
    }
}
