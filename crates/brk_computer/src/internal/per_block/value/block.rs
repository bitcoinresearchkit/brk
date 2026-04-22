use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use vecdb::{
    Database, EagerVec, Exit, ImportableVec, LazyVecFrom1, PcoVec, ReadableCloneableVec, Rw,
    StorageMode,
};

use crate::{
    internal::{CentsUnsignedToDollars, SatsToBitcoin, SatsToCents},
    prices,
};

/// Raw per-block amount data: sats + cents (stored), btc + usd (lazy), no resolutions.
#[derive(Traversable)]
pub struct ValueBlock<M: StorageMode = Rw> {
    pub btc: LazyVecFrom1<Height, Bitcoin, Height, Sats>,
    pub sats: M::Stored<EagerVec<PcoVec<Height, Sats>>>,
    pub usd: LazyVecFrom1<Height, Dollars, Height, Cents>,
    pub cents: M::Stored<EagerVec<PcoVec<Height, Cents>>>,
}

impl ValueBlock {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        let sats: EagerVec<PcoVec<Height, Sats>> =
            EagerVec::forced_import(db, &format!("{name}_sats"), version)?;
        let btc =
            LazyVecFrom1::transformed::<SatsToBitcoin>(name, version, sats.read_only_boxed_clone());
        let cents: EagerVec<PcoVec<Height, Cents>> =
            EagerVec::forced_import(db, &format!("{name}_cents"), version)?;
        let usd = LazyVecFrom1::transformed::<CentsUnsignedToDollars>(
            &format!("{name}_usd"),
            version,
            cents.read_only_boxed_clone(),
        );
        Ok(Self {
            btc,
            sats,
            usd,
            cents,
        })
    }

    pub(crate) fn compute_cents(
        &mut self,
        max_from: Height,
        prices: &prices::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.cents.compute_binary::<Sats, Cents, SatsToCents>(
            max_from,
            &self.sats,
            &prices.spot.cents.height,
            exit,
        )?;
        Ok(())
    }
}
