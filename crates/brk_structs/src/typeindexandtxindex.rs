use byteview::ByteView;
use serde::Serialize;

use super::{TxIndex, TypeIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Serialize)]
pub struct TypeIndexAndTxIndex {
    typeindex: TypeIndex,
    txindex: TxIndex,
}

impl From<(TypeIndex, TxIndex)> for TypeIndexAndTxIndex {
    fn from(value: (TypeIndex, TxIndex)) -> Self {
        Self {
            typeindex: value.0,
            txindex: value.1,
        }
    }
}

impl From<ByteView> for TypeIndexAndTxIndex {
    fn from(value: ByteView) -> Self {
        let typeindex = TypeIndex::from(&value[0..4]);
        let txindex = TxIndex::from(&value[4..8]);
        Self { typeindex, txindex }
    }
}

impl From<TypeIndexAndTxIndex> for ByteView {
    fn from(value: TypeIndexAndTxIndex) -> Self {
        ByteView::from(&value)
    }
}
impl From<&TypeIndexAndTxIndex> for ByteView {
    fn from(value: &TypeIndexAndTxIndex) -> Self {
        ByteView::from(
            [
                u32::from(value.typeindex).to_be_bytes().as_slice(),
                u32::from(value.txindex).to_be_bytes().as_slice(),
            ]
            .concat(),
        )
    }
}
