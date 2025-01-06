use biter::bitcoin::{BlockHash, Txid};

use super::Addressbytes;

pub trait Prefix {
    fn prefix(&self) -> &[u8];
}

impl Prefix for Addressbytes {
    fn prefix(&self) -> &[u8] {
        &self[..8]
    }
}

impl Prefix for BlockHash {
    fn prefix(&self) -> &[u8] {
        &self[..8]
    }
}

impl Prefix for Txid {
    fn prefix(&self) -> &[u8] {
        &self[..8]
    }
}
