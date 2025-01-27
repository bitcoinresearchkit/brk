use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use canopydb::{Database as CanopyDatabase, DbOptions, Error, WriteTransaction};

use super::Environment;

#[derive(Debug)]
pub struct Database {
    db: CanopyDatabase,
    // pub wtx: WriteTransaction,
}
impl Deref for Database {
    type Target = CanopyDatabase;
    fn deref(&self) -> &Self::Target {
        &self.db
    }
}
impl DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.db
    }
}

impl Database {
    pub fn new(environment: &Environment, name: &str) -> color_eyre::Result<Self> {
        let mut options = DbOptions::default();
        options.use_wal = false;
        options.checkpoint_interval = Duration::from_secs(u64::MAX);
        options.checkpoint_target_size = usize::MAX;
        options.throttle_memory_limit = usize::MAX;
        options.stall_memory_limit = usize::MAX;
        options.write_txn_memory_limit = usize::MAX;

        let db = environment.get_or_create_database_with(name, options)?;

        Ok(Self {
            // wtx: db.begin_write()?,
            db,
        })
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        // drop(blockhash_prefix_to_height_tree);
        // blockhash_prefix_to_height_tx_opt.take().map(|tx| tx.commit());
        self.checkpoint()
    }
}
