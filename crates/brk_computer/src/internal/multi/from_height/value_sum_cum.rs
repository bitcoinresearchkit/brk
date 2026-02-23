//! Value type for SumCum pattern from Height.
//!
//! Height-level USD sum is lazy: `sats * price`.
//! Cumulative and day1 stats are stored since they require aggregation
//! across heights with varying prices.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, ReadableCloneableVec, PcoVec, Rw, StorageMode};

use crate::{
    ComputeIndexes,
    indexes,
    internal::{
        ComputedFromHeightSumCum, LazyBinaryComputedFromHeightSumCum, LazyFromHeightSumCum,
        SatsTimesPrice, SatsToBitcoin,
    },
    prices,
};

#[derive(Traversable)]
pub struct ValueFromHeightSumCum<M: StorageMode = Rw> {
    pub sats: ComputedFromHeightSumCum<Sats, M>,
    pub btc: LazyFromHeightSumCum<Bitcoin, Sats>,
    pub usd: LazyBinaryComputedFromHeightSumCum<Dollars, Sats, Dollars, M>,
}

const VERSION: Version = Version::ONE; // Bumped for lazy height dollars

impl ValueFromHeightSumCum {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedFromHeightSumCum::forced_import(db, name, v, indexes)?;

        let btc = LazyFromHeightSumCum::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.read_only_boxed_clone(),
            &sats,
        );

        let usd = LazyBinaryComputedFromHeightSumCum::forced_import::<SatsTimesPrice>(
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
        self.sats.compute_cumulative(starting_indexes, exit)?;
        self.usd.compute_cumulative(starting_indexes, exit)?;
        Ok(())
    }
}
