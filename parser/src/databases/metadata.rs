use allocative::Allocative;
use bincode::{Decode, Encode};
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    fs, io,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use crate::{
    structs::{Counter, Date, Height},
    Serialization,
};

#[derive(Default, Debug, Encode, Decode, Allocative)]
pub struct Metadata {
    path: PathBuf,
    data: MetadataData,
}

impl Deref for Metadata {
    type Target = MetadataData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Metadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Metadata {
    pub fn import(path: PathBuf, version: u16) -> Self {
        Self {
            data: MetadataData::import(&path, version),
            path,
        }
    }

    pub fn export(&mut self, height: Height, date: Date) -> color_eyre::Result<()> {
        if self.last_height.unwrap_or_default() < height {
            self.last_height.replace(height);
        }

        if self.last_date.unwrap_or_default() < date {
            self.last_date.replace(date);
        }

        self.data.export(&self.path)
    }

    pub fn reset(&mut self) {
        let _ = self.data.reset(&self.path);
    }

    pub fn called_insert(&mut self) {
        self.serial += 1;
        self.len.increment();
    }

    pub fn called_remove(&mut self) {
        self.len.decrement();
    }

    pub fn check_if_in_sync(&self, other: &Self) -> bool {
        self.last_date == other.last_date && self.last_height == other.last_height
    }

    pub fn check_farer_or_in_sync(&self, other: &Self) -> bool {
        self.last_date >= other.last_date && self.last_height >= other.last_height
    }
}

#[derive(Default, Debug, Encode, Decode, Serialize, Deserialize, Allocative)]
pub struct MetadataData {
    version: u16,
    pub serial: usize,
    pub len: Counter,
    pub last_height: Option<Height>,
    pub last_date: Option<Date>,
}

impl MetadataData {
    fn full_path(folder_path: &Path) -> PathBuf {
        folder_path.join("metadata")
    }

    pub fn import(path: &Path, version: u16) -> Self {
        let mut s = Self::_import(path, version).unwrap_or_default();
        s.version = version;
        s
    }

    fn _import(path: &Path, version: u16) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        let s: MetadataData = Serialization::Binary.import(path)?;

        if s.version != version {
            return Err(eyre!("Bad version"));
        }

        Ok(s)
    }

    pub fn export(&self, path: &Path) -> color_eyre::Result<()> {
        Serialization::Binary.export(Path::new(&Self::full_path(path)), self)
    }

    pub fn reset(&mut self, path: &Path) -> color_eyre::Result<(), io::Error> {
        self.clear();

        fs::remove_file(Self::full_path(path))
    }

    fn clear(&mut self) {
        self.serial = 0;
        self.len.reset();
        self.last_height = None;
        self.last_date = None;
    }
}
