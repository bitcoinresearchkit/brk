use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ByUnit, RollingFullByUnit, SatsToDollars, WindowStarts},
    prices,
};

#[derive(Traversable)]
pub struct ValueFromHeightFull<M: StorageMode = Rw> {
    pub base: ByUnit<M>,
    pub cumulative: ByUnit<M>,
    #[traversable(flatten)]
    pub rolling: RollingFullByUnit<M>,
}

const VERSION: Version = Version::TWO;

impl ValueFromHeightFull {
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
            rolling: RollingFullByUnit::forced_import(db, name, v, indexes)?,
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
                &prices.price.usd.height,
                exit,
            )?;

        self.cumulative
            .usd
            .height
            .compute_cumulative(max_from, &self.base.usd.height, exit)?;

        self.rolling.compute(
            max_from,
            windows,
            &self.base.sats.height,
            &self.base.usd.height,
            exit,
        )?;

        Ok(())
    }
}
