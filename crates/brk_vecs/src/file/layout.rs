use std::collections::BTreeMap;

use brk_core::Error;
use brk_core::Result;

use super::{PAGE_SIZE, Region, Regions};

#[derive(Debug)]
pub struct Layout {
    start_to_index: BTreeMap<u64, usize>,
    /// key: start, value: gap
    start_to_hole: BTreeMap<u64, u64>,
}

impl From<&Regions> for Layout {
    fn from(value: &Regions) -> Self {
        let mut start_to_index = BTreeMap::new();
        let mut start_to_hole = BTreeMap::new();

        let mut prev_end = 0;

        value
            .as_array()
            .into_iter()
            .enumerate()
            .flat_map(|(index, opt)| opt.as_ref().map(|region| (index, region)))
            .for_each(|(index, region)| {
                let region = region.read();
                let start = region.start();
                start_to_index.insert(start, index);
                if prev_end != start {
                    start_to_hole.insert(prev_end, start - prev_end);
                }
                let reserved = region.reserved();
                prev_end = start + reserved;
            });

        Self {
            start_to_index,
            start_to_hole,
        }
    }
}

impl Layout {
    pub fn get_last_region(&self) -> Option<usize> {
        self.start_to_index
            .last_key_value()
            .map(|(_, index)| *index)
    }

    pub fn find_smallest_adequate_hole(&self, reserved: u64) -> Option<u64> {
        self.start_to_hole
            .iter()
            .filter(|(_, gap)| **gap >= reserved)
            .map(|(start, gap)| (gap, start))
            .collect::<BTreeMap<_, _>>()
            .pop_first()
            .map(|(_, s)| *s)
    }

    pub fn insert_region(&mut self, start: u64, index: usize) {
        assert!(self.start_to_index.insert(start, index).is_none())
    }

    pub fn remove_region(&mut self, index: usize, region: &Region) -> Result<()> {
        let start = region.start();
        let reserved = region.reserved();

        if self
            .start_to_index
            .remove(&start)
            .is_none_or(|index_| index != index_)
        {
            return Err(Error::Str(
                "Something went wrong, indexes of removed region should be the same",
            ));
        }

        if self
            .widen_hole_to_the_left_if_any(start + reserved, reserved)
            .is_none()
            && let Some((&hole_start, gap)) = self.start_to_hole.range(..start).next_back()
            && hole_start + *gap == start
        {
            self.widen_hole_to_the_right_if_any(hole_start, reserved);
        }

        if self
            .start_to_index
            .keys()
            .last()
            .is_none_or(|&region_start| {
                self.start_to_hole
                    .keys()
                    .last()
                    .is_some_and(|&hole_start| hole_start > region_start)
            })
        {
            self.start_to_hole.pop_last();
        }

        Ok(())
    }

    pub fn get_hole(&self, start: u64) -> Option<u64> {
        self.start_to_hole.get(&start).copied()
    }

    pub fn remove_or_compress_hole_to_right(&mut self, start: u64, compress_by: u64) {
        if let Some(gap) = self.start_to_hole.remove(&start)
            && gap != compress_by
        {
            if gap > compress_by {
                self.start_to_hole
                    .insert(start + compress_by, gap - compress_by);
            } else {
                panic!("Hole too small");
            }
        }
    }

    fn widen_hole_to_the_left_if_any(&mut self, start: u64, widen_by: u64) -> Option<u64> {
        assert!(start % PAGE_SIZE == 0);

        if widen_by > start {
            panic!("Hole too small")
        }

        let gap = self.start_to_hole.remove(&start)?;
        assert!(widen_by % PAGE_SIZE == 0);
        let start = start - widen_by;
        let gap = gap + widen_by;

        if let Some((&prev_start, prev_gap)) = self.start_to_hole.range_mut(..start).next_back()
            && prev_start + *prev_gap == start
        {
            *prev_gap += gap;
        } else {
            assert!(self.start_to_hole.insert(start, gap).is_none());
        }

        Some(start)
    }

    fn widen_hole_to_the_right_if_any(&mut self, start: u64, widen_by: u64) -> Option<u64> {
        assert!(start % PAGE_SIZE == 0);

        let gap = self.start_to_hole.get_mut(&start)?;
        assert!(widen_by % PAGE_SIZE == 0);
        *gap += widen_by;

        let next_hole_start = start + *gap;
        if let Some(next_gap) = self.start_to_hole.remove(&next_hole_start) {
            *self.start_to_hole.get_mut(&start).unwrap() += next_gap;
        }

        Some(start)
    }
}
