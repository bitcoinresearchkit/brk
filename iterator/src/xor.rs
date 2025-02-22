use std::{fs, path::Path};

const XOR_LEN: usize = 8;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct XOR([u8; XOR_LEN]);

impl XOR {
    pub fn process(&self, mut bytes: Vec<u8>) -> Vec<u8> {
        if u64::from_ne_bytes(self.0) == 0 {
            return bytes;
        }

        let len = bytes.len();
        let mut bytes_index = 0;
        let mut xor_index = 0;

        while bytes_index < len {
            bytes[bytes_index] ^= self.0[xor_index];
            bytes_index += 1;
            xor_index += 1;
            if xor_index == XOR_LEN {
                xor_index = 0;
            }
        }

        bytes
    }
}

impl From<&Path> for XOR {
    fn from(value: &Path) -> Self {
        Self(
            fs::read(value.join("blocks/xor.dat"))
                .unwrap_or(vec![0; 8])
                .try_into()
                .unwrap(),
        )
    }
}
