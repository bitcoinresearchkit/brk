use std::ops::Div;

use super::UnaryTransform;

/// v -> v / 2
pub struct Halve;

impl<T: Div<usize, Output = T>> UnaryTransform<T> for Halve {
    #[inline(always)]
    fn apply(value: T) -> T {
        value / 2
    }
}
