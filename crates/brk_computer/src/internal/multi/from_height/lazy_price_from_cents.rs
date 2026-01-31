//! Lazy price wrapper for height-based metrics with both USD and sats representations.
//! Derives both from a cents base metric.

use brk_traversable::Traversable;
use brk_types::{CentsUnsigned, Dollars, SatsFract, Version};
use derive_more::{Deref, DerefMut};
use vecdb::IterableCloneableVec;

use super::{ComputedFromHeightLast, LazyFromHeightLast};
use crate::internal::{CentsUnsignedToDollars, CentsUnsignedToSatsFract};

/// Lazy price metric (height-based) with both USD and sats representations.
/// Both are lazily derived from a cents base metric.
///
/// Derefs to the dollars metric, so existing code works unchanged.
/// Access `.sats` for the sats exchange rate version.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyPriceFromCents {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dollars: LazyFromHeightLast<Dollars, CentsUnsigned>,
    pub sats: LazyFromHeightLast<SatsFract, CentsUnsigned>,
}

impl LazyPriceFromCents {
    pub fn from_computed(
        name: &str,
        version: Version,
        cents: &ComputedFromHeightLast<CentsUnsigned>,
    ) -> Self {
        let dollars = LazyFromHeightLast::from_computed::<CentsUnsignedToDollars>(
            name,
            version,
            cents.height.boxed_clone(),
            cents,
        );
        let sats = LazyFromHeightLast::from_computed::<CentsUnsignedToSatsFract>(
            &format!("{name}_sats"),
            version,
            cents.height.boxed_clone(),
            cents,
        );
        Self { dollars, sats }
    }
}
