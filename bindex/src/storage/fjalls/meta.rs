use std::{
    fs, io,
    path::{Path, PathBuf},
};

use storable_vec::UnsafeSizedSerDe;

use super::{Height, Version};

pub struct Meta {
    pathbuf: PathBuf,
    version: Version,
    height: Option<Height>,
    len: usize,
}

impl Meta {
    pub fn checked_open(path: &Path, version: Version) -> color_eyre::Result<Self> {
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
            len: Self::read_length_(path)?,
        };

        this.version.write(&this.path_version())?;

        Ok(this)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn export(&mut self, len: usize, height: Height) -> io::Result<()> {
        self.len = len;
        self.write_length()?;
        self.height = Some(height);
        height.write(&self.path_height())
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

    fn read_length(&self) -> color_eyre::Result<usize> {
        Self::read_length_(&self.pathbuf)
    }
    fn read_length_(path: &Path) -> color_eyre::Result<usize> {
        Ok(fs::read(Self::path_length(path))
            .map(|v| usize::unsafe_try_from_slice(v.as_slice()).cloned().unwrap_or_default())
            .unwrap_or_default())
    }
    fn write_length(&self) -> io::Result<()> {
        Self::write_length_(&self.pathbuf, self.len)
    }
    fn write_length_(path: &Path, len: usize) -> Result<(), io::Error> {
        fs::write(Self::path_length(path), len.to_le_bytes())
    }
    fn path_length(path: &Path) -> PathBuf {
        path.join("length")
    }
}
