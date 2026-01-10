//! Value type for Last pattern from Height.
//!
//! Height-level USD value is lazy: `sats * price`.
//! DateIndex last is stored since it requires finding the last value within each date.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Close, Dollars, Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, IterableCloneableVec, PcoVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        ComputedFromHeightLast, LazyBinaryComputedFromHeightLast, LazyFromHeightLast,
        SatsTimesClosePrice, SatsToBitcoin,
    },
    price,
};

/// Lazy dollars type: `sats[h] * price[h]` at height level, stored derived.
pub type LazyDollarsFromHeightLast =
    LazyBinaryComputedFromHeightLast<Dollars, Sats, Close<Dollars>>;

#[derive(Clone, Traversable)]
pub struct ValueFromHeightLast {
    pub sats: ComputedFromHeightLast<Sats>,
    pub bitcoin: LazyFromHeightLast<Bitcoin, Sats>,
    pub dollars: Option<LazyDollarsFromHeightLast>,
}

const VERSION: Version = Version::ONE; // Bumped for lazy height dollars

impl ValueFromHeightLast {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedFromHeightLast::forced_import(db, name, v, indexes)?;

        let bitcoin = LazyFromHeightLast::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.boxed_clone(),
            &sats,
        );

        let dollars = price
            .map(|price| {
                LazyBinaryComputedFromHeightLast::forced_import::<SatsTimesClosePrice>(
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

        // Derive dollars (height is lazy, just compute dateindex last)
        if let Some(dollars) = self.dollars.as_mut() {
            dollars.derive_from(indexes, starting_indexes, exit)?;
        }

        Ok(())
    }

    /// Compute derived vecs from existing height data.
    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.sats.compute_rest(indexes, starting_indexes, exit)?;

        // Derive dollars (height is lazy, just compute dateindex last)
        if let Some(dollars) = self.dollars.as_mut() {
            dollars.derive_from(indexes, starting_indexes, exit)?;
        }

        Ok(())
    }
}
