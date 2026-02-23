use brk_computer::Computer;
use brk_types::{Day1, Height, Timestamp};
use vecdb::{ReadableVec, Ro, VecIndex};

/// Helper for iterating over day1 ranges with sampling.
pub struct Day1Iter<'a> {
    computer: &'a Computer<Ro>,
    start_di: Day1,
    end_di: Day1,
    step: usize,
}

impl<'a> Day1Iter<'a> {
    pub fn new(computer: &'a Computer<Ro>, start_height: usize, end_height: usize) -> Self {
        let start_di = computer
            .indexes
            .height
            .day1
            .collect_one(Height::from(start_height))
            .unwrap_or_default();
        let end_di = computer
            .indexes
            .height
            .day1
            .collect_one(Height::from(end_height))
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
        F: FnMut(Day1, Timestamp, Height) -> Option<T>,
    {
        let total = self
            .end_di
            .to_usize()
            .saturating_sub(self.start_di.to_usize())
            + 1;
        let timestamps = &self.computer.blocks.time.timestamp.day1;
        let heights = &self.computer.indexes.day1.first_height;

        let mut entries = Vec::with_capacity(total / self.step + 1);
        let mut i = self.start_di.to_usize();

        while i <= self.end_di.to_usize() {
            let di = Day1::from(i);
            if let (Some(ts), Some(h)) = (timestamps.collect_one(di), heights.collect_one(di))
                && let Some(entry) = transform(di, ts, h)
            {
                entries.push(entry);
            }
            i += self.step;
        }

        entries
    }
}
