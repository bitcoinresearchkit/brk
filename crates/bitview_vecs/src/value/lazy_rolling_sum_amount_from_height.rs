use brk_types::{Bitcoin, Cents, Dollars, Sats};

use crate::{LazyPerBlock, LazyRollingSumFromHeight, Value};

pub type LazyRollingSumAmountFromHeight = Value<
    LazyRollingSumFromHeight<Sats>,
    LazyRollingSumFromHeight<Cents>,
    LazyPerBlock<Bitcoin, Sats>,
    LazyPerBlock<Dollars, Cents>,
>;
