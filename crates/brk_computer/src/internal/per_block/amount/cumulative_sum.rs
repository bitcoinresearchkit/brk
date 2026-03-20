use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        AmountPerBlock, CachedWindowStarts, LazyRollingAvgsAmountFromHeight,
        LazyRollingSumsAmountFromHeight, SatsToCents,
    },
    prices,
};

#[derive(Traversable)]
pub struct AmountPerBlockCumulativeWithSums<M: StorageMode = Rw> {
    pub base: AmountPerBlock<M>,
    pub cumulative: AmountPerBlock<M>,
    pub sum: LazyRollingSumsAmountFromHeight,
    pub average: LazyRollingAvgsAmountFromHeight,
}

const VERSION: Version = Version::TWO;

impl AmountPerBlockCumulativeWithSums {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let v = version + VERSION;

        let base = AmountPerBlock::forced_import(db, name, v, indexes)?;
        let cumulative =
            AmountPerBlock::forced_import(db, &format!("{name}_cumulative"), v, indexes)?;
        let sum = LazyRollingSumsAmountFromHeight::new(
            &format!("{name}_sum"),
            v,
            &cumulative.sats.height,
            &cumulative.cents.height,
            cached_starts,
            indexes,
        );
        let average = LazyRollingAvgsAmountFromHeight::new(
            &format!("{name}_average"),
            v,
            &cumulative.sats.height,
            &cumulative.cents.height,
            cached_starts,
            indexes,
        );

        Ok(Self {
            base,
            cumulative,
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
        self.cumulative
            .sats
            .height
            .compute_cumulative(max_from, &self.base.sats.height, exit)?;

        self.base
            .cents
            .height
            .compute_binary::<Sats, Cents, SatsToCents>(
                max_from,
                &self.base.sats.height,
                &prices.spot.cents.height,
                exit,
            )?;

        self.cumulative
            .cents
            .height
            .compute_cumulative(max_from, &self.base.cents.height, exit)?;

        Ok(())
    }
}
