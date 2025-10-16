use byteview::ByteView;
use serde::Serialize;

use super::{OutPoint, TypeIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Serialize)]
pub struct TypeIndexAndOutPoint {
    typeindex: TypeIndex,
    outpoint: OutPoint,
}

impl From<(TypeIndex, OutPoint)> for TypeIndexAndOutPoint {
    fn from(value: (TypeIndex, OutPoint)) -> Self {
        Self {
            typeindex: value.0,
            outpoint: value.1,
        }
    }
}

impl From<ByteView> for TypeIndexAndOutPoint {
    fn from(value: ByteView) -> Self {
        Self {
            typeindex: TypeIndex::from(&value[0..4]),
            outpoint: OutPoint::from(&value[4..]),
        }
    }
}

impl From<TypeIndexAndOutPoint> for ByteView {
    fn from(value: TypeIndexAndOutPoint) -> Self {
        ByteView::from(&value)
    }
}
impl From<&TypeIndexAndOutPoint> for ByteView {
    fn from(value: &TypeIndexAndOutPoint) -> Self {
        ByteView::from(
            [
                u32::from(value.typeindex).to_be_bytes().as_slice(),
                value.outpoint.to_be_bytes().as_slice(),
            ]
            .concat(),
        )
    }
}
