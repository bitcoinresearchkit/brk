use brk_computer::Computer;
use brk_types::{DateIndex, Height, Timestamp};
use vecdb::{GenericStoredVec, IterableVec, VecIndex};

/// Helper for iterating over dateindex ranges with sampling.
pub struct DateIndexIter<'a> {
    computer: &'a Computer,
    start_di: DateIndex,
    end_di: DateIndex,
    step: usize,
}

impl<'a> DateIndexIter<'a> {
    pub fn new(computer: &'a Computer, start_height: usize, end_height: usize) -> Self {
        let start_di = computer
            .indexes
            .height_to_dateindex
            .read_once(Height::from(start_height))
            .unwrap_or_default();
        let end_di = computer
            .indexes
            .height_to_dateindex
            .read_once(Height::from(end_height))
            .unwrap_or_default();

        let total = end_di.to_usize().saturating_sub(start_di.to_usize()) + 1;
        let step = (total / 200).max(1);

        Self {
            computer,
            start_di,
            end_di,
            step,
        }
    }

    /// Iterate and collect entries using the provided transform function.
    pub fn collect<T, F>(&self, mut transform: F) -> Vec<T>
    where
        F: FnMut(DateIndex, Timestamp, Height) -> Option<T>,
    {
        let total = self.end_di.to_usize().saturating_sub(self.start_di.to_usize()) + 1;
        let mut timestamps = self
            .computer
            .chain
            .timeindexes_to_timestamp
            .dateindex_extra
            .unwrap_first()
            .iter();
        let mut heights = self.computer.indexes.dateindex_to_first_height.iter();

        let mut entries = Vec::with_capacity(total / self.step + 1);
        let mut i = self.start_di.to_usize();

        while i <= self.end_di.to_usize() {
            let di = DateIndex::from(i);
            if let (Some(ts), Some(h)) = (timestamps.get(di), heights.get(di)) {
                if let Some(entry) = transform(di, ts, h) {
                    entries.push(entry);
                }
            }
            i += self.step;
        }

        entries
    }
}
