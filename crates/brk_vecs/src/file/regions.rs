use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{BufReader, BufWriter},
    path::Path,
    sync::Arc,
};

use bincode::{decode_from_std_read, encode_into_std_write};
use brk_core::{Error, Result};
use memmap2::MmapMut;
use parking_lot::RwLock;
use zerocopy::{FromBytes, IntoBytes};

use crate::{
    PAGE_SIZE,
    file::region::{Region, SIZE_OF_REGION},
};

#[derive(Debug)]
pub struct Regions {
    id_to_index: HashMap<String, usize>,
    id_to_index_file: fs::File,
    index_to_region: Vec<Option<Arc<RwLock<Region>>>>,
    index_to_region_file: fs::File,
    index_to_region_mmap: MmapMut,
}

impl Regions {
    pub fn open(path: &Path) -> Result<Self> {
        let path = path.join("regions");

        let id_to_index_file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .truncate(false)
            .open(path.join("id_to_index"))?;

        let mut reader = BufReader::new(&id_to_index_file);
        let id_to_index: HashMap<String, usize> =
            decode_from_std_read(&mut reader, bincode::config::standard())?;

        let index_to_region_file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .truncate(false)
            .open(path.join("index_to_region"))?;

        let index_to_region_mmap = unsafe { MmapMut::map_mut(&index_to_region_file)? };

        let mut index_to_region: Vec<Option<Arc<RwLock<Region>>>> = vec![];

        id_to_index
            .iter()
            .try_for_each(|(_, &index)| -> Result<()> {
                let start = index * SIZE_OF_REGION;
                let end = start + SIZE_OF_REGION;
                let region = Region::read_from_bytes(&index_to_region_mmap[start..end])?;
                if index_to_region.len() < index + 1 {
                    index_to_region.resize_with(index + 1, Default::default);
                }
                index_to_region
                    .get_mut(index)
                    .unwrap()
                    .replace(Arc::new(RwLock::new(region)));
                Ok(())
            })?;

        // TODO: Removes Nones from vec if needed, update map accordingly and save them

        Ok(Self {
            id_to_index,
            id_to_index_file,
            index_to_region,
            index_to_region_file,
            index_to_region_mmap,
        })
    }

    pub fn create_region(&mut self, id: String, start: u64) -> Result<usize> {
        let index = self
            .index_to_region
            .iter()
            .enumerate()
            .find(|(_, opt)| opt.is_none())
            .map(|(index, _)| index)
            .unwrap_or_else(|| self.index_to_region.len());

        let region = Region::new(start, PAGE_SIZE, PAGE_SIZE);

        self.index_to_region
            .push(Some(Arc::new(RwLock::new(region.clone()))));

        let end = index * SIZE_OF_REGION + SIZE_OF_REGION;
        if self.index_to_region_mmap.len() < end {
            self.index_to_region_file.set_len(end as u64);
            self.index_to_region_mmap = unsafe { MmapMut::map_mut(&self.index_to_region_file)? };
        }

        self.write_to_mmap(&region, index);

        if self.id_to_index.insert(id, index).is_some() {
            return Err(Error::Str("Already exists"));
        }
        self.flush_id_to_index()?;

        Ok(index)
    }

    pub fn remove_region(&mut self, index: usize) -> Result<Option<Arc<RwLock<Region>>>> {
        let Some(region) = self.index_to_region.get_mut(index).and_then(Option::take) else {
            return Ok(None);
        };

        self.id_to_index
            .remove(&self.find_id_from_index(index).unwrap().to_owned());

        self.flush_id_to_index()?;

        Ok(Some(region))
    }

    fn flush_id_to_index(&mut self) -> Result<()> {
        let mut writer = BufWriter::new(&mut self.id_to_index_file);
        encode_into_std_write(&self.id_to_index, &mut writer, bincode::config::standard())?;
        Ok(())
    }

    pub fn get_region_from_index(&self, index: usize) -> Option<Arc<RwLock<Region>>> {
        self.index_to_region.get(index).cloned().flatten()
    }

    pub fn get_region_index_from_id(&self, id: String) -> Option<usize> {
        self.id_to_index.get(&id).copied()
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

    pub fn as_array(&self) -> &[Option<Arc<RwLock<Region>>>] {
        &self.index_to_region
    }

    fn write_to_mmap(&self, region: &Region, index: usize) {
        let start = index * SIZE_OF_REGION;
        let end = start + SIZE_OF_REGION;
        let mmap = &self.index_to_region_mmap;
        if end > mmap.len() {
            unreachable!("Trying to write beyond mmap")
        }
        let slice = unsafe { std::slice::from_raw_parts_mut(mmap.as_ptr() as *mut u8, mmap.len()) };
        slice[start..end].copy_from_slice(region.as_bytes());
    }
}
