//! Stored value type for Last pattern from Height.
//!
//! Both sats and USD are stored eagerly at the height level.
//! Used for rolling-window sums where USD = sum(usd_per_block),
//! NOT sats * current_price.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{Database, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedFromHeightLast, LazyFromHeightLast, SatsToBitcoin},
};

const VERSION: Version = Version::ZERO;

#[derive(Traversable)]
pub struct StoredValueFromHeightLast<M: StorageMode = Rw> {
    pub sats: ComputedFromHeightLast<Sats, M>,
    pub btc: LazyFromHeightLast<Bitcoin, Sats>,
    pub usd: ComputedFromHeightLast<Dollars, M>,
}

impl StoredValueFromHeightLast {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedFromHeightLast::forced_import(db, name, v, indexes)?;

        let btc = LazyFromHeightLast::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.read_only_boxed_clone(),
            &sats,
        );

        let usd = ComputedFromHeightLast::forced_import(db, &format!("{name}_usd"), v, indexes)?;

        Ok(Self { sats, btc, usd })
    }

    pub(crate) fn compute_rolling_sum(
        &mut self,
        max_from: Height,
        window_starts: &impl ReadableVec<Height, Height>,
        sats_source: &impl ReadableVec<Height, Sats>,
        usd_source: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.sats
            .height
            .compute_rolling_sum(max_from, window_starts, sats_source, exit)?;
        self.usd
            .height
            .compute_rolling_sum(max_from, window_starts, usd_source, exit)?;
        Ok(())
    }
}
