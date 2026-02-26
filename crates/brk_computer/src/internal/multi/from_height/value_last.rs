//! Value type for Last pattern from Height.
//!
//! Height-level USD value is stored (eagerly computed from sats × price).
//! Day1 last is stored since it requires finding the last value within each date.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{Database, Exit, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    indexes, prices,
    internal::{ComputedFromHeightLast, LazyFromHeightLast, SatsToBitcoin, SatsToDollars},
};

#[derive(Traversable)]
pub struct ValueFromHeightLast<M: StorageMode = Rw> {
    pub sats: ComputedFromHeightLast<Sats, M>,
    pub btc: LazyFromHeightLast<Bitcoin, Sats>,
    pub usd: ComputedFromHeightLast<Dollars, M>,
}

const VERSION: Version = Version::TWO; // Bumped for stored height dollars

impl ValueFromHeightLast {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedFromHeightLast::forced_import(db, name, v, indexes)?;

        let btc = LazyFromHeightLast::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.read_only_boxed_clone(),
            &sats,
        );

        let usd = ComputedFromHeightLast::forced_import(db, &format!("{name}_usd"), v, indexes)?;

        Ok(Self {
            sats,
            btc,
            usd,
        })
    }

    /// Eagerly compute USD height values: sats[h] * price[h].
    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.usd.compute_binary::<Sats, Dollars, SatsToDollars>(
            max_from,
            &self.sats.height,
            &prices.usd.price,
            exit,
        )?;
        Ok(())
    }
}
