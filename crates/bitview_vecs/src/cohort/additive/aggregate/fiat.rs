use crate::AggregateFiatPerBlock;
use vecdb::Rw;

// Additive callers publish all = STH + LTH with push_additive.
pub type AdditiveAggregateFiatPerBlock<C, M = Rw> = AggregateFiatPerBlock<C, M>;
