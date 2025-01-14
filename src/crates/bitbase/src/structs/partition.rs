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

    pub fn import(
        keyspace: &TransactionalKeyspace,
        name: &str,
        version: Version,
        exit: &Exit,
    ) -> color_eyre::Result<Self> {
        let data = Self::open_data(keyspace, name)?;
        let meta = Self::open_meta(keyspace, name)?;

        let mut height = None;
        if let Some(height_res) = meta.get(Self::HEIGHT)?.map(Height::try_from) {
            height = Some(height_res?);
        }

        let mut this = Self {
            version,
            height,
            data,
            meta,
        };

        let mut different_version = false;
        if let Some(slice) = this.meta.get(Self::VERSION)? {
            different_version = Version::try_from(slice).map_or(true, |version2| version != version2);
        }

        if different_version {
            this = this.reset(keyspace, name, exit)?;
        }

        Ok(this)
    }

    fn open_data(keyspace: &TransactionalKeyspace, name: &str) -> Result<TransactionalPartitionHandle> {
        keyspace.open_partition(&format!("{name}_data"), Self::create_options())
    }

    fn open_meta(keyspace: &TransactionalKeyspace, name: &str) -> Result<TransactionalPartitionHandle> {
        keyspace.open_partition(&format!("{name}_meta"), Self::create_options())
    }

    fn create_options() -> PartitionCreateOptions {
        PartitionCreateOptions::default().manual_journal_persist(true)
    }

    // TODO: Still needed ?
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
