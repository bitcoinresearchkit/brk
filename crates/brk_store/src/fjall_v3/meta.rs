use std::{
    fs, io,
    path::{Path, PathBuf},
};

use brk_error::Result;
use brk_types::Version;
use fjall3::{Database, Keyspace};

use super::Height;

#[derive(Debug, Clone)]
pub struct StoreMeta {
    pathbuf: PathBuf,
    version: Version,
    height: Option<Height>,
}

impl StoreMeta {
    pub fn checked_open<F>(
        _database: &Database,
        path: &Path,
        version: Version,
        open_partition_handle: F,
    ) -> Result<(Self, Keyspace)>
    where
        F: Fn() -> Result<Keyspace>,
    {
        fs::create_dir_all(path)?;

        let partition = open_partition_handle()?;

        if Version::try_from(Self::path_version_(path).as_path())
            .is_ok_and(|prev_version| version != prev_version)
        {
            todo!();
            // fs::remove_dir_all(path)?;
            // // Doesn't exist
            // // database.delete_partition(partition)?;
            // fs::create_dir(path)?;
            // database.persist(PersistMode::SyncAll)?;
            // partition = open_partition_handle()?;
        }

        let slf = Self {
            pathbuf: path.to_owned(),
            version,
            height: Height::try_from(Self::path_height_(path).as_path()).ok(),
        };

        slf.version.write(&slf.path_version())?;

        Ok((slf, partition))
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn export(&mut self, height: Height) -> io::Result<()> {
        self.height = Some(height);
        height.write(&self.path_height())
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

    #[inline]
    pub fn height(&self) -> Option<Height> {
        self.height
    }
    #[inline]
    pub fn needs(&self, height: Height) -> bool {
        self.height.is_none_or(|self_height| height > self_height)
    }
    #[inline]
    pub fn has(&self, height: Height) -> bool {
        !self.needs(height)
    }
    fn path_height(&self) -> PathBuf {
        Self::path_height_(&self.pathbuf)
    }
    fn path_height_(path: &Path) -> PathBuf {
        path.join("height")
    }
}
