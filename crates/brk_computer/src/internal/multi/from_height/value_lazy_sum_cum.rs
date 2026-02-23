//! Value type with lazy binary height + stored derived SumCum.
//!
//! Use this when the height-level sats is a lazy binary transform (e.g., mask × source).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use schemars::JsonSchema;
use vecdb::{
    BinaryTransform, Database, Exit, ReadableBoxedVec, ReadableCloneableVec, LazyVecFrom2, Rw,
    StorageMode,
};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        ComputedVecValue, LazyComputedFromHeightSumCum, LazyFromHeightSumCum, PriceTimesSats,
        SatsToBitcoin,
    },
    prices,
};

/// Value wrapper with lazy binary height + stored derived SumCum.
///
/// Sats height is a lazy binary transform (e.g., mask × source).
/// Dollars height is also lazy (price × sats).
/// Cumulative and day1 are stored.
#[derive(Traversable)]
pub struct LazyValueFromHeightSumCum<S1T, S2T, M: StorageMode = Rw>
where
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub sats: LazyComputedFromHeightSumCum<Sats, S1T, S2T, M>,
    pub btc: LazyFromHeightSumCum<Bitcoin, Sats>,
    pub usd: LazyComputedFromHeightSumCum<Dollars, Dollars, Sats, M>,
}

const VERSION: Version = Version::ZERO;

impl<S1T, S2T> LazyValueFromHeightSumCum<S1T, S2T>
where
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn forced_import<F>(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        source1: ReadableBoxedVec<Height, S1T>,
        source2: ReadableBoxedVec<Height, S2T>,
        prices: &prices::Vecs,
    ) -> Result<Self>
    where
        F: BinaryTransform<S1T, S2T, Sats>,
    {
        let v = version + VERSION;

        let sats_height = LazyVecFrom2::transformed::<F>(name, v, source1, source2);
        let sats = LazyComputedFromHeightSumCum::forced_import(db, name, v, indexes, sats_height)?;

        let btc = LazyFromHeightSumCum::from_derived::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.read_only_boxed_clone(),
            &sats.rest,
        );

        let usd_height = LazyVecFrom2::transformed::<PriceTimesSats>(
            &format!("{name}_usd"),
            v,
            prices.usd.price.read_only_boxed_clone(),
            sats.height.read_only_boxed_clone(),
        );

        let usd = LazyComputedFromHeightSumCum::forced_import(
            db,
            &format!("{name}_usd"),
            v,
            indexes,
            usd_height,
        )?;

        Ok(Self {
            sats,
            btc,
            usd,
        })
    }

    pub(crate) fn compute_cumulative(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.sats.compute_cumulative(starting_indexes, exit)?;
        self.usd.compute_cumulative(starting_indexes, exit)?;
        Ok(())
    }
}
