use std::path::PathBuf;

use crate::path_to_modified_time;

#[derive(Clone, Copy)]
pub struct BlkMetadata {
    pub index: usize,
    pub modified_time: u64,
}

impl BlkMetadata {
    pub fn new(index: usize, path: &PathBuf) -> Self {
        Self {
            index,
            modified_time: path_to_modified_time(path),
        }
    }
}
