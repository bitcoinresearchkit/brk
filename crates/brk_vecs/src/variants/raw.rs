use std::sync::Arc;

use crate::File;

pub struct RawVec {
    region: usize,
    file: Arc<File>,
}
