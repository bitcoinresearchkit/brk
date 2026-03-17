use std::hash::{Hash, Hasher};

use byteview::ByteView;
use serde::Serialize;
use vecdb::Bytes;

use crate::{AddrIndexTxIndex, Vout};

use super::{OutPoint, TxIndex, TypeIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize)]
#[repr(C)]
pub struct AddrIndexOutPoint {
    addr_index_tx_index: AddrIndexTxIndex, // u64
    vout: Vout,                               // u16
}

impl AddrIndexOutPoint {
    #[inline]
    pub fn tx_index(&self) -> TxIndex {
        self.addr_index_tx_index.tx_index()
    }

    #[inline]
    pub fn vout(&self) -> Vout {
        self.vout
    }
}

impl Hash for AddrIndexOutPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut buf = [0u8; 10];
        buf[0..8].copy_from_slice(&self.addr_index_tx_index.to_bytes());
        buf[8..].copy_from_slice(&self.vout.to_bytes());
        state.write(&buf);
    }
}

impl From<(TypeIndex, OutPoint)> for AddrIndexOutPoint {
    #[inline]
    fn from((addr_index, outpoint): (TypeIndex, OutPoint)) -> Self {
        Self {
            addr_index_tx_index: AddrIndexTxIndex::from((addr_index, outpoint.tx_index())),
            vout: outpoint.vout(),
        }
    }
}

impl From<ByteView> for AddrIndexOutPoint {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self {
            addr_index_tx_index: AddrIndexTxIndex::from(ByteView::new(&value[..8])),
            vout: Vout::from(u16::from_be_bytes([value[8], value[9]])),
        }
    }
}

impl From<AddrIndexOutPoint> for ByteView {
    #[inline]
    fn from(value: AddrIndexOutPoint) -> Self {
        ByteView::from(&value)
    }
}
impl From<&AddrIndexOutPoint> for ByteView {
    #[inline]
    fn from(value: &AddrIndexOutPoint) -> Self {
        ByteView::from(
            [
                &ByteView::from(value.addr_index_tx_index),
                value.vout.to_be_bytes().as_slice(),
            ]
            .concat(),
        )
    }
}
