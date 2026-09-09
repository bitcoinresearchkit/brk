use brk_types::Dollars;
use vecdb::DeltaChange;

use crate::{Fiat, LazyDeltaFromHeight, LazyPerBlock};

pub type LazyDeltaFiatFromHeight<S, C> =
    Fiat<LazyDeltaFromHeight<S, C, DeltaChange>, LazyPerBlock<Dollars, C>>;
