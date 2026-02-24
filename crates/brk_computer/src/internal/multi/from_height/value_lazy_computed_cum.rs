//! Value type with stored sats height + cumulative, lazy btc + lazy dollars.
//!
//! Like LazyComputedValueFromHeightSumCum but with Cum (no old period aggregations).
//! - Sats: stored height + cumulative (ComputedFromHeightCum)
//! - BTC: lazy transform from sats (LazyFromHeightLast)
//! - USD: lazy binary (price × sats), LazyLast per index (no stored cumulative)

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{Database, Exit, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        ComputedFromHeightCum, LazyBinaryComputedFromHeightLast, LazyFromHeightLast,
        PriceTimesSats, SatsToBitcoin,
    },
    prices,
};

/// Value wrapper with stored sats height + cumulative, lazy btc + lazy usd.
#[derive(Traversable)]
pub struct LazyComputedValueFromHeightCum<M: StorageMode = Rw> {
    pub sats: ComputedFromHeightCum<Sats, M>,
    pub btc: LazyFromHeightLast<Bitcoin, Sats>,
    pub usd: LazyBinaryComputedFromHeightLast<Dollars, Dollars, Sats>,
}

const VERSION: Version = Version::ZERO;

impl LazyComputedValueFromHeightCum {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedFromHeightCum::forced_import(db, name, v, indexes)?;

        let btc = LazyFromHeightLast::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.read_only_boxed_clone(),
            &sats,
        );

        let usd = LazyBinaryComputedFromHeightLast::forced_import::<PriceTimesSats>(
            &format!("{name}_usd"),
            v,
            prices.usd.price.read_only_boxed_clone(),
            sats.height.read_only_boxed_clone(),
            indexes,
        );

        Ok(Self { sats, btc, usd })
    }

    /// Compute cumulative from already-filled sats height vec.
    pub(crate) fn compute_cumulative(
        &mut self,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.sats.compute_cumulative(max_from, exit)?;
        Ok(())
    }
}
