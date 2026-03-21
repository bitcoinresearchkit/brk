use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{AmountBlock, AmountPerBlock},
    prices,
};

#[derive(Traversable)]
pub struct AmountPerBlockCumulative<M: StorageMode = Rw> {
    pub block: AmountBlock<M>,
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
            block: AmountBlock::forced_import(db, name, v)?,
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
            .compute_cumulative(max_from, &self.block.sats, exit)?;

        self.block.compute_cents(max_from, prices, exit)?;

        self.cumulative
            .cents
            .height
            .compute_cumulative(max_from, &self.block.cents, exit)?;

        Ok(())
    }

    pub(crate) fn compute_with(
        &mut self,
        max_from: Height,
        prices: &prices::Vecs,
        exit: &Exit,
        compute_sats: impl FnOnce(&mut EagerVec<PcoVec<Height, Sats>>) -> Result<()>,
    ) -> Result<()> {
        compute_sats(&mut self.block.sats)?;
        self.compute(prices, max_from, exit)
    }
}
