//! Generic price wrapper with both USD and sats representations.
//!
//! All prices use this single struct with different USD types.
//! Sats is always lazily derived from USD via DollarsToSatsFract.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, SatsFract, Version};
use schemars::JsonSchema;
use vecdb::{Database, ReadableCloneableVec, UnaryTransform};

use super::{ComputedFromHeightLast, LazyFromHeightLast};
use crate::{
    indexes,
    internal::{ComputedVecValue, DollarsToSatsFract, NumericValue},
};

/// Generic price metric with both USD and sats representations.
#[derive(Clone, Traversable)]
pub struct Price<U> {
    pub usd: U,
    pub sats: LazyFromHeightLast<SatsFract, Dollars>,
}

impl Price<ComputedFromHeightLast<Dollars>> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let usd = ComputedFromHeightLast::forced_import(db, name, version, indexes)?;
        let sats = LazyFromHeightLast::from_computed::<DollarsToSatsFract>(
            &format!("{name}_sats"),
            version,
            usd.height.read_only_boxed_clone(),
            &usd,
        );
        Ok(Self { usd, sats })
    }
}

impl<ST> Price<LazyFromHeightLast<Dollars, ST>>
where
    ST: ComputedVecValue + NumericValue + JsonSchema + 'static,
{
    pub(crate) fn from_computed<F: UnaryTransform<ST, Dollars>>(
        name: &str,
        version: Version,
        source: &ComputedFromHeightLast<ST>,
    ) -> Self {
        let usd = LazyFromHeightLast::from_computed::<F>(
            name,
            version,
            source.height.read_only_boxed_clone(),
            source,
        );
        let sats = LazyFromHeightLast::from_lazy::<DollarsToSatsFract, ST>(
            &format!("{name}_sats"),
            version,
            &usd,
        );
        Self { usd, sats }
    }
}
