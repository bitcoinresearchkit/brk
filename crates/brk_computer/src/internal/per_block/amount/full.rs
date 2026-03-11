use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{AmountPerBlock, RollingFullAmountPerBlock, SatsToCents, WindowStarts},
    prices,
};

#[derive(Traversable)]
pub struct AmountPerBlockFull<M: StorageMode = Rw> {
    pub base: AmountPerBlock<M>,
    pub cumulative: AmountPerBlock<M>,
    #[traversable(flatten)]
    pub rolling: RollingFullAmountPerBlock<M>,
}

const VERSION: Version = Version::TWO;

impl AmountPerBlockFull {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        Ok(Self {
            base: AmountPerBlock::forced_import(db, name, v, indexes)?,
            cumulative: AmountPerBlock::forced_import(
                db,
                &format!("{name}_cumulative"),
                v,
                indexes,
            )?,
            rolling: RollingFullAmountPerBlock::forced_import(db, name, v, indexes)?,
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
                &prices.spot.cents.height,
                exit,
            )?;

        self.cumulative
            .cents
            .height
            .compute_cumulative(max_from, &self.base.cents.height, exit)?;

        self.rolling.compute(
            max_from,
            windows,
            &self.base.sats.height,
            &self.base.cents.height,
            exit,
        )?;

        Ok(())
    }
}
