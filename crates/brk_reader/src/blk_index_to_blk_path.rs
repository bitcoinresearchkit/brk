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
                .map(|entry| entry.unwrap().path())
                .filter(|path| {
                    let is_file = path.is_file();

                    if is_file {
                        let file_name = path.file_name().unwrap().to_str().unwrap();

                        file_name.starts_with(BLK) && file_name.ends_with(DOT_DAT)
                    } else {
                        false
                    }
                })
                .map(|path| {
                    let file_name = path.file_name().unwrap().to_str().unwrap();

                    let blk_index = file_name[BLK.len()..(file_name.len() - DOT_DAT.len())]
                        .parse::<u16>()
                        .unwrap();

                    (blk_index, path)
                })
                .collect::<BTreeMap<_, _>>(),
        )
    }
}
