use brk_traversable::Traversable;
use brk_types::{Cents, Height, OHLCCents};
use vecdb::{PcoVec, Rw, StorageMode};

use crate::internal::{ComputedHeightDerivedLast, EagerIndexes};
use crate::prices::{ohlcs::OhlcVecs, split::SplitOhlc};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[allow(clippy::type_complexity)]
    pub split: SplitOhlc<
        EagerIndexes<Cents, M>,
        EagerIndexes<Cents, M>,
        EagerIndexes<Cents, M>,
        ComputedHeightDerivedLast<Cents>,
    >,
    pub ohlc: OhlcVecs<OHLCCents, M>,
    pub price: M::Stored<PcoVec<Height, Cents>>,
}
