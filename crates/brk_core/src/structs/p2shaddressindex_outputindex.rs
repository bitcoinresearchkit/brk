use byteview::ByteView;
use serde::Serialize;

use crate::{copy_first_4bytes, copy_first_8bytes};

use super::{OutputIndex, P2SHAddressIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Serialize)]
pub struct P2SHAddressIndexOutputindex {
    addressindex: P2SHAddressIndex,
    outputindex: OutputIndex,
}

impl From<ByteView> for P2SHAddressIndexOutputindex {
    fn from(value: ByteView) -> Self {
        let addressindex =
            P2SHAddressIndex::from(u32::from_be_bytes(copy_first_4bytes(&value).unwrap()));
        let outputindex = OutputIndex::from(u64::from_be_bytes(copy_first_8bytes(&value).unwrap()));
        Self {
            addressindex,
            outputindex,
        }
    }
}
impl From<P2SHAddressIndexOutputindex> for ByteView {
    fn from(value: P2SHAddressIndexOutputindex) -> Self {
        ByteView::from(&value)
    }
}
impl From<&P2SHAddressIndexOutputindex> for ByteView {
    fn from(value: &P2SHAddressIndexOutputindex) -> Self {
        ByteView::from(
            [
                u32::from(value.addressindex).to_be_bytes().as_slice(),
                u64::from(value.outputindex).to_be_bytes().as_slice(),
            ]
            .concat(),
        )
    }
}
