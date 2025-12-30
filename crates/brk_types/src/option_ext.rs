/// Extension trait for Option to provide shorter unwrap methods
pub trait OptionExt<T> {
    /// Shorthand for `.as_ref().unwrap()`
    fn u(&self) -> &T;
    /// Shorthand for `.as_mut().unwrap()`
    fn um(&mut self) -> &mut T;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn u(&self) -> &T {
        self.as_ref().unwrap()
    }

    #[inline]
    fn um(&mut self) -> &mut T {
        self.as_mut().unwrap()
    }
}
