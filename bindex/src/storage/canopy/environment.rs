use std::path::Path;

use canopydb::{EnvOptions, Environment as CanopyEnvironment};
use derive_deref::{Deref, DerefMut};

#[derive(Debug, Deref, DerefMut)]
pub struct Environment(CanopyEnvironment);

impl Environment {
    pub fn new(path: &Path) -> color_eyre::Result<Self> {
        let mut options = EnvOptions::new(path);
        // options.use_mmap = true;
        options.disable_fsync = true;
        options.wal_new_file_on_checkpoint = false;
        options.wal_background_sync_interval = None;
        options.wal_write_batch_memory_limit = usize::MAX;

        Ok(Self(CanopyEnvironment::with_options(options)?))
    }
}
