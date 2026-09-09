use brk_types::Dollars;

use crate::{Fiat, LazyPerBlock, LazyRollingSumFromHeight};

pub type LazyRollingSumFiatFromHeight<C> =
    Fiat<LazyRollingSumFromHeight<C>, LazyPerBlock<Dollars, C>>;
