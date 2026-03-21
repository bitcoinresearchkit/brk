use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        AmountPerBlockCumulativeRolling, CachedWindowStarts, RollingDistributionAmountPerBlock,
        WindowStarts,
    },
    prices,
};

#[derive(Deref, DerefMut, Traversable)]
pub struct AmountPerBlockFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: AmountPerBlockCumulativeRolling<M>,
    #[traversable(flatten)]
    pub distribution: RollingDistributionAmountPerBlock<M>,
}

const VERSION: Version = Version::TWO;

impl AmountPerBlockFull {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let v = version + VERSION;

        let inner =
            AmountPerBlockCumulativeRolling::forced_import(db, name, v, indexes, cached_starts)?;
        let distribution =
            RollingDistributionAmountPerBlock::forced_import(db, name, v, indexes)?;

        Ok(Self {
            inner,
            distribution,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        prices: &prices::Vecs,
        exit: &Exit,
        compute_sats: impl FnOnce(&mut EagerVec<PcoVec<Height, Sats>>) -> Result<()>,
    ) -> Result<()> {
        compute_sats(&mut self.inner.block.sats)?;

        self.inner.compute_rest(max_from, prices, exit)?;

        self.distribution.compute(
            max_from,
            windows,
            &self.inner.block.sats,
            &self.inner.block.cents,
            exit,
        )?;

        Ok(())
    }
}
