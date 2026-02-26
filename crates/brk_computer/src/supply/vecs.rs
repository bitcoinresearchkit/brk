use brk_traversable::Traversable;
use brk_types::{Dollars, StoredF32};
use vecdb::{Database, Rw, StorageMode};

use super::{burned, velocity};
use crate::internal::{
    ComputedFromHeightLast, LazyFromHeightLast, LazyValueFromHeightLast,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub circulating: LazyValueFromHeightLast,
    pub burned: burned::Vecs<M>,
    pub inflation: ComputedFromHeightLast<StoredF32, M>,
    pub velocity: velocity::Vecs<M>,
    pub market_cap: LazyFromHeightLast<Dollars>,
    pub market_cap_growth_rate: ComputedFromHeightLast<StoredF32, M>,
    pub realized_cap_growth_rate: ComputedFromHeightLast<StoredF32, M>,
    pub cap_growth_rate_diff: ComputedFromHeightLast<StoredF32, M>,
}
