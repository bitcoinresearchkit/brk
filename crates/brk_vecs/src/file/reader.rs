use memmap2::MmapMut;
use parking_lot::RwLockReadGuard;

use super::Region;

#[derive(Debug)]
pub struct Reader<'a> {
    mmap: RwLockReadGuard<'a, MmapMut>,
    region: RwLockReadGuard<'static, Region>,
}

impl<'a> Reader<'a> {
    pub fn new(
        mmap: RwLockReadGuard<'a, MmapMut>,
        region: RwLockReadGuard<'static, Region>,
    ) -> Self {
        Self { mmap, region }
    }

    pub fn read(&self, offset: u64, len: u64) -> &[u8] {
        debug_assert!(offset + len <= self.region.len());
        let start = self.region.start() + offset;
        let end = start + len;
        &self.mmap[start as usize..end as usize]
    }

    pub fn read_all(&self) -> &[u8] {
        self.read(0, self.region().len())
    }

    pub fn region(&self) -> &Region {
        &self.region
    }
}
