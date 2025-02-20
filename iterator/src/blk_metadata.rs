use std::path::Path;

use crate::path_to_modified_time;

#[derive(Debug, Clone, Copy)]
pub struct BlkMetadata {
    pub index: usize,
    pub modified_time: u64,
}

impl BlkMetadata {
    pub fn new(index: usize, path: &Path) -> Self {
        Self {
            index,
            modified_time: path_to_modified_time(path),
        }
    }
}
