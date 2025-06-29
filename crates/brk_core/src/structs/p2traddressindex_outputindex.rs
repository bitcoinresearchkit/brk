use byteview::ByteView;
use serde::Serialize;

use crate::{TypeIndex, copy_first_4bytes, copy_first_8bytes};

use super::{OutputIndex, P2TRAddressIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Serialize)]
pub struct P2TRAddressIndexOutputindex {
    addressindex: P2TRAddressIndex,
    outputindex: OutputIndex,
}

impl From<(TypeIndex, OutputIndex)> for P2TRAddressIndexOutputindex {
    fn from(value: (TypeIndex, OutputIndex)) -> Self {
        Self::from((P2TRAddressIndex::from(value.0), value.1))
    }
}

impl From<(P2TRAddressIndex, OutputIndex)> for P2TRAddressIndexOutputindex {
    fn from(value: (P2TRAddressIndex, OutputIndex)) -> Self {
        Self {
            addressindex: value.0,
            outputindex: value.1,
        }
    }
}

impl From<ByteView> for P2TRAddressIndexOutputindex {
    fn from(value: ByteView) -> Self {
        let addressindex =
            P2TRAddressIndex::from(u32::from_be_bytes(copy_first_4bytes(&value).unwrap()));
        let outputindex = OutputIndex::from(u64::from_be_bytes(copy_first_8bytes(&value).unwrap()));
        Self {
            addressindex,
            outputindex,
        }
    }
}
impl From<P2TRAddressIndexOutputindex> for ByteView {
    fn from(value: P2TRAddressIndexOutputindex) -> Self {
        ByteView::from(&value)
    }
}
impl From<&P2TRAddressIndexOutputindex> for ByteView {
    fn from(value: &P2TRAddressIndexOutputindex) -> Self {
        ByteView::from(
            [
                u32::from(value.addressindex).to_be_bytes().as_slice(),
                u64::from(value.outputindex).to_be_bytes().as_slice(),
            ]
            .concat(),
        )
    }
}
