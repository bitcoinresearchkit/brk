use byteview::ByteView;
use serde::Serialize;

use crate::{copy_first_4bytes, copy_first_8bytes};

use super::{OutputIndex, P2AAddressIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Serialize)]
pub struct P2AAddressIndexOutputindex {
    addressindex: P2AAddressIndex,
    outputindex: OutputIndex,
}

impl From<(P2AAddressIndex, OutputIndex)> for P2AAddressIndexOutputindex {
    fn from(value: (P2AAddressIndex, OutputIndex)) -> Self {
        Self {
            addressindex: value.0,
            outputindex: value.1,
        }
    }
}

impl From<ByteView> for P2AAddressIndexOutputindex {
    fn from(value: ByteView) -> Self {
        let addressindex =
            P2AAddressIndex::from(u32::from_be_bytes(copy_first_4bytes(&value).unwrap()));
        let outputindex = OutputIndex::from(u64::from_be_bytes(copy_first_8bytes(&value).unwrap()));
        Self {
            addressindex,
            outputindex,
        }
    }
}
impl From<P2AAddressIndexOutputindex> for ByteView {
    fn from(value: P2AAddressIndexOutputindex) -> Self {
        ByteView::from(&value)
    }
}
impl From<&P2AAddressIndexOutputindex> for ByteView {
    fn from(value: &P2AAddressIndexOutputindex) -> Self {
        ByteView::from(
            [
                u32::from(value.addressindex).to_be_bytes().as_slice(),
                u64::from(value.outputindex).to_be_bytes().as_slice(),
            ]
            .concat(),
        )
    }
}
