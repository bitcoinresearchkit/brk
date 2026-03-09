use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{AmountFromHeight, RollingSumAmountFromHeight, SatsToCents, WindowStarts},
    prices,
};

#[derive(Traversable)]
pub struct AmountFromHeightCumulativeSum<M: StorageMode = Rw> {
    pub base: AmountFromHeight<M>,
    pub cumulative: AmountFromHeight<M>,
    pub sum: RollingSumAmountFromHeight<M>,
}

const VERSION: Version = Version::TWO;

impl AmountFromHeightCumulativeSum {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        Ok(Self {
            base: AmountFromHeight::forced_import(db, name, v, indexes)?,
            cumulative: AmountFromHeight::forced_import(
                db,
                &format!("{name}_cumulative"),
                v,
                indexes,
            )?,
            sum: RollingSumAmountFromHeight::forced_import(db, name, v, indexes)?,
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
        compute_sats(&mut self.base.sats.height)?;

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
                &prices.price.cents.height,
                exit,
            )?;

        self.cumulative
            .cents
            .height
            .compute_cumulative(max_from, &self.base.cents.height, exit)?;

        self.sum.compute_rolling_sum(
            max_from,
            windows,
            &self.base.sats.height,
            &self.base.cents.height,
            exit,
        )?;

        Ok(())
    }
}
