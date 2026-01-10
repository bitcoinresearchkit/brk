//! Value type for Sum pattern from Height.
//!
//! Height-level USD value is lazy: `sats * price`.
//! DateIndex sum is stored since it requires aggregation across heights with varying prices.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Close, Dollars, Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, IterableCloneableVec, PcoVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        ComputedFromHeightSum, LazyBinaryComputedFromHeightSum, LazyFromHeightSum,
        SatsTimesClosePrice, SatsToBitcoin,
    },
    price,
};

/// Lazy dollars type: `sats[h] * price[h]` at height level, stored derived.
pub type LazyDollarsFromHeightSum =
    LazyBinaryComputedFromHeightSum<Dollars, Sats, Close<Dollars>>;

#[derive(Clone, Traversable)]
pub struct ValueFromHeightSum {
    pub sats: ComputedFromHeightSum<Sats>,
    pub bitcoin: LazyFromHeightSum<Bitcoin, Sats>,
    pub dollars: Option<LazyDollarsFromHeightSum>,
}

const VERSION: Version = Version::ONE; // Bumped for lazy height dollars

impl ValueFromHeightSum {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedFromHeightSum::forced_import(db, name, v, indexes)?;

        let bitcoin = LazyFromHeightSum::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.boxed_clone(),
            &sats,
        );

        let dollars = price
            .map(|price| {
                LazyBinaryComputedFromHeightSum::forced_import::<SatsTimesClosePrice>(
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

        // Derive dollars (height is lazy, just compute dateindex sum)
        if let Some(dollars) = self.dollars.as_mut() {
            dollars.derive_from(indexes, starting_indexes, exit)?;
        }

        Ok(())
    }
}
