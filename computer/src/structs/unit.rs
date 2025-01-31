use fjall::Slice;

pub struct Unit();
impl From<Slice> for Unit {
    fn from(_: Slice) -> Self {
        Self()
    }
}
impl From<Unit> for Slice {
    fn from(_: Unit) -> Self {
        Self::new(&[])
    }
}
