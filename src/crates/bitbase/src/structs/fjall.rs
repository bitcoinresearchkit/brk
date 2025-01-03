use std::{mem, ops::RangeBounds};

pub use fjall::*;

use crate::structs::{Height, Version};

pub struct Database {
    keyspace: Keyspace,
    data: PartitionHandle,
    meta: PartitionHandle,
    batch: Batch,
    height: Option<Height>,
}

const VERSION: &str = "version";
const HEIGHT: &str = "height";

impl Database {
    pub fn import(name: &str, version: Version) -> Result<Self> {
        let keyspace = fjall::Config::new(format!("./database/{name}")).open()?;

        let data = Self::open_data(&keyspace)?;
        let meta = Self::open_meta(&keyspace)?;

        let batch = keyspace.batch();

        let mut this = Self {
            height: meta.get(HEIGHT)?.map(Height::from),
            keyspace,
            data,
            meta,
            batch,
        };

        if let Some(slice) = this.meta.get(VERSION)? {
            if version != Version::from(slice) {
                this = this.reset()?;
            }
        }

        this.batch
            .insert(&this.meta, VERSION, version.to_be_bytes());

        Ok(this)
    }

    fn open_data(keyspace: &Keyspace) -> Result<PartitionHandle> {
        keyspace.open_partition("data", Self::create_options())
    }

    fn open_meta(keyspace: &Keyspace) -> Result<PartitionHandle> {
        keyspace.open_partition("meta", Self::create_options())
    }

    fn create_options() -> PartitionCreateOptions {
        PartitionCreateOptions::default().manual_journal_persist(true)
    }

    pub fn get(&self, key: Slice) -> Result<Option<Slice>> {
        self.data.get(key)
    }

    pub fn range<'a, K: AsRef<[u8]> + 'a, R: RangeBounds<K> + 'a>(
        &'a self,
        range: R,
    ) -> impl DoubleEndedIterator<Item = Result<KvPair>> + 'static {
        self.data.range(range)
    }

    pub fn prefix<'a, K: AsRef<[u8]> + 'a>(
        &'a self,
        prefix: K,
    ) -> impl DoubleEndedIterator<Item = Result<KvPair>> + 'static {
        self.data.prefix(prefix)
    }

    pub fn insert(&mut self, key: Slice, value: Slice, height: Height) {
        if self.is_safe(height) {
            return;
        }
        self.batch.insert(&self.data, key, value);
    }

    pub fn fetch_update(
        &mut self,
        key: Slice,
        value: Slice,
        height: Height,
    ) -> Result<Option<Slice>> {
        if self.is_safe(height) {
            return Ok(None);
        }
        let prev = self.get(key.clone());
        self.batch.insert(&self.data, key, value);
        prev
    }

    pub fn remove(&mut self, key: Slice) {
        self.batch.remove(&self.data, key);
    }

    pub fn is_safe(&self, height: Height) -> bool {
        self.height.is_some_and(|self_height| self_height >= height)
    }

    fn persist(&self) -> Result<()> {
        self.keyspace.persist(PersistMode::SyncAll)
    }

    pub fn export(&mut self, height: Height) -> Result<()> {
        let mut batch = self.keyspace.batch();
        mem::swap(&mut batch, &mut self.batch);

        batch.insert(&self.meta, HEIGHT, height.to_be_bytes());

        batch.commit()?;

        self.persist()
    }

    fn reset(mut self) -> Result<Self> {
        self.keyspace.delete_partition(self.data)?;
        self.keyspace.delete_partition(self.meta)?;

        self.keyspace.persist(PersistMode::SyncAll)?;

        self.data = Self::open_data(&self.keyspace)?;
        self.meta = Self::open_meta(&self.keyspace)?;

        Ok(self)
    }

    pub fn height(&self) -> &Option<Height> {
        &self.height
    }
}

pub trait DatabaseTrait
where
    Self: Sized,
{
    fn version() -> Version;
}
