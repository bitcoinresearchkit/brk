use std::hash::{Hash, Hasher};

use byteview::ByteView;
use serde::Serialize;
use zerocopy::IntoBytes;

use crate::{TypeIndexAndTxIndex, Vout};

use super::{OutPoint, TypeIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Serialize)]
#[repr(C)]
pub struct TypeIndexAndOutPoint {
    typeindexandtxindex: TypeIndexAndTxIndex, // u64
    vout: Vout,                               // u16
}

impl Hash for TypeIndexAndOutPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut buf = [0u8; 10];
        buf[..8].copy_from_slice(self.typeindexandtxindex.as_bytes());
        buf[8..].copy_from_slice(self.vout.as_bytes());
        state.write(&buf);
    }
}

impl From<(TypeIndex, OutPoint)> for TypeIndexAndOutPoint {
    fn from(value: (TypeIndex, OutPoint)) -> Self {
        Self {
            typeindexandtxindex: TypeIndexAndTxIndex::from((value.0, value.1.txindex())),
            vout: value.1.vout(),
        }
    }
}

impl From<ByteView> for TypeIndexAndOutPoint {
    fn from(value: ByteView) -> Self {
        Self {
            typeindexandtxindex: TypeIndexAndTxIndex::from(&value[0..8]),
            vout: Vout::from(&value[8..]),
        }
    }
}

impl From<TypeIndexAndOutPoint> for ByteView {
    fn from(value: TypeIndexAndOutPoint) -> Self {
        ByteView::from(&value)
    }
}
impl From<&TypeIndexAndOutPoint> for ByteView {
    fn from(value: &TypeIndexAndOutPoint) -> Self {
        ByteView::from(
            [
                value.typeindexandtxindex.to_be_bytes().as_slice(),
                value.vout.to_be_bytes().as_slice(),
            ]
            .concat(),
        )
    }
}
