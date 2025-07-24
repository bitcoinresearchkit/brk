use std::collections::BTreeMap;

use brk_core::Error;
use brk_core::Result;

use super::{Region, Regions};

#[derive(Debug)]
pub struct Layout {
    start_to_index: BTreeMap<u64, usize>,
    start_to_hole: BTreeMap<u64, u64>,
}

impl From<&Regions> for Layout {
    fn from(value: &Regions) -> Self {
        let mut start_to_index = BTreeMap::new();
        let mut start_to_hole = BTreeMap::new();

        let mut prev_end = 0;

        value
            .index_to_region()
            .iter()
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
    pub fn start_to_index(&self) -> &BTreeMap<u64, usize> {
        &self.start_to_index
    }

    pub fn start_to_hole(&self) -> &BTreeMap<u64, u64> {
        &self.start_to_hole
    }

    pub fn get_last_region(&self) -> Option<(u64, usize)> {
        self.start_to_index
            .last_key_value()
            .map(|(start, index)| (*start, *index))
    }

    pub fn get_last_region_index(&self) -> Option<usize> {
        self.get_last_region().map(|(_, index)| index)
    }

    pub fn is_last_region(&self, index: usize) -> bool {
        let last = self.get_last_region();
        let is_last = last.is_some_and(|(_, other_index)| index == other_index);
        if is_last {
            debug_assert!(self.start_to_hole.range(last.unwrap().0..).next().is_none());
        }
        is_last
    }

    pub fn insert_region(&mut self, start: u64, index: usize) {
        debug_assert!(self.start_to_index.insert(start, index).is_none())
        // TODO: Other checks related to holes ?
    }

    pub fn move_region(&mut self, new_start: u64, index: usize, region: &Region) -> Result<()> {
        self.remove_region(index, region)?;
        self.insert_region(new_start, index);
        Ok(())
    }

    pub fn remove_region(&mut self, index: usize, region: &Region) -> Result<()> {
        let start = region.start();
        let mut reserved = region.reserved();

        if self
            .start_to_index
            .remove(&start)
            .is_none_or(|index_| index != index_)
        {
            return Err(Error::Str(
                "Something went wrong, indexes of removed region should be the same",
            ));
        }

        reserved += self
            .start_to_hole
            .remove(&(start + reserved))
            .unwrap_or_default();

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
        } else if let Some((&hole_start, gap)) = self.start_to_hole.range_mut(..start).next_back()
            && hole_start + *gap == start
        {
            *gap += reserved;
        } else {
            self.start_to_hole.insert(start, reserved);
        }

        Ok(())
    }

    pub fn get_hole(&self, start: u64) -> Option<u64> {
        self.start_to_hole.get(&start).copied()
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
}
