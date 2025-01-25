use std::{
    fs, io,
    path::{Path, PathBuf},
};

use snkrj::UnitDatabase;

use super::{Height, Version};

pub struct StoreMeta {
    pathbuf: PathBuf,
    version: Version,
    height: Option<Height>,
    pub len: usize,
}

impl StoreMeta {
    pub fn checked_open(path: &Path, version: Version) -> Result<Self, snkrj::Error> {
        fs::create_dir_all(path)?;

        let is_same_version =
            Version::try_from(Self::path_version_(path).as_path()).is_ok_and(|prev_version| version == prev_version);

        if !is_same_version {
            fs::remove_dir_all(path)?;
            fs::create_dir(path)?;
        }

        let this = Self {
            pathbuf: path.to_owned(),
            version,
            height: Height::try_from(Self::path_height_(path).as_path()).ok(),
            len: UnitDatabase::read_length_(path),
        };

        this.version.write(&this.path_version())?;

        Ok(this)
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn export(mut self, height: Height) -> Result<(), io::Error> {
        self.height = Some(height);
        height.write(&self.path_height())?;
        UnitDatabase::write_length_(&self.pathbuf, self.len)
    }

    pub fn path_parts(&self) -> PathBuf {
        Self::path_parts_(&self.pathbuf)
    }
    fn path_parts_(path: &Path) -> PathBuf {
        path.join("parts")
    }

    fn path_version(&self) -> PathBuf {
        Self::path_version_(&self.pathbuf)
    }
    fn path_version_(path: &Path) -> PathBuf {
        path.join("version")
    }

    pub fn height(&self) -> Option<&Height> {
        self.height.as_ref()
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
}
