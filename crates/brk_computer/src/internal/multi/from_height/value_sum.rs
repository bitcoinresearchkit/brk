//! Value type for Sum pattern from Height.
//!
//! Height-level USD value is lazy: `sats * price`.
//! Day1 sum is stored since it requires aggregation across heights with varying prices.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Sats, Version};
use vecdb::{Database, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        ComputedFromHeightSum, LazyBinaryComputedFromHeightSum, LazyFromHeightSum, SatsTimesPrice,
        SatsToBitcoin,
    },
    prices,
};

#[derive(Traversable)]
pub struct ValueFromHeightSum<M: StorageMode = Rw> {
    pub sats: ComputedFromHeightSum<Sats, M>,
    pub btc: LazyFromHeightSum<Bitcoin, Sats>,
    pub usd: LazyBinaryComputedFromHeightSum<Dollars, Sats, Dollars>,
}

const VERSION: Version = Version::ONE; // Bumped for lazy height dollars

impl ValueFromHeightSum {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedFromHeightSum::forced_import(db, name, v, indexes)?;

        let btc = LazyFromHeightSum::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.read_only_boxed_clone(),
            &sats,
        );

        let usd = LazyBinaryComputedFromHeightSum::forced_import::<SatsTimesPrice>(
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
