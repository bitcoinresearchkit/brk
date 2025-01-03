use biter::bitcoin::BlockHash;
use color_eyre::eyre::eyre;
use derive_deref::{Deref, DerefMut};

use crate::structs::{Database, DatabaseTrait, Height, Version};

#[derive(Deref, DerefMut)]
pub struct BlockhashPrefixToHeight(Database);

impl BlockhashPrefixToHeight {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self(Database::import(
            "blockhash_suffix_to_height",
            Self::version(),
        )?))
    }

    pub fn insert(&mut self, blockhash: &BlockHash, height: Height) -> color_eyre::Result<()> {
        if let Some(_height) = self.fetch_update(blockhash[..8].into(), height.into(), height)? {
            // dbg!(height, Height::from(other), hash);
            return Err(eyre!("BlockhashSuffixToHeight: key collision"));
        }
        Ok(())
    }

    pub fn remove(&mut self, blockhash: &BlockHash) {
        self.0.remove((&blockhash[..]).into())
    }
}

impl DatabaseTrait for BlockhashPrefixToHeight {
    fn version() -> Version {
        Version::from(1)
    }
}
