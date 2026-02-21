use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use derive_more::{Deref, DerefMut};

const BLK: &str = "blk";
const DOT_DAT: &str = ".dat";

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct BlkIndexToBlkPath(BTreeMap<u16, PathBuf>);

impl BlkIndexToBlkPath {
    pub fn scan(blocks_dir: &Path) -> Self {
        Self(
            fs::read_dir(blocks_dir)
                .unwrap()
                .filter_map(|entry| {
                    let path = entry.unwrap().path();
                    let file_name = path.file_name()?.to_str()?;

                    let index_str = file_name.strip_prefix(BLK)?.strip_suffix(DOT_DAT)?;
                    let blk_index = index_str.parse::<u16>().ok()?;

                    path.is_file().then_some((blk_index, path))
                })
                .collect(),
        )
    }
}
