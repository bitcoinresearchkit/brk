use std::{
    fs,
    io::{self, Read},
    path::Path,
};

use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct Version(u32);

impl Version {
    pub fn write(&self, path: &Path) -> Result<(), io::Error> {
        fs::write(path, self.as_bytes())
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
        Ok(*(Self::ref_from_bytes(&buf)?))
    }
}
