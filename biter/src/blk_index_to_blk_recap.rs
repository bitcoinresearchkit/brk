use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use derived_deref::{Deref, DerefMut};

use crate::{blk_recap::BlkRecap, BlkMetadataAndBlock};

#[derive(Deref, DerefMut, Debug)]
pub struct BlkIndexToBlkRecap {
    path: String,
    #[target]
    tree: BTreeMap<usize, BlkRecap>,
}

impl BlkIndexToBlkRecap {
    pub fn import(blocks_dir: &BTreeMap<usize, PathBuf>, export_dir: &str) -> Self {
        let path = format!("{export_dir}/blk_index_to_blk_recap.json");

        let tree = {
            fs::create_dir_all(export_dir).unwrap();

            if let Ok(file) = File::open(&path) {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader).unwrap_or_default()
            } else {
                BTreeMap::default()
            }
        };

        let mut this = Self { path, tree };

        this.clean_outdated(blocks_dir);

        this
    }

    pub fn clean_outdated(&mut self, blocks_dir: &BTreeMap<usize, PathBuf>) {
        blocks_dir.iter().for_each(|(blk_index, blk_path)| {
            if let Some(blk_recap) = self.get(blk_index) {
                if blk_recap.has_different_modified_time(blk_path) {
                    self.remove(blk_index);
                }
            }
        });
    }

    pub fn get_start_recap(&self, start: Option<usize>) -> Option<(usize, BlkRecap)> {
        if let Some(start) = start {
            let (last_key, last_value) = self.last_key_value()?;

            if last_value.height() < start {
                return Some((*last_key, *last_value));
            } else if let Some((blk_index, _)) = self
                .iter()
                .find(|(_, blk_recap)| blk_recap.is_younger_than(start))
            {
                if *blk_index != 0 {
                    let blk_index = *blk_index - 1;
                    return Some((blk_index, *self.get(&blk_index).unwrap()));
                }
            }
        }

        None
    }

    pub fn update(&mut self, blk_metadata_and_block: &BlkMetadataAndBlock, height: usize) {
        let blk_index = blk_metadata_and_block.blk_metadata.index;

        if let Some(last_entry) = self.last_entry() {
            // if last_entry.get().is_older_than(height) {
            match last_entry.key().cmp(&blk_index) {
                Ordering::Greater => {
                    last_entry.remove_entry();
                }
                Ordering::Less => {
                    self.insert(blk_index, BlkRecap::from(height, blk_metadata_and_block));
                }
                Ordering::Equal => {}
            };
            // }
        } else {
            if blk_index != 0 || height != 0 {
                // dbg!(blk_index, height);
                unreachable!();
            }

            self.insert(blk_index, BlkRecap::first(blk_metadata_and_block));
        }
    }

    pub fn export(&self) {
        let file = File::create(&self.path).unwrap_or_else(|_| {
            dbg!(&self.path);
            panic!("No such file or directory")
        });

        serde_json::to_writer_pretty(&mut BufWriter::new(file), &self.tree).unwrap();
    }
}
