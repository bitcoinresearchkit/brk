use brk_types::StoredF32;

use crate::internal::{LazyPerBlock, Windows};

pub type Vecs = Windows<LazyPerBlock<StoredF32>>;
