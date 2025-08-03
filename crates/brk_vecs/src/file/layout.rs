use std::collections::BTreeMap;

use crate::{Error, Result};

use super::{Region, Regions};

#[derive(Debug)]
pub struct Layout {
    start_to_index: BTreeMap<u64, usize>,
    start_to_hole: BTreeMap<u64, u64>,
    start_to_reserved: BTreeMap<u64, u64>,
}

impl From<&Regions> for Layout {
    fn from(value: &Regions) -> Self {
        let mut start_to_index = BTreeMap::new();

        let index_to_region = value.index_to_region();

        value
            .index_to_region()
            .iter()
            .enumerate()
            .flat_map(|(index, opt)| opt.as_ref().map(|region| (index, region)))
            .for_each(|(index, region)| {
                let region = region.read();
                let start = region.start();
                start_to_index.insert(start, index);
            });

        let mut start_to_hole = BTreeMap::new();

        let mut prev_end = 0;

        start_to_index.iter().for_each(|(&start, &index)| {
            if prev_end != start {
                start_to_hole.insert(prev_end, start - prev_end);
            }
            let reserved = index_to_region[index].as_ref().unwrap().read().reserved();
            prev_end = start + reserved;
        });

        Self {
            start_to_index,
            start_to_hole,
            start_to_reserved: BTreeMap::default(),
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

    pub fn len(&self, regions: &Regions) -> u64 {
        let mut len = 0;
        let mut start = 0;
        if let Some((start_reserved, reserved)) = self.get_last_reserved() {
            start = start_reserved;
            len = start + reserved;
        }
        if let Some((hole_start, gap)) = self.get_last_hole()
            && hole_start > start
        {
            start = hole_start;
            len = start + gap;
        }
        if let Some((region_start, region_index)) = self.get_last_region()
            && region_start > start
        {
            len = region_start
                + regions
                    .get_region_from_index(region_index)
                    .unwrap()
                    .read()
                    .reserved();
        }
        len
    }

    pub fn get_last_region_index(&self) -> Option<usize> {
        self.get_last_region().map(|(_, index)| index)
    }

    pub fn get_last_region(&self) -> Option<(u64, usize)> {
        self.start_to_index
            .last_key_value()
            .map(|(start, index)| (*start, *index))
    }

    fn get_last_hole(&self) -> Option<(u64, u64)> {
        self.start_to_hole
            .last_key_value()
            .map(|(start, gap)| (*start, *gap))
    }

    fn get_last_reserved(&self) -> Option<(u64, u64)> {
        self.start_to_reserved
            .last_key_value()
            .map(|(start, reserved)| (*start, *reserved))
    }

    pub fn is_last_anything(&self, index: usize) -> bool {
        if let Some((last_start, last_index)) = self.get_last_region()
            && last_index == index
            && self
                .get_last_hole()
                .is_none_or(|(hole_start, _)| last_start > hole_start)
            && self
                .get_last_reserved()
                .is_none_or(|(reserved_start, _)| last_start > reserved_start)
        {
            true
        } else {
            false
        }
    }

    pub fn insert_region(&mut self, start: u64, index: usize) {
        assert!(self.start_to_index.insert(start, index).is_none())
        // TODO: Other checks related to holes and reserved ?
    }

    pub fn move_region(&mut self, new_start: u64, index: usize, region: &Region) -> Result<()> {
        self.remove_region(index, region)?;
        self.insert_region(new_start, index);
        Ok(())
    }

    pub fn remove_region(&mut self, index: usize, region: &Region) -> Result<()> {
        // info!("Remove region {index}");

        let start = region.start();
        let mut reserved = region.reserved();

        let removed = self.start_to_index.remove(&start);

        if removed.is_none_or(|index_| index != index_) {
            dbg!((index, removed));
            return Err(Error::Str(
                "Something went wrong, indexes of removed region should be the same",
            ));
        }

        reserved += self
            .start_to_hole
            .remove(&(start + reserved))
            .unwrap_or_default();

        if let Some((&hole_start, gap)) = self.start_to_hole.range_mut(..start).next_back()
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

    pub fn remove_or_compress_hole(&mut self, start: u64, compress_by: u64) {
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

    pub fn reserve(&mut self, start: u64, reserved: u64) {
        if self.start_to_reserved.insert(start, reserved).is_some() {
            unreachable!();
        }
    }

    pub fn reserved(&mut self, start: u64) -> Option<u64> {
        self.start_to_reserved.remove(&start)
    }
}
