mod rolling_full;
mod rolling_sum;
mod windows;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Sats, Version};
use vecdb::{Database, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        CentsUnsignedToDollars, ComputedFromHeightLast, LazyFromHeightLast, SatsToBitcoin,
    },
};

pub use rolling_full::*;
pub use rolling_sum::*;

#[derive(Traversable)]
pub struct ByUnit<M: StorageMode = Rw> {
    pub sats: ComputedFromHeightLast<Sats, M>,
    pub btc: LazyFromHeightLast<Bitcoin, Sats>,
    pub cents: ComputedFromHeightLast<Cents, M>,
    pub usd: LazyFromHeightLast<Dollars, Cents>,
}

impl ByUnit {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let sats = ComputedFromHeightLast::forced_import(db, name, version, indexes)?;

        let btc = LazyFromHeightLast::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            version,
            sats.height.read_only_boxed_clone(),
            &sats,
        );

        let cents = ComputedFromHeightLast::forced_import(
            db,
            &format!("{name}_cents"),
            version,
            indexes,
        )?;

        let usd = LazyFromHeightLast::from_computed::<CentsUnsignedToDollars>(
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
