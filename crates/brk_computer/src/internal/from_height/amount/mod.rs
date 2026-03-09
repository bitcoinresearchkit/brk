mod cumulative;
mod cumulative_sum;
mod full;
mod lazy;
mod lazy_derived;
mod rolling;
mod rolling_full;
mod rolling_sum;
mod windows;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use vecdb::{AnyVec, Database, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        CentsUnsignedToDollars, ComputedFromHeight, LazyFromHeight, SatsToBitcoin, SatsToCents,
        Windows,
    },
    prices,
};

pub use cumulative::*;
pub use cumulative_sum::*;
pub use full::*;
pub use lazy::*;
pub use rolling::*;
pub use rolling_full::*;
pub use rolling_sum::*;
pub use lazy_derived::*;
pub use windows::*;

#[derive(Traversable)]
pub struct AmountFromHeight<M: StorageMode = Rw> {
    pub sats: ComputedFromHeight<Sats, M>,
    pub btc: LazyFromHeight<Bitcoin, Sats>,
    pub cents: ComputedFromHeight<Cents, M>,
    pub usd: LazyFromHeight<Dollars, Cents>,
}

impl AmountFromHeight {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let sats =
            ComputedFromHeight::forced_import(db, &format!("{name}_sats"), version, indexes)?;

        let btc = LazyFromHeight::from_computed::<SatsToBitcoin>(
            name,
            version,
            sats.height.read_only_boxed_clone(),
            &sats,
        );

        let cents =
            ComputedFromHeight::forced_import(db, &format!("{name}_cents"), version, indexes)?;

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

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.sats.height.len()
    }

    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.cents.compute_binary::<Sats, Cents, SatsToCents>(
            max_from,
            &self.sats.height,
            &prices.price.cents.height,
            exit,
        )?;
        Ok(())
    }

    pub(crate) fn compute_rolling_sum(
        &mut self,
        max_from: Height,
        window_starts: &impl ReadableVec<Height, Height>,
        sats_source: &impl ReadableVec<Height, Sats>,
        cents_source: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        self.sats
            .height
            .compute_rolling_sum(max_from, window_starts, sats_source, exit)?;
        self.cents
            .height
            .compute_rolling_sum(max_from, window_starts, cents_source, exit)?;
        Ok(())
    }
}

impl Windows<AmountFromHeight> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Windows::try_from_fn(|suffix| {
            AmountFromHeight::forced_import(db, &format!("{name}_{suffix}"), version, indexes)
        })
    }
}
