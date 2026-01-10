//! Value type for SumCum pattern from Height.
//!
//! Height-level USD sum is lazy: `sats * price`.
//! Cumulative and dateindex stats are stored since they require aggregation
//! across heights with varying prices.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Close, Dollars, Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, IterableCloneableVec, IterableVec, PcoVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        ComputedFromHeightSumCum, LazyBinaryComputedFromHeightSumCum, LazyFromHeightSumCum,
        SatsTimesClosePrice, SatsToBitcoin,
    },
    price,
};

/// Lazy dollars type: `sats[h] * price[h]` at height level, stored derived.
pub type LazyDollarsFromHeightSumCum =
    LazyBinaryComputedFromHeightSumCum<Dollars, Sats, Close<Dollars>>;

#[derive(Clone, Traversable)]
pub struct ValueFromHeightSumCum {
    pub sats: ComputedFromHeightSumCum<Sats>,
    pub bitcoin: LazyFromHeightSumCum<Bitcoin, Sats>,
    pub dollars: Option<LazyDollarsFromHeightSumCum>,
}

const VERSION: Version = Version::ONE; // Bumped for lazy height dollars

impl ValueFromHeightSumCum {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedFromHeightSumCum::forced_import(db, name, v, indexes)?;

        let bitcoin = LazyFromHeightSumCum::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.boxed_clone(),
            &sats,
        );

        let dollars = price
            .map(|price| {
                LazyBinaryComputedFromHeightSumCum::forced_import::<SatsTimesClosePrice>(
                    db,
                    &format!("{name}_usd"),
                    v,
                    sats.height.boxed_clone(),
                    price.usd.split.close.height.boxed_clone(),
                    indexes,
                )
            })
            .transpose()?;

        Ok(Self {
            sats,
            bitcoin,
            dollars,
        })
    }

    pub fn compute_all<F>(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<Height, Sats>>) -> Result<()>,
    {
        // Compute sats (closure receives &mut height vec)
        self.sats
            .compute_all(indexes, starting_indexes, exit, |v| compute(v))?;

        // Derive dollars (height is lazy, just compute cumulative and dateindex)
        if let Some(dollars) = self.dollars.as_mut() {
            dollars.derive_from(indexes, starting_indexes, exit)?;
        }

        Ok(())
    }

    /// Derive from an external height source (e.g., a LazyVec).
    pub fn derive_from(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        source: &impl IterableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        // Derive sats from source
        self.sats
            .derive_from(indexes, starting_indexes, source, exit)?;

        // Derive dollars (height is lazy, just compute cumulative and dateindex)
        if let Some(dollars) = self.dollars.as_mut() {
            dollars.derive_from(indexes, starting_indexes, exit)?;
        }

        Ok(())
    }

    /// Compute rest (derived indexes) from already-computed height.
    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.sats.compute_rest(indexes, starting_indexes, exit)?;

        // Derive dollars (height is lazy, just compute cumulative and dateindex)
        if let Some(dollars) = self.dollars.as_mut() {
            dollars.derive_from(indexes, starting_indexes, exit)?;
        }

        Ok(())
    }
}
