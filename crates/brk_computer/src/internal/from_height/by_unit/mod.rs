mod rolling_full;
mod rolling_sum;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Sats, Version};
use vecdb::{Database, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        CentsUnsignedToDollars, ComputedFromHeight, LazyFromHeight, SatsToBitcoin, Windows,
    },
};

pub use rolling_full::*;
pub use rolling_sum::*;

#[derive(Traversable)]
pub struct ByUnit<M: StorageMode = Rw> {
    pub sats: ComputedFromHeight<Sats, M>,
    pub btc: LazyFromHeight<Bitcoin, Sats>,
    pub cents: ComputedFromHeight<Cents, M>,
    pub usd: LazyFromHeight<Dollars, Cents>,
}

impl ByUnit {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let sats = ComputedFromHeight::forced_import(db, name, version, indexes)?;

        let btc = LazyFromHeight::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            version,
            sats.height.read_only_boxed_clone(),
            &sats,
        );

        let cents = ComputedFromHeight::forced_import(
            db,
            &format!("{name}_cents"),
            version,
            indexes,
        )?;

        let usd = LazyFromHeight::from_computed::<CentsUnsignedToDollars>(
            &format!("{name}_usd"),
            version,
            cents.height.read_only_boxed_clone(),
            &cents,
        );

        Ok(Self {
            sats,
            btc,
            cents,
            usd,
        })
    }
}

impl Windows<ByUnit> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Windows::try_from_fn(|suffix| {
            ByUnit::forced_import(db, &format!("{name}_{suffix}"), version, indexes)
        })
    }
}
