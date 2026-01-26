//! Price wrapper for height+date-based metrics with both USD and sats representations.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, SatsFract, Version};
use derive_more::{Deref, DerefMut};
use vecdb::Database;

use super::{ComputedFromHeightAndDateLast, LazyUnaryFromHeightAndDateLast};
use crate::{indexes, internal::DollarsToSatsFract};

/// Price metric (height+date-based) with both USD and sats representations.
///
/// Derefs to the dollars metric, so existing code works unchanged.
/// Access `.sats` for the sats exchange rate version.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct PriceFromHeightAndDate {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dollars: ComputedFromHeightAndDateLast<Dollars>,
    pub sats: LazyUnaryFromHeightAndDateLast<SatsFract, Dollars>,
}

impl PriceFromHeightAndDate {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let dollars = ComputedFromHeightAndDateLast::forced_import(db, name, version, indexes)?;
        Ok(Self::from_computed(name, version, dollars))
    }

    pub fn from_computed(
        name: &str,
        version: Version,
        dollars: ComputedFromHeightAndDateLast<Dollars>,
    ) -> Self {
        let sats = LazyUnaryFromHeightAndDateLast::from_computed_last::<DollarsToSatsFract>(
            &format!("{name}_sats"),
            version,
            &dollars,
        );
        Self { dollars, sats }
    }
}
