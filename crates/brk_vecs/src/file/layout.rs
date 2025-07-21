use std::fs::OpenOptions;
use std::sync::Arc;
use std::{collections::HashMap, fs, io::BufReader, path::Path};

use bincode::decode_from_std_read;
use bincode::{Decode, Encode, config};
use brk_core::Result;
use parking_lot::{RwLock, RwLockReadGuard};

use crate::PAGE_SIZE;

use super::Region;

#[derive(Debug)]
pub struct Layout {
    file: fs::File,
    pub id_to_index: HashMap<String, usize>,
    pub index_to_region: Vec<Arc<RwLock<Region>>>,
    // holes
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
            let mut index_to_region = vec![];

            serialized.0.into_iter().for_each(|(str, region)| {
                id_to_index.insert(str, index_to_region.len());
                index_to_region.push(Arc::new(RwLock::new(region)));
            });

            Self {
                file,
                id_to_index,
                index_to_region,
            }
        } else {
            Self {
                file,
                id_to_index: HashMap::new(),
                index_to_region: Vec::new(),
            }
        })
    }

    pub fn get_or_create_region_from_id(&mut self, id: String) -> Result<usize> {
        if let Some(v) = self.id_to_index.get(&id) {
            return Ok(*v);
        }
        let index = self.create_region()?;
        self.id_to_index.insert(id, index);
        Ok(index)
    }

    fn create_region(&mut self) -> Result<usize> {
        let index = self.index_to_region.len();

        let length = PAGE_SIZE;

        Ok(0)
    }

    pub fn get(&self, region: usize) -> Option<RwLockReadGuard<'_, Region>> {
        self.index_to_region.get(region).map(|r| r.read())
    }
}

#[derive(Debug, Encode, Decode)]
struct SerializedRegions(HashMap<String, Region>);
