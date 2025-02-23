use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{Height, path_to_modified_time};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct BlkRecap {
    pub max_height: Height,
    pub modified_time: u64,
}

impl BlkRecap {
    pub fn has_different_modified_time(&self, blk_path: &Path) -> bool {
        self.modified_time != path_to_modified_time(blk_path)
    }
}
