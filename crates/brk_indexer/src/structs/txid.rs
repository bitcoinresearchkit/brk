use std::mem;

use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Deref, Clone, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
pub struct Txid([u8; 32]);

impl From<bitcoin::Txid> for Txid {
    fn from(value: bitcoin::Txid) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<Txid> for bitcoin::Txid {
    fn from(value: Txid) -> Self {
        unsafe { mem::transmute(value) }
    }
}
