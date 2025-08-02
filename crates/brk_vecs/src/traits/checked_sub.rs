pub trait CheckedSub<Rhs = Self>: Sized {
    fn checked_sub(self, rhs: Rhs) -> Option<Self>;
}
