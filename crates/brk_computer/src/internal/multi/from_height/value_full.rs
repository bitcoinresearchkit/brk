//! Value type for Full pattern from Height.
//!
//! Height-level USD stats are stored (eagerly computed from sats × price).
//! Uses CumFull: stored base + cumulative + rolling windows.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, PcoVec, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        ComputedFromHeightCumulativeFull, LazyFromHeightLast, SatsToBitcoin, SatsToDollars,
        WindowStarts,
    },
    prices,
};

#[derive(Traversable)]
pub struct ValueFromHeightFull<M: StorageMode = Rw> {
    pub sats: ComputedFromHeightCumulativeFull<Sats, M>,
    pub btc: LazyFromHeightLast<Bitcoin, Sats>,
    pub usd: ComputedFromHeightCumulativeFull<Dollars, M>,
}

const VERSION: Version = Version::TWO; // Bumped for stored height dollars

impl ValueFromHeightFull {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedFromHeightCumulativeFull::forced_import(db, name, v, indexes)?;

        let btc = LazyFromHeightLast::from_height_source::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.read_only_boxed_clone(),
            indexes,
        );

        let usd =
            ComputedFromHeightCumulativeFull::forced_import(db, &format!("{name}_usd"), v, indexes)?;

        Ok(Self { sats, btc, usd })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        prices: &prices::Vecs,
        exit: &Exit,
        compute_sats: impl FnOnce(&mut EagerVec<PcoVec<Height, Sats>>) -> Result<()>,
    ) -> Result<()> {
        self.sats.compute(max_from, windows, exit, compute_sats)?;

        self.usd.compute(max_from, windows, exit, |vec| {
            Ok(vec.compute_binary::<Sats, Dollars, SatsToDollars>(
                max_from,
                &self.sats.height,
                &prices.usd.price,
                exit,
            )?)
        })
    }
}
