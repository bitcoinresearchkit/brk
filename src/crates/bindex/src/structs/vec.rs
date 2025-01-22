use std::{
    fmt::Debug,
    fs, io,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use super::{Height, Version};

pub struct StorableVec<I, T> {
    pathbuf: PathBuf,
    version: Version,
    vec: storable_vec::StorableVec<I, T>,
}

impl<I, T> StorableVec<I, T>
where
    I: Into<usize>,
    T: Sized + Debug,
{
    pub fn import(path: &Path, version: Version) -> io::Result<Self> {
        fs::create_dir_all(path)?;

        let pathbuf = path.to_owned();
        let path_vec = Self::_path_vec(path);
        let path_version = Self::_path_version(path);

        let is_same_version =
            Version::try_from(path_version.as_path()).is_ok_and(|prev_version| version == prev_version);
        if !is_same_version {
            let _ = fs::remove_file(&path_vec);
            let _ = fs::remove_file(&path_version);
            let _ = fs::remove_file(Self::_path_height(path));
        }

        Ok(Self {
            pathbuf,
            version,
            vec: storable_vec::StorableVec::import(&path_vec)?,
        })
    }

    pub fn flush(&mut self, height: Height) -> io::Result<()> {
        height.write(&self.path_height())?;
        self.version.write(&self.path_version())?;
        self.vec.flush()
    }

    // fn path_vec(&self) -> PathBuf {
    //     Self::_path_vec(&self.path)
    // }
    fn _path_vec(path: &Path) -> PathBuf {
        path.join("vec")
    }

    fn path_version(&self) -> PathBuf {
        Self::_path_version(&self.pathbuf)
    }
    fn _path_version(path: &Path) -> PathBuf {
        path.join("version")
    }

    pub fn height(&self) -> color_eyre::Result<Height> {
        Height::try_from(self.path_height().as_path())
    }
    fn path_height(&self) -> PathBuf {
        Self::_path_height(&self.pathbuf)
    }
    fn _path_height(path: &Path) -> PathBuf {
        path.join("height")
    }

    fn reset_cache(&mut self) {
        self.vec.reset_cache();
    }

    // pub fn needs(&self, height: Height) -> bool {
    //     self.height() // store height in struct
    // }
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

pub trait AnyBindexVec {
    fn height(&self) -> color_eyre::Result<Height>;
    fn reset_cache(&mut self);
    fn flush(&mut self, height: Height) -> io::Result<()>;
}

impl<I, T> AnyBindexVec for StorableVec<I, T>
where
    I: Into<usize>,
    T: Sized + Debug,
{
    fn height(&self) -> color_eyre::Result<Height> {
        self.height()
    }

    fn reset_cache(&mut self) {
        self.reset_cache();
    }

    fn flush(&mut self, height: Height) -> io::Result<()> {
        self.flush(height)
    }
}
