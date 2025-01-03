use derive_deref::{Deref, DerefMut};
use fjall::Slice;

use crate::structs::{Database, DatabaseTrait, Height, Txindex, Version};

#[derive(Deref, DerefMut)]
pub struct HeightToTxindex(Database);

#[derive(Debug, PartialEq, Eq)]
pub enum HeightToTxindexPosition {
    First,
    Last,
}

impl HeightToTxindex {
    pub fn import(position: HeightToTxindexPosition) -> color_eyre::Result<Self> {
        Ok(Self(Database::import(
            &format!(
                "height_to_{}_txindex",
                format!("{position:?}").to_lowercase()
            ),
            Self::version(),
        )?))
    }

    pub fn insert(&mut self, height: Height, txindex: Txindex) {
        self.0.insert(height.into(), txindex.into(), height)
    }

    pub fn get(&self, height: Height) -> fjall::Result<Option<Txindex>> {
        self.0
            .get(Slice::from(height))
            .map(|opt| opt.map(|slice| slice.into()))
    }

    pub fn remove(&mut self, height: Height) {
        self.0.remove(Slice::from(height))
    }
}

impl DatabaseTrait for HeightToTxindex {
    fn version() -> Version {
        Version::from(1)
    }
}
