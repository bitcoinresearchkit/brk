use std::{
    fs,
    io::{self, Read},
    path::Path,
};

use unsafe_slice_serde::UnsafeSliceSerde;

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u32);

impl Version {
    pub fn write(&self, path: &Path) -> Result<(), io::Error> {
        fs::write(path, self.0.unsafe_as_slice())
    }

    pub fn swap_bytes(self) -> Self {
        Self(self.0.swap_bytes())
    }
}

impl From<u32> for Version {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<&Path> for Version {
    type Error = Error;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let mut buf = [0; 4];
        fs::read(value)?.as_slice().read_exact(&mut buf)?;
        Ok(*(Self::unsafe_try_from_slice(&buf)?))
    }
}
