use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::sync::Arc;
use std::{collections::HashMap, fs, io::BufReader, path::Path};

use bincode::decode_from_std_read;
use bincode::{Decode, Encode, config};
use brk_core::Result;
use parking_lot::RwLock;

use crate::PAGE_SIZE;

use super::Region;

#[derive(Debug)]
pub struct Layout {
    file: fs::File,
    id_to_index: HashMap<String, usize>,
    start_to_index: BTreeMap<u64, usize>,
    index_to_region: Vec<Option<Arc<RwLock<Region>>>>,
    /// key: start, value: gap
    start_to_hole: BTreeMap<u64, u64>,
}

impl Layout {
    pub fn open(path: &Path) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .truncate(false)
            .open(path)?;

        Ok(if file.metadata()?.len() != 0 {
            let config = config::standard();

            let mut reader = BufReader::new(&file);
            let serialized: SerializedRegions = decode_from_std_read(&mut reader, config)?;

            let mut id_to_index = HashMap::new();
            let mut start_to_index = BTreeMap::new();
            let mut index_to_region = vec![];

            serialized.0.into_iter().for_each(|(str, region)| {
                let index = index_to_region.len();
                id_to_index.insert(str, index);
                start_to_index.insert(region.start(), index);
                index_to_region.push(Some(Arc::new(RwLock::new(region))));
            });

            Self {
                file,
                id_to_index,
                start_to_index,
                index_to_region,
                start_to_hole: BTreeMap::new(),
            }
        } else {
            Self {
                file,
                id_to_index: HashMap::new(),
                index_to_region: Vec::new(),
                start_to_index: BTreeMap::new(),
                start_to_hole: BTreeMap::new(),
            }
        })
    }

    pub fn get_region_from_index(&self, index: usize) -> Option<Arc<RwLock<Region>>> {
        self.index_to_region.get(index).cloned().flatten()
    }

    pub fn get_region_index_from_id(&self, id: String) -> Option<usize> {
        self.id_to_index.get(&id).copied()
    }

    pub fn create_region_from_hole(&mut self, id: String) -> Option<usize> {
        let index = self.index_to_region.len();

        let start = self.find_smallest_adequate_hole(PAGE_SIZE)?;

        self.remove_or_compress_hole_to_right(start, PAGE_SIZE);

        self.id_to_index.insert(id, index);
        self.start_to_index.insert(start, index);

        self.index_to_region
            .push(Some(Arc::new(RwLock::new(Region::new(
                start, PAGE_SIZE, PAGE_SIZE,
            )))));

        Some(index)
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

    pub fn push_region(&mut self, id: String) -> (usize, Region) {
        let index = self.index_to_region.len();

        self.id_to_index.insert(id, index);

        let start = self
            .start_to_index
            .last_key_value()
            .map(|(_, index)| {
                let region = self
                    .index_to_region
                    .get(*index)
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .read();
                region.start() + region.reserved()
            })
            .unwrap_or_default();

        let region = Region::new(start, PAGE_SIZE, PAGE_SIZE);

        self.index_to_region
            .push(Some(Arc::new(RwLock::new(region.clone()))));

        (index, region)
    }

    pub fn remove_region(&mut self, index: usize) -> Option<Arc<RwLock<Region>>> {
        let region = self.index_to_region.get_mut(index).and_then(Option::take)?;

        self.id_to_index
            .remove(&self.find_id_from_index(index).unwrap().to_owned());
        self.start_to_index.remove(&region.read().start());

        let lock = region.read();
        let start = lock.start();
        let reserved = lock.reserved();

        if self
            .widen_hole_to_the_left_if_any(start + reserved, reserved)
            .is_none()
            && let Some((&hole_start, gap)) = self.start_to_hole.range(..start).next_back()
            && hole_start + *gap == start
        {
            self.widen_hole_to_the_right_if_any(hole_start, reserved);
        }

        drop(lock);

        Some(region)
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

    fn find_id_from_index(&self, index: usize) -> Option<&String> {
        Some(
            self.id_to_index
                .iter()
                .find(|(_, v)| **v == index)
                .unwrap()
                .0,
        )
    }
}

#[derive(Debug, Encode, Decode)]
struct SerializedRegions(HashMap<String, Region>);
