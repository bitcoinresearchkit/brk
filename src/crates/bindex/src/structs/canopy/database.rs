use std::time::Duration;

use canopydb::{Database as CanopyDatabase, DbOptions};
use derive_deref::{Deref, DerefMut};

use super::Environment;

#[derive(Debug, Deref, DerefMut)]
pub struct Database(CanopyDatabase);

impl Database {
    pub fn new(environment: &Environment, name: &str) -> color_eyre::Result<Self> {
        let mut options = DbOptions::default();
        options.use_wal = false;
        options.checkpoint_interval = Duration::from_secs(u64::MAX);
        options.checkpoint_target_size = usize::MAX;
        options.throttle_memory_limit = usize::MAX;
        options.stall_memory_limit = usize::MAX;
        options.write_txn_memory_limit = usize::MAX;

        Ok(Self(environment.get_or_create_database_with(name, options)?))
    }
}
