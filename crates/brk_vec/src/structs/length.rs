use std::{
    fs,
    io::{self, Read},
    ops::{AddAssign, Deref, DerefMut},
    path::Path,
};

use brk_core::{Error, Result};
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    IntoBytes,
    Immutable,
    KnownLayout,
)]
pub struct Length(usize);

impl Length {
    pub fn write(&self, path: &Path) -> Result<(), io::Error> {
        fs::write(path, self.as_bytes())
    }
}

impl From<usize> for Length {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Deref for Length {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Length {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<&Path> for Length {
    type Error = Error;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let mut buf = [0; 8];
        if let Ok(bytes) = fs::read(value) {
            bytes.as_slice().read_exact(&mut buf)?;
            Ok(*(Self::ref_from_bytes(&buf)?))
        } else {
            Ok(Self::default())
        }
    }
}

impl AddAssign<usize> for Length {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}
