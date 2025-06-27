use byteview::ByteView;
use serde::Serialize;

use crate::{copy_first_4bytes, copy_first_8bytes};

use super::{OutputIndex, P2PKHAddressIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Serialize)]
pub struct P2PKHAddressIndexOutputindex {
    addressindex: P2PKHAddressIndex,
    outputindex: OutputIndex,
}

impl From<ByteView> for P2PKHAddressIndexOutputindex {
    fn from(value: ByteView) -> Self {
        let addressindex =
            P2PKHAddressIndex::from(u32::from_be_bytes(copy_first_4bytes(&value).unwrap()));
        let outputindex = OutputIndex::from(u64::from_be_bytes(copy_first_8bytes(&value).unwrap()));
        Self {
            addressindex,
            outputindex,
        }
    }
}
impl From<P2PKHAddressIndexOutputindex> for ByteView {
    fn from(value: P2PKHAddressIndexOutputindex) -> Self {
        ByteView::from(&value)
    }
}
impl From<&P2PKHAddressIndexOutputindex> for ByteView {
    fn from(value: &P2PKHAddressIndexOutputindex) -> Self {
        ByteView::from(
            [
                u32::from(value.addressindex).to_be_bytes().as_slice(),
                u64::from(value.outputindex).to_be_bytes().as_slice(),
            ]
            .concat(),
        )
    }
}
