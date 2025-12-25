//! Combined transaction output data for efficient access.

use std::fmt::{self, Display};
use std::mem::size_of;

use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{Bytes, Formattable};

use crate::{OutputType, Sats, TypeIndex};

/// Core transaction output data: value, type, and type index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[repr(C)]
pub struct TxOutData {
    pub value: Sats,
    pub typeindex: TypeIndex,
    pub outputtype: OutputType,
    _padding: u16,
}

impl TxOutData {
    #[inline]
    pub const fn new(value: Sats, outputtype: OutputType, typeindex: TypeIndex) -> Self {
        Self {
            value,
            typeindex,
            outputtype,
            _padding: 0,
        }
    }
}

impl Bytes for TxOutData {
    type Array = [u8; size_of::<Self>()];

    #[inline]
    fn to_bytes(&self) -> Self::Array {
        let mut bytes = [0u8; 16];
        bytes[0..8].copy_from_slice(&self.value.to_bytes());
        bytes[8..12].copy_from_slice(&self.typeindex.to_bytes());
        bytes[12..14].copy_from_slice(&self.outputtype.to_bytes());
        // bytes[14..16] is padding, already zero
        bytes
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> vecdb::Result<Self> {
        if bytes.len() != size_of::<Self>() {
            return Err(vecdb::Error::WrongLength {
                expected: size_of::<Self>(),
                received: bytes.len(),
            });
        }
        Ok(Self {
            value: Sats::from_bytes(&bytes[0..8])?,
            typeindex: TypeIndex::from_bytes(&bytes[8..12])?,
            outputtype: OutputType::from_bytes(&bytes[12..14])?,
            _padding: 0,
        })
    }
}

impl Display for TxOutData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "value: {}, outputtype: {}, typeindex: {}",
            self.value, self.outputtype, self.typeindex
        )
    }
}

impl Formattable for TxOutData {
    fn may_need_escaping() -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(size_of::<TxOutData>(), 16);
    }

    #[test]
    fn test_roundtrip() {
        let data = TxOutData::new(
            Sats::from(123456789u64),
            OutputType::P2TR,
            TypeIndex::from(42u32),
        );
        let bytes = data.to_bytes();
        let decoded = TxOutData::from_bytes(&bytes).unwrap();
        assert_eq!(data, decoded);
    }
}
