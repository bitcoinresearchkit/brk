use std::sync::Arc;

use crate::Result;

pub trait Bytes: Sized {
    const LEN: usize = size_of::<Self>();
    fn to_bytes(&self) -> Arc<[u8]>;
    fn try_from_bytes(bytes: &[u8]) -> Result<Self>;
}

pub trait UnsafeBytes {}
