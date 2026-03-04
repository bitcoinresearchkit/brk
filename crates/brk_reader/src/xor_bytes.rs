use std::{fs, path::Path};

use derive_more::Deref;

pub const XOR_LEN: usize = 8;

#[derive(Debug, Clone, Copy, Deref, PartialEq, Eq)]
pub struct XORBytes([u8; XOR_LEN]);

impl From<[u8; XOR_LEN]> for XORBytes {
    fn from(value: [u8; XOR_LEN]) -> Self {
        Self(value)
    }
}

impl From<&Path> for XORBytes {
    #[inline]
    fn from(value: &Path) -> Self {
        Self(
            fs::read(value.join("xor.dat"))
                .unwrap_or(vec![0; 8])
                .try_into()
                .unwrap(),
        )
    }
}
