use std::{fmt, mem, str::FromStr};

use bitcoin::hashes::Hash;
use brk_error::Error;
use derive_deref::Deref;
use schemars::JsonSchema;
use serde::{Serialize, Serializer};
use vecdb::{Bytes, Formattable};

/// Block hash
#[derive(Debug, Deref, Clone, PartialEq, Eq, Bytes, JsonSchema)]
#[repr(C)]
pub struct BlockHash([u8; 32]);

impl TryFrom<&str> for BlockHash {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self::from(bitcoin::BlockHash::from_str(s)?))
    }
}

impl FromStr for BlockHash {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl From<bitcoin::BlockHash> for BlockHash {
    #[inline]
    fn from(value: bitcoin::BlockHash) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<&bitcoin::BlockHash> for &BlockHash {
    #[inline]
    fn from(value: &bitcoin::BlockHash) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<BlockHash> for bitcoin::BlockHash {
    #[inline]
    fn from(value: BlockHash) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<&BlockHash> for &bitcoin::BlockHash {
    #[inline]
    fn from(value: &BlockHash) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<&BlockHash> for bitcoin::BlockHash {
    #[inline]
    fn from(value: &BlockHash) -> Self {
        bitcoin::BlockHash::from_slice(&value.0).unwrap()
    }
}

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&bitcoin::BlockHash::from(self).to_string())
    }
}

impl Serialize for BlockHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Formattable for BlockHash {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
