pub use fjall::{PartitionCreateOptions, PersistMode, Result, TransactionalKeyspace, TransactionalPartitionHandle};

use crate::structs::{Height, Version};

use super::Exit;

pub struct Partition {
    version: Version,
    data: TransactionalPartitionHandle,
    meta: TransactionalPartitionHandle,
    height: Option<Height>,
}

impl Partition {
    pub const VERSION: &str = "version";
    pub const HEIGHT: &str = "height";

    pub fn import(keyspace: &TransactionalKeyspace, name: &str, version: Version, exit: &Exit) -> Result<Self> {
        let data = Self::open_data(keyspace, name)?;
        let meta = Self::open_meta(keyspace, name)?;

        let mut this = Self {
            version,
            height: meta.get(Self::HEIGHT)?.map(Height::from),
            data,
            meta,
        };

        if let Some(slice) = this.meta.get(Self::VERSION)? {
            if version != Version::from(slice) {
                this = this.reset(keyspace, name, exit)?;
            }
        }

        Ok(this)
    }

    fn open_data(keyspace: &TransactionalKeyspace, name: &str) -> Result<TransactionalPartitionHandle> {
        keyspace.open_partition(&format!("{name}-data"), Self::create_options())
    }

    fn open_meta(keyspace: &TransactionalKeyspace, name: &str) -> Result<TransactionalPartitionHandle> {
        keyspace.open_partition(&format!("{name}-meta"), Self::create_options())
    }

    fn create_options() -> PartitionCreateOptions {
        PartitionCreateOptions::default().manual_journal_persist(true)
    }

    pub fn is_safe(&self, height: Height) -> bool {
        self.height.is_some_and(|self_height| self_height >= height)
    }

    pub fn needs(&self, height: Height) -> bool {
        !self.is_safe(height)
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn data(&self) -> &TransactionalPartitionHandle {
        &self.data
    }

    pub fn meta(&self) -> &TransactionalPartitionHandle {
        &self.meta
    }

    fn reset(mut self, keyspace: &TransactionalKeyspace, name: &str, exit: &Exit) -> Result<Self> {
        exit.block();

        keyspace.delete_partition(self.data)?;
        keyspace.delete_partition(self.meta)?;

        keyspace.persist(PersistMode::SyncAll)?;

        self.data = Self::open_data(keyspace, name)?;
        self.meta = Self::open_meta(keyspace, name)?;
        self.height = None;

        exit.unblock();

        Ok(self)
    }

    pub fn height(&self) -> &Option<Height> {
        &self.height
    }
}
