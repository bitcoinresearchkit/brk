use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use vecdb::{Database, Exit, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{CentsUnsignedToDollars, LazyPerBlock, PerBlock, SatsToBitcoin, SatsToCents},
    prices,
};

#[derive(Traversable)]
pub struct AmountPerBlock<M: StorageMode = Rw> {
    pub btc: LazyPerBlock<Bitcoin, Sats>,
    pub sats: PerBlock<Sats, M>,
    pub usd: LazyPerBlock<Dollars, Cents>,
    pub cents: PerBlock<Cents, M>,
}

impl AmountPerBlock {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let sats = PerBlock::forced_import(db, &format!("{name}_sats"), version, indexes)?;

        let btc = LazyPerBlock::from_computed::<SatsToBitcoin>(
            name,
            version,
            sats.height.read_only_boxed_clone(),
            &sats,
        );

        let cents = PerBlock::forced_import(db, &format!("{name}_cents"), version, indexes)?;

        let usd = LazyPerBlock::from_computed::<CentsUnsignedToDollars>(
            &format!("{name}_usd"),
            version,
            cents.height.read_only_boxed_clone(),
            &cents,
        );

        Ok(Self {
            btc,
            sats,
            usd,
            cents,
        })
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
