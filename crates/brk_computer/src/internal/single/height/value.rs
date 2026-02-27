//! Value type with height-level data only (no period-derived views).
//!
//! Stores sats and USD per height, plus a lazy btc transform.
//! Use when period views are unnecessary (e.g., rolling windows provide windowed data).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{
    Database, EagerVec, Exit, ImportableVec, LazyVecFrom1, PcoVec, ReadableCloneableVec, Rw,
    StorageMode,
};

use crate::{internal::{SatsToBitcoin, SatsToDollars}, prices};

const VERSION: Version = Version::TWO; // Match ValueFromHeightLast versioning

#[derive(Traversable)]
pub struct ValueFromHeight<M: StorageMode = Rw> {
    pub sats: M::Stored<EagerVec<PcoVec<Height, Sats>>>,
    pub btc: LazyVecFrom1<Height, Bitcoin, Height, Sats>,
    pub usd: M::Stored<EagerVec<PcoVec<Height, Dollars>>>,
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
        let usd = EagerVec::forced_import(db, &format!("{name}_usd"), v)?;

        Ok(Self { sats, btc, usd })
    }

    /// Eagerly compute USD height values: sats[h] * price[h].
    pub(crate) fn compute_usd(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.usd.compute_binary::<Sats, Dollars, SatsToDollars>(
            max_from,
            &self.sats,
            &prices.price.usd,
            exit,
        )?;
        Ok(())
    }
}
