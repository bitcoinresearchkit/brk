use brk_traversable::Traversable;
use brk_types::{Dollars, StoredF32};
use vecdb::{Database, Rw, StorageMode};

use super::{burned, velocity};
use crate::internal::{
    ComputedFromHeight, LazyFromHeight, LazyValueFromHeight,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub circulating: LazyValueFromHeight,
    pub burned: burned::Vecs<M>,
    pub inflation: ComputedFromHeight<StoredF32, M>,
    pub velocity: velocity::Vecs<M>,
    pub market_cap: LazyFromHeight<Dollars>,
    pub market_cap_growth_rate: ComputedFromHeight<StoredF32, M>,
    pub realized_cap_growth_rate: ComputedFromHeight<StoredF32, M>,
    pub cap_growth_rate_diff: ComputedFromHeight<StoredF32, M>,
}
