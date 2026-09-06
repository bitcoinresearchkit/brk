use std::{
    mem,
    sync::{
        Arc, Weak,
        atomic::{AtomicBool, Ordering},
    },
};

use parking_lot::{Mutex, RwLock, RwLockWriteGuard};

use crate::{Database, RegionMetadata, WeakDatabase, region_group::RegionGroupInner};

#[derive(Debug)]
pub struct RegionInner {
    pub db: WeakDatabase,
    pub index: usize,
    accessed: AtomicBool,
    pub meta: RwLock<RegionMetadata>,
    /// Sorted, merged dirty byte ranges relative to the region start.
    dirty_ranges: Mutex<Vec<(usize, usize)>>,
    group: RwLock<Weak<RegionGroupInner>>,
}

impl RegionInner {
    pub fn new(db: &Database, index: usize, meta: RegionMetadata) -> Self {
        Self {
            db: db.weak_clone(),
            index,
            accessed: AtomicBool::new(false),
            meta: RwLock::new(meta),
            dirty_ranges: Mutex::new(Vec::new()),
            group: RwLock::new(Weak::new()),
        }
    }

    #[inline(always)]
    pub fn mark_accessed(&self) {
        self.accessed.store(true, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn was_accessed(&self) -> bool {
        self.accessed.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn group(&self) -> Option<Arc<RegionGroupInner>> {
        self.group.read().upgrade()
    }

    #[inline]
    pub fn set_group(&self, group: Weak<RegionGroupInner>) {
        *self.group.write() = group;
    }

    #[inline(always)]
    pub fn meta_mut(&self) -> RwLockWriteGuard<'_, RegionMetadata> {
        self.meta.write()
    }

    #[inline]
    pub fn mark_dirty(&self, offset: usize, len: usize) {
        if len == 0 {
            return;
        }
        let end = offset + len;
        let mut ranges = self.dirty_ranges.lock();
        let mut start = offset;
        let mut end = end;
        let at = ranges.partition_point(|&(_, range_end)| range_end < start);
        while at < ranges.len() && ranges[at].0 <= end {
            let range = ranges.remove(at);
            start = start.min(range.0);
            end = end.max(range.1);
        }
        ranges.insert(at, (start, end));
    }

    #[inline]
    pub fn take_dirty_ranges(&self) -> Vec<(usize, usize)> {
        mem::take(&mut *self.dirty_ranges.lock())
    }

    #[inline]
    pub fn restore_dirty_ranges(&self, ranges: &[(usize, usize)]) {
        for &(start, end) in ranges {
            self.mark_dirty(start, end - start);
        }
    }
}
