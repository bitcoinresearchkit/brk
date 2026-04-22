use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        FiatType, FiatBlock, FiatPerBlock, LazyRollingSumsFiatFromHeight, WindowStartVec, Windows,
    },
};

#[derive(Traversable)]
pub struct FiatPerBlockCumulativeWithSums<C: FiatType, M: StorageMode = Rw> {
    pub block: FiatBlock<C, M>,
    pub cumulative: FiatPerBlock<C, M>,
    pub sum: LazyRollingSumsFiatFromHeight<C>,
}

impl<C: FiatType> FiatPerBlockCumulativeWithSums<C> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let block = FiatBlock::forced_import(db, name, version)?;
        let cumulative =
            FiatPerBlock::forced_import(db, &format!("{name}_cumulative"), version, indexes)?;
        let sum = LazyRollingSumsFiatFromHeight::new(
            &format!("{name}_sum"),
            version,
            &cumulative.cents.height,
            cached_starts,
            indexes,
        );
        Ok(Self {
            block,
            cumulative,
            sum,
        })
    }

    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()>
    where
        C: Default,
    {
        self.cumulative
            .cents
            .height
            .compute_cumulative(max_from, &self.block.cents, exit)?;
        Ok(())
    }
}
