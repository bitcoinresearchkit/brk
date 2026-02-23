//! Value type for Last pattern from Height.
//!
//! Height-level USD value is lazy: `sats * price`.
//! Day1 last is stored since it requires finding the last value within each date.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Sats, Version};
use vecdb::{Database, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        ComputedFromHeightLast, LazyBinaryComputedFromHeightLast, LazyFromHeightLast,
        SatsTimesPrice, SatsToBitcoin,
    },
    prices,
};

#[derive(Traversable)]
pub struct ValueFromHeightLast<M: StorageMode = Rw> {
    pub sats: ComputedFromHeightLast<Sats, M>,
    pub btc: LazyFromHeightLast<Bitcoin, Sats>,
    pub usd: LazyBinaryComputedFromHeightLast<Dollars, Sats, Dollars>,
}

const VERSION: Version = Version::ONE; // Bumped for lazy height dollars

impl ValueFromHeightLast {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedFromHeightLast::forced_import(db, name, v, indexes)?;

        let btc = LazyFromHeightLast::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.read_only_boxed_clone(),
            &sats,
        );

        let usd = LazyBinaryComputedFromHeightLast::forced_import::<SatsTimesPrice>(
            &format!("{name}_usd"),
            v,
            sats.height.read_only_boxed_clone(),
            prices.usd.price.read_only_boxed_clone(),
            indexes,
        );

        Ok(Self {
            sats,
            btc,
            usd,
        })
    }
}
