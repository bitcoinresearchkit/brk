//! Generic price wrapper with cents, USD, and sats representations.
//!
//! All prices use this single struct with different cents types.
//! USD is always lazily derived from cents via CentsUnsignedToDollars.
//! Sats is always lazily derived from USD via DollarsToSatsFract.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, SatsFract, Version};
use schemars::JsonSchema;
use vecdb::{Database, ReadableCloneableVec, UnaryTransform};

use super::{ComputedFromHeight, LazyFromHeight};
use crate::{
    indexes,
    internal::{CentsUnsignedToDollars, ComputedVecValue, DollarsToSatsFract, NumericValue},
};

/// Generic price metric with cents, USD, and sats representations.
#[derive(Clone, Traversable)]
pub struct Price<C> {
    pub cents: C,
    pub usd: LazyFromHeight<Dollars, Cents>,
    pub sats: LazyFromHeight<SatsFract, Dollars>,
}

impl Price<ComputedFromHeight<Cents>> {
    /// Import from database: stored cents, lazy USD + sats.
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let cents =
            ComputedFromHeight::forced_import(db, &format!("{name}_cents"), version, indexes)?;
        let usd = LazyFromHeight::from_computed::<CentsUnsignedToDollars>(
            &format!("{name}_usd"),
            version,
            cents.height.read_only_boxed_clone(),
            &cents,
        );
        let sats = LazyFromHeight::from_lazy::<DollarsToSatsFract, Cents>(
            &format!("{name}_sats"),
            version,
            &usd,
        );
        Ok(Self { cents, usd, sats })
    }
}

impl<ST> Price<LazyFromHeight<Cents, ST>>
where
    ST: ComputedVecValue + NumericValue + JsonSchema + 'static,
{
    /// Create from a computed source, applying a transform to produce Cents.
    pub(crate) fn from_cents_source<F: UnaryTransform<ST, Cents>>(
        name: &str,
        version: Version,
        source: &ComputedFromHeight<ST>,
    ) -> Self {
        let cents = LazyFromHeight::from_computed::<F>(
            &format!("{name}_cents"),
            version,
            source.height.read_only_boxed_clone(),
            source,
        );
        let usd = LazyFromHeight::from_lazy::<CentsUnsignedToDollars, ST>(
            &format!("{name}_usd"),
            version,
            &cents,
        );
        let sats = LazyFromHeight::from_lazy::<DollarsToSatsFract, Cents>(
            &format!("{name}_sats"),
            version,
            &usd,
        );
        Self { cents, usd, sats }
    }
}
