//! StoredValueRollingWindows - window-first ordering.
//!
//! Access pattern: `coinbase_sum._24h.sats.height`
//! Each window (24h, 7d, 30d, 1y) contains sats (stored) + btc (lazy) + usd (stored).

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use brk_types::{Dollars, Sats};

use crate::{
    indexes,
    internal::{StoredValueFromHeightLast, WindowStarts, Windows},
};

const VERSION: Version = Version::ZERO;

/// Stored value rolling windows — window-first, currency-last.
///
/// Each window contains `StoredValueFromHeightLast` (sats + btc lazy + usd).
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct StoredValueRollingWindows<M: StorageMode = Rw>(
    pub Windows<StoredValueFromHeightLast<M>>,
);

impl StoredValueRollingWindows {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        Ok(Self(Windows {
            _24h: StoredValueFromHeightLast::forced_import(
                db,
                &format!("{name}_24h"),
                v,
                indexes,
            )?,
            _7d: StoredValueFromHeightLast::forced_import(
                db,
                &format!("{name}_7d"),
                v,
                indexes,
            )?,
            _30d: StoredValueFromHeightLast::forced_import(
                db,
                &format!("{name}_30d"),
                v,
                indexes,
            )?,
            _1y: StoredValueFromHeightLast::forced_import(
                db,
                &format!("{name}_1y"),
                v,
                indexes,
            )?,
        }))
    }

    pub(crate) fn compute_rolling_sum(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        sats_source: &impl ReadableVec<Height, Sats>,
        usd_source: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.0
            ._24h
            .compute_rolling_sum(max_from, windows._24h, sats_source, usd_source, exit)?;
        self.0
            ._7d
            .compute_rolling_sum(max_from, windows._7d, sats_source, usd_source, exit)?;
        self.0
            ._30d
            .compute_rolling_sum(max_from, windows._30d, sats_source, usd_source, exit)?;
        self.0
            ._1y
            .compute_rolling_sum(max_from, windows._1y, sats_source, usd_source, exit)?;
        Ok(())
    }
}
