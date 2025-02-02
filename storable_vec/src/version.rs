use std::{
    fs,
    io::{self, Read},
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u32);

impl Version {
    pub fn write(&self, path: &Path) -> Result<(), io::Error> {
        fs::write(path, self.0.to_le_bytes())
    }
}

impl From<u32> for Version {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<&Path> for Version {
    type Error = io::Error;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let mut buf = [0; 4];
        fs::read(value)?.as_slice().read_exact(&mut buf)?;
        Ok(Self(u32::from_le_bytes(buf)))
    }
}
