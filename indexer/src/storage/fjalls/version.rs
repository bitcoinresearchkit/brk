use std::{fs, io, path::Path};

use derive_deref::Deref;
use fjall::Slice;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deref)]
pub struct Version(u32);

impl Version {
    pub fn write(&self, path: &Path) -> Result<(), io::Error> {
        fs::write(path, self.to_ne_bytes())
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
        Self::try_from(&fs::read(value)?)
    }
}
impl TryFrom<Slice> for Version {
    type Error = fjall::Error;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        Self::from(&value)
    }
}
impl TryFrom<&[u8]> for Version {
    type Error = storable_vec::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut buf: [u8; 4] = [0; 4];
        let buf_len = buf.len();
        if value.len() != buf_len {
            panic!();
        }
        value.iter().enumerate().for_each(|(i, r)| {
            buf[i] = *r;
        });
        Ok(Self(u32::from_ne_bytes(buf)))
    }
}
impl From<Version> for Slice {
    fn from(value: Version) -> Self {
        Self::new(&value.to_ne_bytes())
    }
}
