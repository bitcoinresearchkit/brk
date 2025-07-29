use byteview::ByteView;
use serde::Serialize;

use crate::{TypeIndex, copy_first_4bytes, copy_first_8bytes};

use super::OutputIndex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Serialize)]
pub struct TypeIndexWithOutputindex {
    typeindex: TypeIndex,
    outputindex: OutputIndex,
}

impl From<(TypeIndex, OutputIndex)> for TypeIndexWithOutputindex {
    fn from(value: (TypeIndex, OutputIndex)) -> Self {
        Self {
            typeindex: value.0,
            outputindex: value.1,
        }
    }
}

impl From<ByteView> for TypeIndexWithOutputindex {
    fn from(value: ByteView) -> Self {
        let typeindex = TypeIndex::from(u32::from_be_bytes(copy_first_4bytes(&value).unwrap()));
        let outputindex = OutputIndex::from(u64::from_be_bytes(copy_first_8bytes(&value).unwrap()));
        Self {
            typeindex,
            outputindex,
        }
    }
}

impl From<TypeIndexWithOutputindex> for ByteView {
    fn from(value: TypeIndexWithOutputindex) -> Self {
        ByteView::from(&value)
    }
}
impl From<&TypeIndexWithOutputindex> for ByteView {
    fn from(value: &TypeIndexWithOutputindex) -> Self {
        ByteView::from(
            [
                u32::from(value.typeindex).to_be_bytes().as_slice(),
                u64::from(value.outputindex).to_be_bytes().as_slice(),
            ]
            .concat(),
        )
    }
}
