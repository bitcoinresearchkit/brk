use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        AmountPerBlockCumulative, CachedWindowStarts, LazyRollingAvgsAmountFromHeight,
        LazyRollingSumsAmountFromHeight,
    },
    prices,
};

#[derive(Deref, DerefMut, Traversable)]
pub struct AmountPerBlockCumulativeRolling<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: AmountPerBlockCumulative<M>,
    pub sum: LazyRollingSumsAmountFromHeight,
    pub average: LazyRollingAvgsAmountFromHeight,
}

const VERSION: Version = Version::TWO;

impl AmountPerBlockCumulativeRolling {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let v = version + VERSION;

        let inner = AmountPerBlockCumulative::forced_import(db, name, v, indexes)?;
        let sum = LazyRollingSumsAmountFromHeight::new(
            &format!("{name}_sum"),
            v,
            &inner.cumulative.sats.height,
            &inner.cumulative.cents.height,
            cached_starts,
            indexes,
        );
        let average = LazyRollingAvgsAmountFromHeight::new(
            &format!("{name}_average"),
            v,
            &inner.cumulative.sats.height,
            &inner.cumulative.cents.height,
            cached_starts,
            indexes,
        );

        Ok(Self {
            inner,
            sum,
            average,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        prices: &prices::Vecs,
        exit: &Exit,
        compute_sats: impl FnOnce(&mut EagerVec<PcoVec<Height, Sats>>) -> Result<()>,
    ) -> Result<()> {
        compute_sats(&mut self.base.sats.height)?;
        self.compute_rest(max_from, prices, exit)
    }

    pub(crate) fn compute_rest(
        &mut self,
        max_from: Height,
        prices: &prices::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.inner.compute(prices, max_from, exit)
    }
}
