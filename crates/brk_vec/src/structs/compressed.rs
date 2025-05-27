use std::{fs, io, ops::Deref, path::Path};

use brk_core::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Compressed(bool);

impl Compressed {
    pub const YES: Self = Self(true);
    pub const NO: Self = Self(false);

    pub fn write(&self, path: &Path) -> Result<(), io::Error> {
        fs::write(path, self.as_bytes())
    }

    fn as_bytes(&self) -> Vec<u8> {
        if self.0 { vec![1] } else { vec![0] }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() != 1 {
            panic!();
        }
        if bytes[0] == 1 {
            Self(true)
        } else if bytes[0] == 0 {
            Self(false)
        } else {
            panic!()
        }
    }

    pub fn validate(&self, path: &Path) -> Result<()> {
        if let Ok(prev_compressed) = Compressed::try_from(path) {
            if prev_compressed != *self {
                return Err(Error::DifferentCompressionMode);
            }
        }

        Ok(())
    }
}

impl TryFrom<&Path> for Compressed {
    type Error = Error;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(&fs::read(value)?))
    }
}

impl From<bool> for Compressed {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Deref for Compressed {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
