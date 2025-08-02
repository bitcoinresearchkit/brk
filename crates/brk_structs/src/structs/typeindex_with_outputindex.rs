use byteview::ByteView;
use serde::Serialize;

use super::{OutputIndex, TypeIndex};

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
        let typeindex = TypeIndex::from(&value[0..4]);
        let outputindex = OutputIndex::from(&value[4..12]);
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
