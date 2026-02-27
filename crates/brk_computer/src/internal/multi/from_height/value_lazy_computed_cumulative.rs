use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ByUnit, SatsToDollars},
    prices,
};

#[derive(Traversable)]
pub struct LazyComputedValueFromHeightCumulative<M: StorageMode = Rw> {
    pub base: ByUnit<M>,
    pub cumulative: ByUnit<M>,
}

const VERSION: Version = Version::ONE;

impl LazyComputedValueFromHeightCumulative {
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
        })
    }

    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.cumulative
            .sats
            .height
            .compute_cumulative(max_from, &self.base.sats.height, exit)?;

        self.base
            .usd
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

        Ok(())
    }
}
