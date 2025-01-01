use biter::bitcoin::{hashes::Hash, BlockHash};
use derive_deref::{Deref, DerefMut};
use fjall::Slice;

use crate::structs::{Database, DatabaseTrait, Height, Version};

#[derive(Deref, DerefMut)]
pub struct HeightToBlockhash(Database);

impl HeightToBlockhash {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self(Database::import(
            "height_to_blockhash",
            Self::version(),
        )?))
    }

    pub fn insert(&mut self, height: Height, blockhash: &BlockHash) {
        self.0.insert(height.into(), blockhash[..].into(), height)
    }

    pub fn get(&self, height: Height) -> fjall::Result<Option<BlockHash>> {
        self.0
            .get(Slice::from(height))
            .map(|opt| opt.map(|slice| BlockHash::from_slice(&slice).unwrap()))
    }

    pub fn remove(&mut self, height: Height) {
        self.0.remove(Slice::from(height))
    }
}

impl DatabaseTrait for HeightToBlockhash {
    fn version() -> Version {
        Version::from(1)
    }
}
