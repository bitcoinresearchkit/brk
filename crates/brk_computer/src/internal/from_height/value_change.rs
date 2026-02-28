//! Change values from Height - stores signed sats and dollars (changes can be negative).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, SatsSigned, Version};
use vecdb::{Database, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedFromHeightLast, LazyFromHeightLast, SatsSignedToBitcoin},
};

const VERSION: Version = Version::ZERO;

/// Change values indexed by height - sats (stored), btc (lazy), usd (stored).
#[derive(Traversable)]
pub struct ValueChangeFromHeight<M: StorageMode = Rw> {
    pub sats: ComputedFromHeightLast<SatsSigned, M>,
    pub btc: LazyFromHeightLast<Bitcoin, SatsSigned>,
    pub usd: ComputedFromHeightLast<Dollars, M>,
}

impl ValueChangeFromHeight {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedFromHeightLast::forced_import(db, name, v, indexes)?;

        let btc = LazyFromHeightLast::from_computed::<SatsSignedToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.read_only_boxed_clone(),
            &sats,
        );

        let usd = ComputedFromHeightLast::forced_import(
            db,
            &format!("{name}_usd"),
            v,
            indexes,
        )?;

        Ok(Self { sats, btc, usd })
    }

    /// Compute rolling change for both sats and dollars in one call.
    pub(crate) fn compute_rolling(
        &mut self,
        starting_height: Height,
        window_starts: &impl ReadableVec<Height, Height>,
        sats_source: &impl ReadableVec<Height, Sats>,
        dollars_source: &(impl ReadableVec<Height, Dollars> + Sync),
        exit: &Exit,
    ) -> Result<()> {
        self.sats
            .height
            .compute_rolling_change(starting_height, window_starts, sats_source, exit)?;
        self.usd
            .height
            .compute_rolling_change(starting_height, window_starts, dollars_source, exit)?;
        Ok(())
    }
}
