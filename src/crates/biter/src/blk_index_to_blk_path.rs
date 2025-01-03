use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use derived_deref::{Deref, DerefMut};

const BLK: &str = "blk";
const DAT: &str = ".dat";

#[derive(Debug, Deref, DerefMut)]
pub struct BlkIndexToBlkPath(BTreeMap<usize, PathBuf>);

impl BlkIndexToBlkPath {
    pub fn scan(data_dir: &Path) -> Self {
        let blocks_dir = data_dir.join("blocks");

        Self(
            fs::read_dir(blocks_dir)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| {
                    let is_file = path.is_file();

                    if is_file {
                        let file_name = path.file_name().unwrap().to_str().unwrap();

                        file_name.starts_with(BLK) && file_name.ends_with(DAT)
                    } else {
                        false
                    }
                })
                .map(|path| {
                    let file_name = path.file_name().unwrap().to_str().unwrap();

                    let blk_index = file_name[BLK.len()..(file_name.len() - DAT.len())]
                        .parse::<usize>()
                        .unwrap();

                    (blk_index, path)
                })
                .collect::<BTreeMap<_, _>>(),
        )
    }
}
