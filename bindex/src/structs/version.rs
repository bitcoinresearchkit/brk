use std::{fs, io, path::Path};

use storable_vec::UnsafeSizedSerDe;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u32);

impl Version {
    pub fn write(&self, path: &Path) -> Result<(), io::Error> {
        fs::write(path, self.unsafe_as_slice())
    }
}

impl From<u32> for Version {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<&Path> for Version {
    type Error = color_eyre::Report;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Ok(Self::unsafe_try_from_slice(fs::read(value)?.as_slice())?.to_owned())
    }
}
