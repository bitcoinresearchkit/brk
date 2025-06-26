use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use crate::{BlkIndexToBlkPath, Height, blk_recap::BlkRecap};

#[derive(Debug)]
pub struct BlkIndexToBlkRecap {
    pub path: PathBuf,
    pub tree: BTreeMap<u16, BlkRecap>,
}

impl BlkIndexToBlkRecap {
    pub fn import(
        outputs_dir: &Path,
        blk_index_to_blk_path: &BlkIndexToBlkPath,
        start: Option<Height>,
    ) -> (Self, u16) {
        let path = outputs_dir.join("blk_index_to_blk_recap.json");

        let tree = {
            if let Ok(file) = File::open(&path) {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader).unwrap_or_default()
            } else {
                BTreeMap::default()
            }
        };

        let mut slf = Self { path, tree };

        let min_removed = slf.clean_outdated(blk_index_to_blk_path);

        let blk_index = slf.get_start_recap(min_removed, start);

        (slf, blk_index)
    }

    fn clean_outdated(&mut self, blk_index_to_blk_path: &BlkIndexToBlkPath) -> Option<u16> {
        let mut min_removed_blk_index: Option<u16> = None;

        let mut unprocessed_keys = self.tree.keys().copied().collect::<BTreeSet<_>>();

        blk_index_to_blk_path
            .iter()
            .for_each(|(blk_index, blk_path)| {
                unprocessed_keys.remove(blk_index);
                if let Some(blk_recap) = self.tree.get(blk_index) {
                    if blk_recap.has_different_modified_time(blk_path) {
                        self.tree.remove(blk_index).unwrap();
                        if min_removed_blk_index.is_none_or(|_blk_index| *blk_index < _blk_index) {
                            min_removed_blk_index.replace(*blk_index);
                        }
                    }
                }
            });

        unprocessed_keys.into_iter().for_each(|blk_index| {
            self.tree.remove(&blk_index).unwrap();
            if min_removed_blk_index.is_none_or(|_blk_index| blk_index < _blk_index) {
                min_removed_blk_index.replace(blk_index);
            }
        });

        min_removed_blk_index
    }

    pub fn get_start_recap(&mut self, min_removed: Option<u16>, start: Option<Height>) -> u16 {
        if start.is_none() {
            return 0;
        }

        let height = start.unwrap();

        let mut start = None;

        if let Some(found) = self
            .tree
            .iter()
            .find(|(_, recap)| recap.max_height >= height)
        {
            start = Some(*found.0);
        }

        if let Some(min_removed) = min_removed {
            if start.is_none_or(|start| start > min_removed) {
                start = Some(min_removed);
            }
        }

        // Should only be none if asking for a too high start
        start.unwrap_or_else(|| self.tree.last_key_value().map_or(0, |(i, _)| *i))
    }

    pub fn export(&self) {
        let file = File::create(&self.path).unwrap_or_else(|e| {
            dbg!(e);
            dbg!(&self.path);
            panic!("Cannot write file");
        });

        serde_json::to_writer(&mut BufWriter::new(file), &self.tree).unwrap();
    }
}
