//! Change values from Height - stores signed sats and dollars (changes can be negative).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, CentsSigned, Dollars, Height, Sats, SatsSigned, Version};
use vecdb::{Database, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        CentsSignedToDollars, ComputedFromHeight, LazyFromHeight, SatsSignedToBitcoin,
    },
};

/// Change values indexed by height - sats (stored), btc (lazy), cents (stored), usd (lazy).
#[derive(Traversable)]
pub struct ValueFromHeightChange<M: StorageMode = Rw> {
    pub sats: ComputedFromHeight<SatsSigned, M>,
    pub btc: LazyFromHeight<Bitcoin, SatsSigned>,
    pub cents: ComputedFromHeight<CentsSigned, M>,
    pub usd: LazyFromHeight<Dollars, CentsSigned>,
}

impl ValueFromHeightChange {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let sats = ComputedFromHeight::forced_import(db, name, version, indexes)?;

        let btc = LazyFromHeight::from_computed::<SatsSignedToBitcoin>(
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

        let usd = LazyFromHeight::from_computed::<CentsSignedToDollars>(
            &format!("{name}_usd"),
            version,
            cents.height.read_only_boxed_clone(),
            &cents,
        );

        Ok(Self { sats, btc, cents, usd })
    }

    /// Compute rolling change for both sats and cents in one call.
    pub(crate) fn compute_rolling(
        &mut self,
        starting_height: Height,
        window_starts: &impl ReadableVec<Height, Height>,
        sats_source: &impl ReadableVec<Height, Sats>,
        cents_source: &(impl ReadableVec<Height, Cents> + Sync),
        exit: &Exit,
    ) -> Result<()> {
        self.sats
            .height
            .compute_rolling_change(starting_height, window_starts, sats_source, exit)?;
        self.cents
            .height
            .compute_rolling_change(starting_height, window_starts, cents_source, exit)?;
        Ok(())
    }
}
