//! Price wrapper that provides both USD and sats representations.
//!
//! The struct derefs to dollars, making it transparent for existing code.
//! Access `.sats` for the sats representation.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, SatsFract, Version};
use derive_more::{Deref, DerefMut};
use vecdb::Database;

use super::{ComputedFromDateLast, LazyUnaryFromDateLast};
use crate::{indexes, internal::DollarsToSatsFract};

/// Price metric with both USD and sats representations.
///
/// Derefs to the dollars metric, so existing code works unchanged.
/// Access `.sats` for the sats exchange rate version.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct Price {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dollars: ComputedFromDateLast<Dollars>,
    pub sats: LazyUnaryFromDateLast<SatsFract, Dollars>,
}

impl Price {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let dollars = ComputedFromDateLast::forced_import(db, name, version, indexes)?;
        Ok(Self::from_computed(name, version, dollars))
    }

    pub fn from_computed(name: &str, version: Version, dollars: ComputedFromDateLast<Dollars>) -> Self {
        let sats = LazyUnaryFromDateLast::from_computed_last::<DollarsToSatsFract>(
            &format!("{name}_sats"),
            version,
            &dollars,
        );
        Self { dollars, sats }
    }
}
