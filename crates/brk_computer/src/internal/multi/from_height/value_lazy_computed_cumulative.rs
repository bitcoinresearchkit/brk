//! Value type with stored sats height + cumulative, stored usd, lazy btc.
//!
//! - Sats: stored height + cumulative (ComputedFromHeightCumulative)
//! - BTC: lazy transform from sats (LazyFromHeightLast)
//! - USD: stored (eagerly computed from price × sats)

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{Database, Exit, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedFromHeightCumulative, ComputedFromHeightLast, LazyFromHeightLast, SatsToBitcoin},
    prices,
};

/// Value wrapper with stored sats height + cumulative, lazy btc + stored usd.
#[derive(Traversable)]
pub struct LazyComputedValueFromHeightCumulative<M: StorageMode = Rw> {
    pub sats: ComputedFromHeightCumulative<Sats, M>,
    pub btc: LazyFromHeightLast<Bitcoin, Sats>,
    pub usd: ComputedFromHeightLast<Dollars, M>,
}

const VERSION: Version = Version::ONE; // Bumped for stored height dollars

impl LazyComputedValueFromHeightCumulative {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedFromHeightCumulative::forced_import(db, name, v, indexes)?;

        let btc = LazyFromHeightLast::from_height_source::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.read_only_boxed_clone(),
            indexes,
        );

        let usd = ComputedFromHeightLast::forced_import(db, &format!("{name}_usd"), v, indexes)?;

        Ok(Self { sats, btc, usd })
    }

    /// Compute cumulative + USD from already-filled sats height vec.
    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.sats.compute_rest(max_from, exit)?;

        self.usd.height.compute_transform2(
            max_from,
            &prices.usd.price,
            &self.sats.height,
            |(h, price, sats, ..)| {
                let btc = *sats as f64 / 100_000_000.0;
                (h, Dollars::from(*price * btc))
            },
            exit,
        )?;
        Ok(())
    }
}
