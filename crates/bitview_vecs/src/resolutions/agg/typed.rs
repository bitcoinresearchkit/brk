use vecdb::{TypedVec, VecIndex, VecValue};

use super::LazyAggVec;

impl<I, O, S1I, S1T, Strat> TypedVec for LazyAggVec<I, O, S1I, S1T, Strat>
where
    I: VecIndex,
    O: VecValue,
    S1I: VecIndex,
    S1T: VecValue,
    Strat: 'static,
{
    type I = I;
    type T = O;
}
