//! Value type for Full pattern from Height.
//!
//! Height-level USD stats are lazy: `sats * price`.
//! Cumulative and day1 stats are stored since they require aggregation
//! across heights with varying prices.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, ReadableCloneableVec, PcoVec, Rw, StorageMode};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        ComputedFromHeightFull, LazyBinaryComputedFromHeightFull, LazyFromHeightFull,
        SatsTimesPrice, SatsToBitcoin,
    },
    prices,
};

#[derive(Traversable)]
pub struct ValueFromHeightFull<M: StorageMode = Rw> {
    pub sats: ComputedFromHeightFull<Sats, M>,
    pub btc: LazyFromHeightFull<Bitcoin, Sats>,
    pub usd: LazyBinaryComputedFromHeightFull<Dollars, Sats, Dollars, M>,
}

const VERSION: Version = Version::ONE; // Bumped for lazy height dollars

impl ValueFromHeightFull {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedFromHeightFull::forced_import(db, name, v, indexes)?;

        let btc = LazyFromHeightFull::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.read_only_boxed_clone(),
            &sats,
        );

        let usd = LazyBinaryComputedFromHeightFull::forced_import::<SatsTimesPrice>(
            db,
            &format!("{name}_usd"),
            v,
            sats.height.read_only_boxed_clone(),
            prices.usd.price.read_only_boxed_clone(),
            indexes,
        )?;

        Ok(Self {
            sats,
            btc,
            usd,
        })
    }

    pub(crate) fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        mut compute: impl FnMut(&mut EagerVec<PcoVec<Height, Sats>>) -> Result<()>,
    ) -> Result<()> {
        compute(&mut self.sats.height)?;
        self.sats.rest.compute_cumulative(starting_indexes, &self.sats.height, exit)?;
        self.usd.compute_cumulative(starting_indexes, exit)?;
        Ok(())
    }
}
