//! Value type for Height-only storage (no derived indexes).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom1, PcoVec};

use crate::internal::SatsToBitcoin;

const VERSION: Version = Version::ZERO;

/// Value type with only height indexing (no derived dateindex/periods).
///
/// Used for metrics that are computed per height but don't need index aggregations.
#[derive(Clone, Traversable)]
pub struct ValueBlockHeight {
    pub sats: EagerVec<PcoVec<Height, Sats>>,
    pub bitcoin: LazyVecFrom1<Height, Bitcoin, Height, Sats>,
    pub dollars: Option<EagerVec<PcoVec<Height, Dollars>>>,
}

impl ValueBlockHeight {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        compute_dollars: bool,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = EagerVec::forced_import(db, name, v)?;

        let bitcoin = LazyVecFrom1::transformed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.boxed_clone(),
        );

        let dollars = compute_dollars
            .then(|| EagerVec::forced_import(db, &format!("{name}_usd"), v))
            .transpose()?;

        Ok(Self {
            sats,
            bitcoin,
            dollars,
        })
    }
}
