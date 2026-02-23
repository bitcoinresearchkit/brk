//! Value type with stored height + lazy dollars for SumCum pattern.
//!
//! Use this when:
//! - Sats height is stored (primary source of truth)
//! - Sats indexes are derived from height
//! - Bitcoin is lazy (transform from sats)
//! - Dollars height is lazy (price × sats), with stored indexes

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Sats, Version};
use vecdb::{Database, Exit, ReadableCloneableVec, LazyVecFrom2, Rw, StorageMode};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        ComputedFromHeightSumCum, LazyComputedFromHeightSumCum, LazyFromHeightSumCum,
        PriceTimesSats, SatsToBitcoin,
    },
    prices,
};

/// Value wrapper with stored sats height + lazy dollars.
///
/// Sats height is stored (computed directly or from stateful loop).
/// Dollars height is lazy (price × sats).
/// Cumulative and day1 aggregates are stored for both.
#[derive(Traversable)]
pub struct LazyComputedValueFromHeightSumCum<M: StorageMode = Rw> {
    pub sats: ComputedFromHeightSumCum<Sats, M>,
    pub btc: LazyFromHeightSumCum<Bitcoin, Sats>,
    pub usd: LazyComputedFromHeightSumCum<Dollars, Dollars, Sats, M>,
}

const VERSION: Version = Version::ZERO;

impl LazyComputedValueFromHeightSumCum {
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

    /// Compute cumulative from already-computed height.
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
