//! Price wrapper for height-based metrics with both USD and sats representations.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, SatsFract, Version};
use derive_more::{Deref, DerefMut};
use vecdb::Database;

use super::{ComputedFromHeightLast, LazyUnaryFromHeightLast};
use crate::{indexes, internal::DollarsToSatsFract};

/// Price metric (height-based) with both USD and sats representations.
///
/// Derefs to the dollars metric, so existing code works unchanged.
/// Access `.sats` for the sats exchange rate version.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct PriceFromHeight {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dollars: ComputedFromHeightLast<Dollars>,
    pub sats: LazyUnaryFromHeightLast<SatsFract, Dollars>,
}

impl PriceFromHeight {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let dollars = ComputedFromHeightLast::forced_import(db, name, version, indexes)?;
        Ok(Self::from_computed(name, version, dollars))
    }

    pub fn from_computed(
        name: &str,
        version: Version,
        dollars: ComputedFromHeightLast<Dollars>,
    ) -> Self {
        let sats = LazyUnaryFromHeightLast::from_computed_last::<DollarsToSatsFract>(
            &format!("{name}_sats"),
            version,
            &dollars,
        );
        Self { dollars, sats }
    }
}
