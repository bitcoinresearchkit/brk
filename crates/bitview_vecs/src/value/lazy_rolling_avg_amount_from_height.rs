use brk_types::{Bitcoin, Cents, Dollars, Sats, StoredF32};

use crate::{LazyPerBlock, LazyRollingAvgFromHeight, Value};

pub type LazyRollingAvgAmountFromHeight = Value<
    LazyRollingAvgFromHeight<Sats>,
    LazyRollingAvgFromHeight<Cents>,
    LazyPerBlock<Bitcoin, StoredF32>,
    LazyPerBlock<Dollars, StoredF32>,
>;
