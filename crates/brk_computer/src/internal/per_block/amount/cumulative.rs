use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Sats, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{AmountPerBlock, SatsToCents},
    prices,
};

#[derive(Traversable)]
pub struct AmountPerBlockCumulative<M: StorageMode = Rw> {
    pub base: AmountPerBlock<M>,
    pub cumulative: AmountPerBlock<M>,
}

const VERSION: Version = Version::ONE;

impl AmountPerBlockCumulative {
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

        self.base.cents.compute_binary::<Sats, Cents, SatsToCents>(
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
