use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ByUnit, RollingSumByUnit, SatsToDollars, WindowStarts},
    prices,
};

#[derive(Traversable)]
pub struct ValueFromHeightSumCumulative<M: StorageMode = Rw> {
    pub base: ByUnit<M>,
    pub cumulative: ByUnit<M>,
    pub sum: RollingSumByUnit<M>,
}

const VERSION: Version = Version::TWO;

impl ValueFromHeightSumCumulative {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        Ok(Self {
            base: ByUnit::forced_import(db, name, v, indexes)?,
            cumulative: ByUnit::forced_import(db, &format!("{name}_cumulative"), v, indexes)?,
            sum: RollingSumByUnit::forced_import(db, name, v, indexes)?,
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
            .usd
            .height
            .compute_binary::<Sats, Dollars, SatsToDollars>(
                max_from,
                &self.base.sats.height,
                &prices.price.usd,
                exit,
            )?;

        self.cumulative
            .usd
            .height
            .compute_cumulative(max_from, &self.base.usd.height, exit)?;

        self.sum.compute_rolling_sum(
            max_from,
            windows,
            &self.base.sats.height,
            &self.base.usd.height,
            exit,
        )?;

        Ok(())
    }
}
