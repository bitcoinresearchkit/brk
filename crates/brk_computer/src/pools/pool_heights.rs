use std::sync::Arc;

use brk_types::{Height, PoolSlug};
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use vecdb::{AnyVec, BytesVec, VecIndex};

#[derive(Clone, Default)]
pub struct PoolHeights(Arc<RwLock<FxHashMap<PoolSlug, Vec<Height>>>>);

impl PoolHeights {
    pub fn build(pool: &BytesVec<Height, PoolSlug>) -> Self {
        let len = pool.len();
        let mut map: FxHashMap<PoolSlug, Vec<Height>> = FxHashMap::default();
        let reader = pool.reader();
        for h in 0..len {
            map.entry(reader.get(h)).or_default().push(Height::from(h));
        }
        Self(Arc::new(RwLock::new(map)))
    }

    pub fn truncate(&self, min: usize) {
        let mut cache = self.0.write();
        for heights in cache.values_mut() {
            let cut = heights.partition_point(|h| h.to_usize() < min);
            heights.truncate(cut);
        }
    }

    pub fn push(&self, slug: PoolSlug, height: Height) {
        self.0.write().entry(slug).or_default().push(height);
    }

    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, FxHashMap<PoolSlug, Vec<Height>>> {
        self.0.read()
    }
}
