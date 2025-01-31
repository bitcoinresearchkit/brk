use std::{
    fmt::Debug,
    fs, io,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use super::{Height, Version};

#[derive(Debug)]
pub struct StorableVec<I, T> {
    height: Option<Height>,
    pathbuf: PathBuf,
    version: Version,
    vec: storable_vec::StorableVec<I, T>,
}

impl<I, T> StorableVec<I, T>
where
    I: TryInto<usize>,
    T: Sized + Debug + Clone,
{
    pub fn import(path: &Path, version: Version) -> io::Result<Self> {
        fs::create_dir_all(path)?;

        let pathbuf = path.to_owned();
        let path_vec = Self::path_vec_(path);
        let path_version = Self::path_version_(path);

        let is_same_version =
            Version::try_from(path_version.as_path()).is_ok_and(|prev_version| version == prev_version);
        if !is_same_version {
            let _ = fs::remove_file(&path_vec);
            let _ = fs::remove_file(&path_version);
            let _ = fs::remove_file(Self::path_height_(path));
        }

        let this = Self {
            height: Height::try_from(Self::path_height_(path).as_path()).ok(),
            pathbuf,
            version,
            vec: storable_vec::StorableVec::import(&path_vec)?,
        };

        this.version.write(&this.path_version())?;

        Ok(this)
    }

    pub fn flush(&mut self, height: Height) -> io::Result<()> {
        if self.needs(height) {
            height.write(&self.path_height())?;
        }

        self.vec.flush()
    }

    // fn path_vec(&self) -> PathBuf {
    //     Self::_path_vec(&self.path)
    // }
    fn path_vec_(path: &Path) -> PathBuf {
        path.join("vec")
    }

    fn path_version(&self) -> PathBuf {
        Self::path_version_(&self.pathbuf)
    }
    fn path_version_(path: &Path) -> PathBuf {
        path.join("version")
    }

    pub fn height(&self) -> color_eyre::Result<Height> {
        Height::try_from(self.path_height().as_path())
    }
    fn path_height(&self) -> PathBuf {
        Self::path_height_(&self.pathbuf)
    }
    fn path_height_(path: &Path) -> PathBuf {
        path.join("height")
    }

    pub fn needs(&self, height: Height) -> bool {
        self.height.is_none_or(|self_height| height > self_height)
    }
    #[allow(unused)]
    pub fn has(&self, height: Height) -> bool {
        !self.needs(height)
    }
}

impl<I, T> Deref for StorableVec<I, T> {
    type Target = storable_vec::StorableVec<I, T>;
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
impl<I, T> DerefMut for StorableVec<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

pub trait AnyStorableVec {
    fn height(&self) -> color_eyre::Result<Height>;
    fn flush(&mut self, height: Height) -> io::Result<()>;
}

impl<I, T> AnyStorableVec for StorableVec<I, T>
where
    I: Into<usize>,
    T: Sized + Debug + Clone,
{
    fn height(&self) -> color_eyre::Result<Height> {
        self.height()
    }

    fn flush(&mut self, height: Height) -> io::Result<()> {
        self.flush(height)
    }
}
