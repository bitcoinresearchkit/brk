use std::hash::{Hash, Hasher};

use byteview::ByteView;
use serde::Serialize;
use zerocopy::IntoBytes;

use crate::OutputType;

use super::{TxIndex, TypeIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize)]
pub struct AddressTypeAddressIndexTxIndex {
    addresstype: OutputType,
    addressindextxindex: u64,
}

impl Hash for AddressTypeAddressIndexTxIndex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut buf = [0u8; 9];
        buf[..1].copy_from_slice(self.addresstype.as_bytes());
        buf[1..].copy_from_slice(self.addressindextxindex.as_bytes());
        state.write(&buf);
    }
}

impl AddressTypeAddressIndexTxIndex {
    pub fn addresstype(&self) -> OutputType {
        self.addresstype
    }

    pub fn addressindex(&self) -> u32 {
        (self.addressindextxindex >> 32) as u32
    }

    pub fn txindex(&self) -> u32 {
        self.addressindextxindex as u32
    }

    pub fn addressindextxindex(&self) -> u64 {
        self.addressindextxindex
    }
}

impl From<(OutputType, TypeIndex, TxIndex)> for AddressTypeAddressIndexTxIndex {
    #[inline]
    fn from((addresstype, addressindex, txindex): (OutputType, TypeIndex, TxIndex)) -> Self {
        Self {
            addresstype,
            addressindextxindex: (u64::from(addressindex) << 32) | u64::from(txindex),
        }
    }
}

impl From<ByteView> for AddressTypeAddressIndexTxIndex {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self::from(&*value)
    }
}

impl From<&[u8]> for AddressTypeAddressIndexTxIndex {
    #[inline]
    fn from(value: &[u8]) -> Self {
        let addresstype = OutputType::from(&value[0..1]);
        let addressindex = TypeIndex::from(&value[1..5]);
        let txindex = TxIndex::from(&value[5..9]);
        Self::from((addresstype, addressindex, txindex))
    }
}

impl From<AddressTypeAddressIndexTxIndex> for ByteView {
    #[inline]
    fn from(value: AddressTypeAddressIndexTxIndex) -> Self {
        ByteView::from(&value)
    }
}
impl From<&AddressTypeAddressIndexTxIndex> for ByteView {
    #[inline]
    fn from(value: &AddressTypeAddressIndexTxIndex) -> Self {
        ByteView::from(
            [
                value.addresstype.as_bytes(),
                value.addressindextxindex.to_be_bytes().as_slice(),
            ]
            .concat(),
        )
    }
}
