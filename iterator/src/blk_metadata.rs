use std::path::Path;

use crate::path_to_modified_time;

#[derive(Debug, Clone, Copy)]
pub struct BlkMetadata {
    pub index: u16,
    pub modified_time: u64,
}

impl BlkMetadata {
    pub fn new(index: u16, path: &Path) -> Self {
        Self {
            index,
            modified_time: path_to_modified_time(path),
        }
    }
}
