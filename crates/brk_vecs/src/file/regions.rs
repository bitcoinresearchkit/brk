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

use super::{
    Identifier, PAGE_SIZE,
    region::{Region, SIZE_OF_REGION},
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

        fs::create_dir_all(&path)?;

        let id_to_index_file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .truncate(false)
            .open(path.join("id_to_index"))?;

        let mut id_to_index: HashMap<String, usize> = HashMap::new();

        if id_to_index_file.metadata()?.len() > 0 {
            let mut reader = BufReader::new(&id_to_index_file);
            id_to_index = decode_from_std_read(&mut reader, bincode::config::standard())?;
        }

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

    pub fn set_min_len(&mut self, len: u64) -> Result<()> {
        if self.index_to_region_mmap.len() < len as usize {
            self.index_to_region_file.set_len(len)?;
            self.index_to_region_mmap = unsafe { MmapMut::map_mut(&self.index_to_region_file)? };
        }
        Ok(())
    }

    pub fn create_region(
        &mut self,
        id: String,
        start: u64,
    ) -> Result<(usize, Arc<RwLock<Region>>)> {
        let index = self
            .index_to_region
            .iter()
            .enumerate()
            .find(|(_, opt)| opt.is_none())
            .map(|(index, _)| index)
            .unwrap_or_else(|| self.index_to_region.len());

        let region = Region::new(start, 0, PAGE_SIZE);

        self.set_min_len(((index + 1) * SIZE_OF_REGION) as u64)?;

        self.write_to_mmap(&region, index);

        let region_arc = Arc::new(RwLock::new(region));

        let region_opt = Some(region_arc.clone());
        if index < self.index_to_region.len() {
            self.index_to_region[index] = region_opt
        } else {
            self.index_to_region.push(region_opt);
        }

        if self.id_to_index.insert(id, index).is_some() {
            return Err(Error::Str("Already exists"));
        }
        self.flush_id_to_index()?;

        Ok((index, region_arc))
    }

    fn flush_id_to_index(&mut self) -> Result<()> {
        let mut writer = BufWriter::new(&mut self.id_to_index_file);
        encode_into_std_write(&self.id_to_index, &mut writer, bincode::config::standard())?;
        Ok(())
    }

    #[inline]
    pub fn get_region(&self, identifier: Identifier) -> Option<Arc<RwLock<Region>>> {
        match identifier {
            Identifier::Number(index) => self.get_region_from_index(index),
            Identifier::String(id) => self.get_region_from_id(&id),
        }
    }

    #[inline]
    pub fn get_region_from_index(&self, index: usize) -> Option<Arc<RwLock<Region>>> {
        self.index_to_region.get(index).cloned().flatten()
    }

    #[inline]
    pub fn get_region_from_id(&self, id: &str) -> Option<Arc<RwLock<Region>>> {
        self.get_region_index_from_id(id)
            .and_then(|index| self.get_region_from_index(index))
    }

    #[inline]
    pub fn get_region_index_from_id(&self, id: &str) -> Option<usize> {
        self.id_to_index.get(id).copied()
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

    #[inline]
    pub fn index_to_region(&self) -> &[Option<Arc<RwLock<Region>>>] {
        &self.index_to_region
    }

    #[inline]
    pub fn id_to_index(&self) -> &HashMap<String, usize> {
        &self.id_to_index
    }

    #[inline]
    pub fn identifier_to_index(&self, identifier: Identifier) -> Option<usize> {
        match identifier {
            Identifier::Number(index) => Some(index),
            Identifier::String(id) => self.get_region_index_from_id(&id),
        }
    }

    pub fn remove_region(&mut self, identifier: Identifier) -> Result<Option<Arc<RwLock<Region>>>> {
        match identifier {
            Identifier::Number(index) => self.remove_region_from_index(index),
            Identifier::String(id) => self.remove_region_from_id(&id),
        }
    }

    pub fn remove_region_from_id(&mut self, id: &str) -> Result<Option<Arc<RwLock<Region>>>> {
        let Some(index) = self.get_region_index_from_id(id) else {
            return Ok(None);
        };
        self.remove_region_from_index(index)
    }

    pub fn remove_region_from_index(
        &mut self,
        index: usize,
    ) -> Result<Option<Arc<RwLock<Region>>>> {
        let Some(region) = self.index_to_region.get_mut(index).and_then(Option::take) else {
            return Ok(None);
        };

        self.id_to_index
            .remove(&self.find_id_from_index(index).unwrap().to_owned());

        self.flush_id_to_index()?;

        Ok(Some(region))
    }

    pub fn write_to_mmap(&self, region: &Region, index: usize) {
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
