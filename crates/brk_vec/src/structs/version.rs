use std::{
    fs,
    io::{self, Read},
    ops::Add,
    path::Path,
};

use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{Error, Result};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromBytes, IntoBytes, Immutable, KnownLayout,
)]
pub struct Version(u32);

impl Version {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);

    pub fn write(&self, path: &Path) -> Result<(), io::Error> {
        fs::write(path, self.as_bytes())
    }

    pub fn swap_bytes(self) -> Self {
        Self(self.0.swap_bytes())
    }

    pub fn validate(&self, path: &Path) -> Result<()> {
        if let Ok(prev_version) = Version::try_from(path) {
            if prev_version != *self {
                if prev_version.swap_bytes() == *self {
                    return Err(Error::WrongEndian);
                }
                return Err(Error::DifferentVersion {
                    found: prev_version,
                    expected: *self,
                });
            }
        }

        Ok(())
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

impl Add<Version> for Version {
    type Output = Self;
    fn add(self, rhs: Version) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
