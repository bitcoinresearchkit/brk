use std::hash::{Hash, Hasher};

use byteview::ByteView;
use serde::Serialize;
use zerocopy::IntoBytes;

use crate::{AddressTypeAddressIndexTxIndex, OutputType, Vout};

use super::{OutPoint, TypeIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize)]
#[repr(C)]
pub struct AddressTypeAddressIndexOutPoint {
    addresstypeaddressindextxindex: AddressTypeAddressIndexTxIndex, // (u8; u64)
    vout: Vout,                                                     // u16
}

impl Hash for AddressTypeAddressIndexOutPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut buf = [0u8; 11];
        buf[..1].copy_from_slice(self.addresstypeaddressindextxindex.addresstype().as_bytes());
        buf[1..9].copy_from_slice(
            self.addresstypeaddressindextxindex
                .addressindextxindex()
                .as_bytes(),
        );
        buf[9..].copy_from_slice(self.vout.as_bytes());
        state.write(&buf);
    }
}

impl From<(OutputType, TypeIndex, OutPoint)> for AddressTypeAddressIndexOutPoint {
    #[inline]
    fn from((addresstype, addressindex, outpoint): (OutputType, TypeIndex, OutPoint)) -> Self {
        Self {
            addresstypeaddressindextxindex: AddressTypeAddressIndexTxIndex::from((
                addresstype,
                addressindex,
                outpoint.txindex(),
            )),
            vout: outpoint.vout(),
        }
    }
}

impl From<ByteView> for AddressTypeAddressIndexOutPoint {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self {
            addresstypeaddressindextxindex: AddressTypeAddressIndexTxIndex::from(&value[0..9]),
            vout: Vout::from(&value[9..]),
        }
    }
}

impl From<AddressTypeAddressIndexOutPoint> for ByteView {
    #[inline]
    fn from(value: AddressTypeAddressIndexOutPoint) -> Self {
        ByteView::from(&value)
    }
}
impl From<&AddressTypeAddressIndexOutPoint> for ByteView {
    #[inline]
    fn from(value: &AddressTypeAddressIndexOutPoint) -> Self {
        ByteView::from(
            [
                &ByteView::from(value.addresstypeaddressindextxindex),
                value.vout.to_be_bytes().as_slice(),
            ]
            .concat(),
        )
    }
}
