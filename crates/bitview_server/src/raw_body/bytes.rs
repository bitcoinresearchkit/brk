use axum::body::Bytes;

use super::RawBodyPermit;

pub struct RetainedBytes {
    pub bytes: Bytes,
    pub _permit: RawBodyPermit,
}

impl AsRef<[u8]> for RetainedBytes {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}
