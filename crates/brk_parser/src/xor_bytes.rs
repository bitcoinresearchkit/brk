use std::{fs, path::Path};

use derive_deref::Deref;

pub const XOR_LEN: usize = 8;

#[derive(Debug, Clone, Copy, Deref)]
pub struct XORBytes([u8; XOR_LEN]);

impl From<&Path> for XORBytes {
    fn from(value: &Path) -> Self {
        Self(
            fs::read(value.join("xor.dat"))
                .unwrap_or(vec![0; 8])
                .try_into()
                .unwrap(),
        )
    }
}
