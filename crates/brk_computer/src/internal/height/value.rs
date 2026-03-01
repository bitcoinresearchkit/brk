//! Value type with height-level data only (no period-derived views).
//!
//! Stores sats and cents per height, plus lazy btc and usd transforms.
//! Use when period views are unnecessary (e.g., rolling windows provide windowed data).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use vecdb::{
    Database, EagerVec, Exit, ImportableVec, LazyVecFrom1, PcoVec, ReadableCloneableVec, Rw,
    StorageMode,
};

use crate::{internal::{CentsUnsignedToDollars, SatsToBitcoin, SatsToCents}, prices};

const VERSION: Version = Version::TWO; // Match ValueFromHeightLast versioning

#[derive(Traversable)]
pub struct ValueFromHeight<M: StorageMode = Rw> {
    pub sats: M::Stored<EagerVec<PcoVec<Height, Sats>>>,
    pub btc: LazyVecFrom1<Height, Bitcoin, Height, Sats>,
    pub cents: M::Stored<EagerVec<PcoVec<Height, Cents>>>,
    pub usd: LazyVecFrom1<Height, Dollars, Height, Cents>,
}

impl ValueFromHeight {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats: EagerVec<PcoVec<Height, Sats>> = EagerVec::forced_import(db, name, v)?;
        let btc = LazyVecFrom1::transformed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.read_only_boxed_clone(),
        );
        let cents: EagerVec<PcoVec<Height, Cents>> =
            EagerVec::forced_import(db, &format!("{name}_cents"), v)?;
        let usd = LazyVecFrom1::transformed::<CentsUnsignedToDollars>(
            &format!("{name}_usd"),
            v,
            cents.read_only_boxed_clone(),
        );

        Ok(Self { sats, btc, cents, usd })
    }

    /// Eagerly compute cents height values: sats[h] * price_cents[h] / 1e8.
    pub(crate) fn compute_cents(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.cents.compute_binary::<Sats, Cents, SatsToCents>(
            max_from,
            &self.sats,
            &prices.price.cents.height,
            exit,
        )?;
        Ok(())
    }
}
