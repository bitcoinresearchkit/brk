use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes, prices,
    internal::{ByUnit, SatsToCents},
};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct ValueFromHeight<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    pub base: ByUnit<M>,
}

const VERSION: Version = Version::TWO;

impl ValueFromHeight {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        Ok(Self {
            base: ByUnit::forced_import(db, name, v, indexes)?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.base.cents.compute_binary::<Sats, Cents, SatsToCents>(
            max_from,
            &self.base.sats.height,
            &prices.price.cents.height,
            exit,
        )?;
        Ok(())
    }

    pub(crate) fn compute_rolling_sum(
        &mut self,
        max_from: Height,
        window_starts: &impl ReadableVec<Height, Height>,
        sats_source: &impl ReadableVec<Height, Sats>,
        cents_source: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        self.base
            .sats
            .height
            .compute_rolling_sum(max_from, window_starts, sats_source, exit)?;
        self.base
            .cents
            .height
            .compute_rolling_sum(max_from, window_starts, cents_source, exit)?;
        Ok(())
    }

    pub(crate) fn compute_ema(
        &mut self,
        starting_height: Height,
        window_starts: &impl ReadableVec<Height, Height>,
        sats_source: &impl ReadableVec<Height, Sats>,
        cents_source: &(impl ReadableVec<Height, Cents> + Sync),
        exit: &Exit,
    ) -> Result<()> {
        self.base
            .sats
            .height
            .compute_rolling_ema(starting_height, window_starts, sats_source, exit)?;
        self.base
            .cents
            .height
            .compute_rolling_ema(starting_height, window_starts, cents_source, exit)?;
        Ok(())
    }
}
