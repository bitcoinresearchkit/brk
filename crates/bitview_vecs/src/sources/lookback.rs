use crate::LazyWindowStartVec;

pub trait Lookback {
    fn start_vec(&self, days: usize) -> &LazyWindowStartVec;
}
