use std::{
    fs, io,
    path::{Path, PathBuf},
};

use brk_core::{Result, Version, copy_first_8bytes};
use fjall::{TransactionalKeyspace, TransactionalPartitionHandle};

use super::Height;

#[derive(Debug, Clone)]
pub struct StoreMeta {
    pathbuf: PathBuf,
    version: Version,
    height: Option<Height>,
    len: usize,
}

impl StoreMeta {
    pub fn checked_open<F>(
        keyspace: &TransactionalKeyspace,
        path: &Path,
        version: Version,
        open_partition_handle: F,
    ) -> Result<(Self, TransactionalPartitionHandle)>
    where
        F: Fn() -> Result<TransactionalPartitionHandle>,
    {
        fs::create_dir_all(path)?;

        let read_version = Version::try_from(Self::path_version_(path).as_path());

        let is_same_version = read_version
            .as_ref()
            .is_ok_and(|prev_version| &version == prev_version);

        let mut partition = open_partition_handle()?;

        if !is_same_version {
            fs::remove_dir_all(path)?;
            fs::create_dir(path)?;
            keyspace.delete_partition(partition)?;
            keyspace.persist(fjall::PersistMode::SyncAll)?;
            partition = open_partition_handle()?;
        }

        let len = Self::read_length_(path);

        let slf = Self {
            pathbuf: path.to_owned(),
            version,
            height: Height::try_from(Self::path_height_(path).as_path()).ok(),
            len,
        };

        slf.version.write(&slf.path_version())?;

        Ok((slf, partition))
    }

    pub fn len(&self) -> usize {
        self.len
    }

    // pub fn is_empty(&self) -> bool {
    //     self.len() == 0
    // }

    // pub fn version(&self) -> Version {
    //     self.version
    // }

    pub fn export(&mut self, len: usize, height: Height) -> io::Result<()> {
        self.len = len;
        self.write_length()?;
        self.height = Some(height);
        height.write(&self.path_height())
    }

    pub fn reset(&mut self) {
        self.height.take();
        self.len = 0
    }

    pub fn path(&self) -> &Path {
        &self.pathbuf
    }

    fn path_version(&self) -> PathBuf {
        Self::path_version_(&self.pathbuf)
    }
    fn path_version_(path: &Path) -> PathBuf {
        path.join("version")
    }

    pub fn height(&self) -> Option<Height> {
        self.height
    }
    pub fn needs(&self, height: Height) -> bool {
        self.height.is_none_or(|self_height| height > self_height)
    }
    pub fn has(&self, height: Height) -> bool {
        !self.needs(height)
    }
    fn path_height(&self) -> PathBuf {
        Self::path_height_(&self.pathbuf)
    }
    fn path_height_(path: &Path) -> PathBuf {
        path.join("height")
    }

    fn read_length_(path: &Path) -> usize {
        fs::read(Self::path_length(path))
            .map(|v| usize::from_ne_bytes(copy_first_8bytes(v.as_slice()).unwrap()))
            .unwrap_or_default()
    }
    fn write_length(&self) -> io::Result<()> {
        Self::write_length_(&self.pathbuf, self.len)
    }
    fn write_length_(path: &Path, len: usize) -> io::Result<()> {
        fs::write(Self::path_length(path), len.to_ne_bytes())
    }
    fn path_length(path: &Path) -> PathBuf {
        path.join("length")
    }
}
