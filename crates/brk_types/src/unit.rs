use byteview::ByteView;
use redb::{TypeName, Value};

#[derive(Debug, Clone)]
pub struct Unit;

impl From<ByteView> for Unit {
    #[inline]
    fn from(_: ByteView) -> Self {
        Self
    }
}
impl From<Unit> for ByteView {
    #[inline]
    fn from(_: Unit) -> Self {
        Self::new(&[])
    }
}

impl Value for Unit {
    type SelfType<'a>
        = Unit
    where
        Self: 'a;
    type AsBytes<'a>
        = &'a [u8]
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        Some(0)
    }

    #[allow(clippy::unused_unit, clippy::semicolon_if_nothing_returned)]
    fn from_bytes<'a>(_data: &'a [u8]) -> Unit
    where
        Self: 'a,
    {
        Unit
    }

    #[allow(clippy::ignored_unit_patterns)]
    fn as_bytes<'a, 'b: 'a>(_: &'a Self::SelfType<'b>) -> &'a [u8]
    where
        Self: 'b,
    {
        &[]
    }

    fn type_name() -> TypeName {
        TypeName::new("Unit")
    }
}
