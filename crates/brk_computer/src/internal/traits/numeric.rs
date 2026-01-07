use super::ComputedVecValue;

pub trait NumericValue: ComputedVecValue + From<f64> + Into<f64> {}

impl<T> NumericValue for T where T: ComputedVecValue + From<f64> + Into<f64> {}
