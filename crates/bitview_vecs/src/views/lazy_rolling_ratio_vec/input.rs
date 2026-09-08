use brk_types::Height;
use vecdb::{ReadableBoxedVec, VecIndex, VecValue};

/// Own the requested current and preceding ranges before invoking consumers.
/// Overlapping ranges share one read; disjoint ranges skip the unused gap.
pub(super) struct RollingInput<T> {
    values: Vec<T>,
    previous: Option<Vec<T>>,
    base: usize,
    offset: usize,
}

impl<T: VecValue> RollingInput<T> {
    pub(super) fn new(
        source: &ReadableBoxedVec<Height, T>,
        from: usize,
        to: usize,
        starts: &[Height],
    ) -> Self {
        let previous = |start: &Height| start.to_usize().checked_sub(1);
        let first = starts.iter().find_map(previous);
        let last = starts.iter().rev().find_map(previous);
        if let Some((first, last)) = first.zip(last) {
            if last + 1 >= from {
                let base = first.min(from);
                return Self {
                    values: source.collect_range_dyn(base, to),
                    previous: None,
                    base,
                    offset: from - base,
                };
            }
            return Self {
                values: source.collect_range_dyn(from, to),
                previous: Some(source.collect_range_dyn(first, last + 1)),
                base: first,
                offset: 0,
            };
        }
        Self {
            values: source.collect_range_dyn(from, to),
            previous: None,
            base: from,
            offset: 0,
        }
    }

    pub(super) fn current(&self) -> &[T] {
        &self.values[self.offset..]
    }

    pub(super) fn previous(&self, index: usize) -> T {
        self.previous.as_ref().unwrap_or(&self.values)[index - self.base].clone()
    }
}
