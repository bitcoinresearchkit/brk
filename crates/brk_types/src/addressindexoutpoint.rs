use std::hash::{Hash, Hasher};

use byteview::ByteView;
use serde::Serialize;
use vecdb::Bytes;

use crate::{AddressIndexTxIndex, Vout};

use super::{OutPoint, TypeIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize)]
#[repr(C)]
pub struct AddressIndexOutPoint {
    addressindextxindex: AddressIndexTxIndex, // u64
    vout: Vout,                               // u16
}

impl Hash for AddressIndexOutPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut buf = [0u8; 10];
        buf[0..8].copy_from_slice(&self.addressindextxindex.to_bytes());
        buf[8..].copy_from_slice(&self.vout.to_bytes());
        state.write(&buf);
    }
}

impl From<(TypeIndex, OutPoint)> for AddressIndexOutPoint {
    #[inline]
    fn from((addressindex, outpoint): (TypeIndex, OutPoint)) -> Self {
        Self {
            addressindextxindex: AddressIndexTxIndex::from((addressindex, outpoint.txindex())),
            vout: outpoint.vout(),
        }
    }
}

impl From<ByteView> for AddressIndexOutPoint {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self {
            addressindextxindex: AddressIndexTxIndex::from_bytes(&value[0..8]).unwrap(),
            vout: Vout::from_bytes(&value[8..]).unwrap(),
        }
    }
}

impl From<AddressIndexOutPoint> for ByteView {
    #[inline]
    fn from(value: AddressIndexOutPoint) -> Self {
        ByteView::from(&value)
    }
}
impl From<&AddressIndexOutPoint> for ByteView {
    #[inline]
    fn from(value: &AddressIndexOutPoint) -> Self {
        ByteView::from(
            [
                &ByteView::from(value.addressindextxindex),
                value.vout.to_be_bytes().as_slice(),
            ]
            .concat(),
        )
    }
}
