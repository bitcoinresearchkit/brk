use std::{fs, io, path::Path};

use brk_core::{Error, Result};
use clap_derive::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum,
)]
pub enum Format {
    Compressed,
    #[default]
    Raw,
}

impl Format {
    pub fn write(&self, path: &Path) -> Result<(), io::Error> {
        fs::write(path, self.as_bytes())
    }

    pub fn is_raw(&self) -> bool {
        *self == Self::Raw
    }

    pub fn is_compressed(&self) -> bool {
        *self == Self::Compressed
    }

    fn as_bytes(&self) -> Vec<u8> {
        if self.is_compressed() {
            vec![1]
        } else {
            vec![0]
        }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() != 1 {
            panic!();
        }
        if bytes[0] == 1 {
            Self::Compressed
        } else if bytes[0] == 0 {
            Self::Raw
        } else {
            panic!()
        }
    }

    pub fn validate(&self, path: &Path) -> Result<()> {
        if let Ok(prev_compressed) = Format::try_from(path) {
            if prev_compressed != *self {
                return Err(Error::DifferentCompressionMode);
            }
        }

        Ok(())
    }
}

impl TryFrom<&Path> for Format {
    type Error = Error;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(&fs::read(value)?))
    }
}
