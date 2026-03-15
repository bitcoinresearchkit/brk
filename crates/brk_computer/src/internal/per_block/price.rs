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

use super::{LazyPerBlock, PerBlock};
use crate::{
    indexes,
    internal::{CentsUnsignedToDollars, ComputedVecValue, DollarsToSatsFract, NumericValue},
};

/// Generic price metric with cents, USD, and sats representations.
#[derive(Clone, Traversable)]
pub struct Price<C> {
    pub usd: LazyPerBlock<Dollars, Cents>,
    pub cents: C,
    pub sats: LazyPerBlock<SatsFract, Dollars>,
}

impl Price<PerBlock<Cents>> {
    /// Import from database: stored cents, lazy USD + sats.
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let cents = PerBlock::forced_import(db, &format!("{name}_cents"), version, indexes)?;
        let usd = LazyPerBlock::from_computed::<CentsUnsignedToDollars>(
            name,
            version,
            cents.height.read_only_boxed_clone(),
            &cents,
        );
        let sats = LazyPerBlock::from_lazy::<DollarsToSatsFract, Cents>(
            &format!("{name}_sats"),
            version,
            &usd,
        );
        Ok(Self { usd, cents, sats })
    }
}

impl<ST> Price<LazyPerBlock<Cents, ST>>
where
    ST: ComputedVecValue + NumericValue + JsonSchema + 'static,
{
    /// Create from a computed source, applying a transform to produce Cents.
    pub(crate) fn from_cents_source<F: UnaryTransform<ST, Cents>>(
        name: &str,
        version: Version,
        source: &PerBlock<ST>,
    ) -> Self {
        let cents = LazyPerBlock::from_computed::<F>(
            &format!("{name}_cents"),
            version,
            source.height.read_only_boxed_clone(),
            source,
        );
        let usd = LazyPerBlock::from_lazy::<CentsUnsignedToDollars, ST>(name, version, &cents);
        let sats = LazyPerBlock::from_lazy::<DollarsToSatsFract, Cents>(
            &format!("{name}_sats"),
            version,
            &usd,
        );
        Self { usd, cents, sats }
    }
}
