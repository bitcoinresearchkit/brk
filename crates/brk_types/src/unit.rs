use byteview::ByteView;

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
