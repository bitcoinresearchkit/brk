use std::{
    collections::HashMap,
    fs,
    sync::{Arc, RwLock},
};

use crate::file::region::Region;

#[derive(Debug)]
pub struct Regions {
    file: fs::File,
    id_to_index: HashMap<String, usize>,
    index_to_region: Vec<Option<Arc<RwLock<Region>>>>,
}
