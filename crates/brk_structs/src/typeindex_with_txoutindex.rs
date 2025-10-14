use byteview::ByteView;
use serde::Serialize;

use super::{TxOutIndex, TypeIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Serialize)]
pub struct TypeIndexWithOutputindex {
    typeindex: TypeIndex,
    txoutindex: TxOutIndex,
}

impl From<(TypeIndex, TxOutIndex)> for TypeIndexWithOutputindex {
    fn from(value: (TypeIndex, TxOutIndex)) -> Self {
        Self {
            typeindex: value.0,
            txoutindex: value.1,
        }
    }
}

impl From<ByteView> for TypeIndexWithOutputindex {
    fn from(value: ByteView) -> Self {
        let typeindex = TypeIndex::from(&value[0..4]);
        let txoutindex = TxOutIndex::from(&value[4..12]);
        Self {
            typeindex,
            txoutindex,
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
                u64::from(value.txoutindex).to_be_bytes().as_slice(),
            ]
            .concat(),
        )
    }
}
