use lsm_tree::{Ingestion as TreeIngestion, Slice};

use crate::{Keyspace, Result};

/// A strictly sorted stream written directly into `SSTables`.
pub struct Ingestion<'a> {
    keyspace: &'a Keyspace,
    inner: TreeIngestion<'a>,
}

impl<'a> Ingestion<'a> {
    /// Starts an ingestion for `keyspace`.
    pub fn new(keyspace: &'a Keyspace) -> Result<Self> {
        let inner = TreeIngestion::new(&keyspace.inner.tree)?;
        Ok(Self { keyspace, inner })
    }

    /// Appends a key-value pair. Keys must be strictly increasing.
    ///
    /// # Errors
    ///
    /// Returns an error if the table writer fails.
    pub fn write<K: Into<Slice>, V: Into<Slice>>(&mut self, key: K, value: V) -> Result<()> {
        self.inner.write(key, value).map_err(Into::into)
    }

    /// Appends a weak tombstone. Keys must be strictly increasing.
    ///
    /// # Errors
    ///
    /// Returns an error if the table writer fails.
    pub fn write_weak_tombstone<K: Into<Slice>>(&mut self, key: K) -> Result<()> {
        self.inner.write_weak_tombstone(key).map_err(Into::into)
    }

    /// Persists and publishes the new immutable tables.
    ///
    /// # Errors
    ///
    /// Returns an error if table or manifest persistence fails.
    pub fn finish(self) -> Result<()> {
        self.inner.finish()?;
        self.keyspace.request_compaction();
        Ok(())
    }
}
