use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use byteview::ByteView;
use redb::{Key, TypeName, Value};
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
    #[inline]
    fn from(value: (TypeIndex, OutPoint)) -> Self {
        Self {
            typeindexandtxindex: TypeIndexAndTxIndex::from((value.0, value.1.txindex())),
            vout: value.1.vout(),
        }
    }
}

impl From<ByteView> for TypeIndexAndOutPoint {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self {
            typeindexandtxindex: TypeIndexAndTxIndex::from(&value[0..8]),
            vout: Vout::from(&value[8..]),
        }
    }
}

impl From<TypeIndexAndOutPoint> for ByteView {
    #[inline]
    fn from(value: TypeIndexAndOutPoint) -> Self {
        ByteView::from(&value)
    }
}
impl From<&TypeIndexAndOutPoint> for ByteView {
    #[inline]
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

impl Value for TypeIndexAndOutPoint {
    type SelfType<'a> = TypeIndexAndOutPoint;
    type AsBytes<'a>
        = [u8; 10]
    // 8 bytes (u64) + 2 bytes (u16)
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        Some(10) // 8 + 2
    }

    fn from_bytes<'a>(data: &'a [u8]) -> TypeIndexAndOutPoint
    where
        Self: 'a,
    {
        TypeIndexAndOutPoint {
            typeindexandtxindex: TypeIndexAndTxIndex::from_bytes(&data[0..8]),
            vout: Vout::from_bytes(&data[8..10]),
        }
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> [u8; 10]
    where
        Self: 'a,
        Self: 'b,
    {
        let mut bytes = [0u8; 10];
        bytes[0..8].copy_from_slice(&<TypeIndexAndTxIndex as redb::Value>::as_bytes(
            &value.typeindexandtxindex,
        ));
        bytes[8..10].copy_from_slice(&<Vout as redb::Value>::as_bytes(&value.vout));
        bytes
    }

    fn type_name() -> TypeName {
        TypeName::new("TypeIndexAndOutPoint")
    }
}

impl Key for TypeIndexAndOutPoint {
    fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
        Self::from_bytes(data1).cmp(&Self::from_bytes(data2))
    }
}
