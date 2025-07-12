use crate::xor_bytes::{XOR_LEN, XORBytes};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct XORIndex(usize);

impl XORIndex {
    pub fn bytes<'a>(&mut self, bytes: &'a mut [u8], xor_bytes: &XORBytes) -> &'a mut [u8] {
        let len = bytes.len();
        let mut bytes_index = 0;

        while bytes_index < len {
            bytes[bytes_index] ^= xor_bytes[self.0];
            self.increment();
            bytes_index += 1;
        }

        bytes
    }

    #[inline]
    pub fn byte(&mut self, mut byte: u8, xor_bytes: &XORBytes) -> u8 {
        byte ^= xor_bytes[self.0];
        self.increment();
        byte
    }

    #[inline]
    pub fn increment(&mut self) {
        self.0 += 1;
        if self.0 == XOR_LEN {
            self.0 = 0;
        }
    }

    #[inline]
    pub fn add_assign(&mut self, i: usize) {
        self.0 = (self.0 + i) % XOR_LEN;
    }
}
