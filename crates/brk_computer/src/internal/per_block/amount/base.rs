use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use vecdb::{AnyVec, Database, Exit, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        CentsUnsignedToDollars, ComputedPerBlock, LazyPerBlock, SatsToBitcoin, SatsToCents,
    },
    prices,
};

#[derive(Traversable)]
pub struct AmountPerBlock<M: StorageMode = Rw> {
    pub sats: ComputedPerBlock<Sats, M>,
    pub btc: LazyPerBlock<Bitcoin, Sats>,
    pub cents: ComputedPerBlock<Cents, M>,
    pub usd: LazyPerBlock<Dollars, Cents>,
}

impl AmountPerBlock {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let sats =
            ComputedPerBlock::forced_import(db, &format!("{name}_sats"), version, indexes)?;

        let btc = LazyPerBlock::from_computed::<SatsToBitcoin>(
            name,
            version,
            sats.height.read_only_boxed_clone(),
            &sats,
        );

        let cents =
            ComputedPerBlock::forced_import(db, &format!("{name}_cents"), version, indexes)?;

        let usd = LazyPerBlock::from_computed::<CentsUnsignedToDollars>(
            &format!("{name}_usd"),
            version,
            cents.height.read_only_boxed_clone(),
            &cents,
        );

        Ok(Self {
            sats,
            btc,
            cents,
            usd,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.sats.height.len()
    }

    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.cents.compute_binary::<Sats, Cents, SatsToCents>(
            max_from,
            &self.sats.height,
            &prices.spot.cents.height,
            exit,
        )?;
        Ok(())
    }

}

