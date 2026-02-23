use std::ops::{Add, Div};

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

pub(crate) fn get_percentile<T>(sorted: &[T], percentile: f64) -> T
where
    T: Clone + Div<usize, Output = T> + Add<T, Output = T>,
{
    let len = sorted.len();

    if len == 0 {
        panic!();
    } else if len == 1 {
        sorted[0].clone()
    } else {
        let index = (len - 1) as f64 * percentile;

        let fract = index.fract();

        if fract != 0.0 {
            let left = sorted.get(index as usize).unwrap().clone();
            let right = sorted.get(index.ceil() as usize).unwrap().clone();
            left / 2 + right / 2
        } else {
            // dbg!(sorted.len(), index);
            sorted.get(index as usize).unwrap().clone()
        }
    }
}
