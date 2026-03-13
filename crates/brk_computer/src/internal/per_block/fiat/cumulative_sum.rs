use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{CachedWindowStarts, CentsType, FiatPerBlock, LazyRollingSumsFiatFromHeight},
};

#[derive(Traversable)]
pub struct FiatPerBlockCumulativeWithSums<C: CentsType, M: StorageMode = Rw> {
    pub base: FiatPerBlock<C, M>,
    pub cumulative: FiatPerBlock<C, M>,
    pub sum: LazyRollingSumsFiatFromHeight<C>,
}

impl<C: CentsType> FiatPerBlockCumulativeWithSums<C> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let base = FiatPerBlock::forced_import(db, name, version, indexes)?;
        let cumulative = FiatPerBlock::forced_import(
            db,
            &format!("{name}_cumulative"),
            version,
            indexes,
        )?;
        let sum = LazyRollingSumsFiatFromHeight::new(
            &format!("{name}_sum"),
            version,
            &cumulative.cents.height,
            cached_starts,
            indexes,
        );
        Ok(Self { base, cumulative, sum })
    }

    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()>
    where
        C: Default,
    {
        self.cumulative
            .cents
            .height
            .compute_cumulative(max_from, &self.base.cents.height, exit)?;
        Ok(())
    }
}
