use crate::RangeMap;

const CACHE_SIZE: usize = 1024;
pub(super) type LookupCache<I, V> = [(I, I, V, bool); CACHE_SIZE];

pub(super) fn empty_cache<I: Default + Copy, V: Default + Copy>() -> LookupCache<I, V> {
    [(I::default(), I::default(), V::default(), false); CACHE_SIZE]
}

#[inline]
pub(super) fn cached_get<I: Ord + Copy + Into<usize>, V: From<usize> + Copy>(
    starts: &[I],
    cache: &mut LookupCache<I, V>,
    index: I,
) -> Option<V> {
    if starts.is_empty() {
        return None;
    }
    let entry = &mut cache[index.into() & (CACHE_SIZE - 1)];
    if entry.3 && index >= entry.0 && index < entry.1 {
        return Some(entry.2);
    }
    let position = starts.partition_point(|&start| start <= index);
    let value = V::from(position.checked_sub(1)?);
    if let Some(&end) = starts.get(position) {
        *entry = (starts[position - 1], end, value, true);
    }
    Some(value)
}

/// Cached floor lookups over borrowed boundaries. The borrow pins the mapping
/// for the cursor's lifetime, so the local interval cache cannot become stale.
pub struct CachedRangeMapCursor<'a, I, V> {
    source: &'a RangeMap<I, V>,
    cache: LookupCache<I, V>,
}

impl<'a, I: Default + Copy, V: Default + Copy> CachedRangeMapCursor<'a, I, V> {
    pub(super) fn new(source: &'a RangeMap<I, V>) -> Self {
        Self {
            source,
            cache: empty_cache(),
        }
    }
}

impl<I: Ord + Copy + Into<usize>, V: From<usize> + Copy> CachedRangeMapCursor<'_, I, V> {
    #[inline]
    pub fn get(&mut self, index: I) -> Option<V> {
        cached_get(self.source.as_slice(), &mut self.cache, index)
    }
}
