use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{Cursor, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use brk_core::{Error, Result};
use memmap2::MmapMut;
use parking_lot::{RwLock, RwLockWriteGuard};
use zerocopy::{FromBytes, IntoBytes};

use super::{
    Identifier, PAGE_SIZE,
    region::{Region, SIZE_OF_REGION},
};

#[derive(Debug)]
pub struct Regions {
    id_to_index: HashMap<String, usize>,
    id_to_index_path: PathBuf,
    index_to_region: Vec<Option<Arc<RwLock<Region>>>>,
    index_to_region_file: fs::File,
    index_to_region_mmap: MmapMut,
}

impl Regions {
    pub fn open(path: &Path) -> Result<Self> {
        let path = path.join("regions");

        fs::create_dir_all(&path)?;

        let id_to_index_path = path.join("id_to_index");

        let id_to_index: HashMap<String, usize> =
            Self::deserialize(&fs::read(&id_to_index_path).unwrap_or_default()).unwrap_or_default();

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
            id_to_index_path,
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

        let region_lock = RwLock::new(region);

        self.write_to_mmap(&region_lock.write(), index);

        let region_arc = Arc::new(region_lock);

        let region_opt = Some(region_arc.clone());
        if index < self.index_to_region.len() {
            self.index_to_region[index] = region_opt
        } else {
            self.index_to_region.push(region_opt);
            self.index_to_region_mmap.flush()?;
        }

        if self.id_to_index.insert(id, index).is_some() {
            return Err(Error::Str("Already exists"));
        }
        self.flush_id_to_index()?;

        Ok((index, region_arc))
    }

    fn flush_id_to_index(&mut self) -> Result<()> {
        fs::write(&self.id_to_index_path, Self::serialize(&self.id_to_index))?;
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

        self.index_to_region_mmap.flush()?;

        self.id_to_index
            .remove(&self.find_id_from_index(index).unwrap().to_owned());

        self.flush_id_to_index()?;

        Ok(Some(region))
    }

    pub fn write_to_mmap(&self, region: &RwLockWriteGuard<Region>, index: usize) {
        let mmap = &self.index_to_region_mmap;
        let start = index * SIZE_OF_REGION;
        let end = start + SIZE_OF_REGION;

        if end > mmap.len() {
            unreachable!("Trying to write beyond mmap")
        }

        let slice = unsafe { std::slice::from_raw_parts_mut(mmap.as_ptr() as *mut u8, mmap.len()) };

        slice[start..end].copy_from_slice(region.as_bytes());
    }

    pub fn flush(&self) -> Result<()> {
        self.index_to_region_mmap.flush().map_err(|e| e.into())
    }

    fn serialize(map: &HashMap<String, usize>) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&map.len().to_ne_bytes());

        for (key, value) in map {
            buffer.extend_from_slice(key.len().as_bytes());
            buffer.extend_from_slice(key.as_bytes());
            buffer.extend_from_slice(value.as_bytes());
        }

        buffer
    }

    fn deserialize(data: &[u8]) -> Result<HashMap<String, usize>> {
        let mut cursor = Cursor::new(data);
        let mut buffer = [0u8; 8];

        cursor
            .read_exact(&mut buffer)
            .map_err(|_| Error::Str("Failed to read entry count"))?;
        let entry_count = usize::read_from_bytes(&buffer)?;

        let mut map = HashMap::with_capacity(entry_count);

        for _ in 0..entry_count {
            cursor
                .read_exact(&mut buffer)
                .map_err(|_| Error::Str("Failed to read key length"))?;
            let key_len = usize::read_from_bytes(&buffer)?;

            let mut key_bytes = vec![0u8; key_len];
            cursor.read_exact(&mut key_bytes)?;
            let key =
                String::from_utf8(key_bytes).map_err(|_| Error::Str("Invalid UTF-8 in key"))?;

            cursor.read_exact(&mut buffer)?;
            let value = usize::read_from_bytes(&buffer)?;

            map.insert(key, value);
        }

        Ok(map)
    }
}
