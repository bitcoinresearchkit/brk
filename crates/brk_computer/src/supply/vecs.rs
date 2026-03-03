use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Dollars};
use vecdb::{Database, Rw, StorageMode};

use super::{burned, velocity};
use crate::internal::{
    ComputedFromHeight, LazyFromHeight, LazyValueFromHeight, PercentFromHeight,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub circulating: LazyValueFromHeight,
    pub burned: burned::Vecs<M>,
    pub inflation_rate: PercentFromHeight<BasisPointsSigned32, M>,
    pub velocity: velocity::Vecs<M>,
    pub market_cap: LazyFromHeight<Dollars>,
    pub market_cap_growth_rate: PercentFromHeight<BasisPointsSigned32, M>,
    pub realized_cap_growth_rate: PercentFromHeight<BasisPointsSigned32, M>,
    pub market_minus_realized_cap_growth_rate: ComputedFromHeight<BasisPointsSigned32, M>,
}
