use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{path_to_modified_time, Height};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct BlkRecap {
    pub max_height: Height,
    pub modified_time: u64,
}

impl BlkRecap {
    pub fn has_different_modified_time(&self, blk_path: &Path) -> bool {
        if self.modified_time != path_to_modified_time(blk_path) {
            dbg!(self.modified_time, path_to_modified_time(blk_path));
        }
        self.modified_time != path_to_modified_time(blk_path)
    }
}
