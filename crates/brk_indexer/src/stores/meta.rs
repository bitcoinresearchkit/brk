use std::{
    fs, io,
    path::{Path, PathBuf},
};

use brk_vec::Version;
use fjall::{TransactionalKeyspace, TransactionalPartitionHandle};
use zerocopy::{FromBytes, IntoBytes};

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
    ) -> color_eyre::Result<(Self, TransactionalPartitionHandle)>
    where
        F: Fn() -> fjall::Result<TransactionalPartitionHandle>,
    {
        fs::create_dir_all(path)?;

        let is_same_version = Version::try_from(Self::path_version_(path).as_path())
            .is_ok_and(|prev_version| version == prev_version);

        let mut partition = open_partition_handle()?;

        if !is_same_version {
            Self::reset_(path)?;
            keyspace.delete_partition(partition)?;
            keyspace.persist(fjall::PersistMode::SyncAll)?;
            partition = open_partition_handle()?;
        }

        let slf = Self {
            pathbuf: path.to_owned(),
            version,
            height: Height::try_from(Self::path_height_(path).as_path()).ok(),
            len: Self::read_length_(path)?,
        };

        slf.version.write(&slf.path_version())?;

        Ok((slf, partition))
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn export(&mut self, len: usize, height: Height) -> io::Result<()> {
        self.len = len;
        self.write_length()?;
        self.height = Some(height);
        height.write(&self.path_height())
    }

    pub fn reset(&self) -> io::Result<()> {
        Self::reset_(self.pathbuf.as_path())
    }
    fn reset_(path: &Path) -> io::Result<()> {
        fs::remove_dir_all(path)?;
        fs::create_dir(path)
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

    fn read_length_(path: &Path) -> color_eyre::Result<usize> {
        Ok(fs::read(Self::path_length(path))
            .map(|v| usize::read_from_bytes(v.as_slice()).unwrap_or_default())
            .unwrap_or_default())
    }
    fn write_length(&self) -> io::Result<()> {
        Self::write_length_(&self.pathbuf, self.len)
    }
    fn write_length_(path: &Path, len: usize) -> io::Result<()> {
        fs::write(Self::path_length(path), len.as_bytes())
    }
    fn path_length(path: &Path) -> PathBuf {
        path.join("length")
    }
}
