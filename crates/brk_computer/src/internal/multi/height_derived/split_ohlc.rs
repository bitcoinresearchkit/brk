//! OHLC split into separate First/Last/Max/Min period groupings derived from height-level data.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::ReadableBoxedVec;

use crate::{
    indexes,
    internal::{
        ComputedHeightDerivedFirst, ComputedHeightDerivedLast, ComputedHeightDerivedMax,
        ComputedHeightDerivedMin, ComputedVecValue, NumericValue,
    },
};

/// Split OHLC vecs for all periods, derived from height data.
#[derive(Clone, Traversable)]
pub struct ComputedHeightDerivedSplitOHLC<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub open: ComputedHeightDerivedFirst<T>,
    pub high: ComputedHeightDerivedMax<T>,
    pub low: ComputedHeightDerivedMin<T>,
    pub close: ComputedHeightDerivedLast<T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedHeightDerivedSplitOHLC<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        height_source: ReadableBoxedVec<Height, T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            open: ComputedHeightDerivedFirst::forced_import(&format!("{name}_open"), height_source.clone(), v, indexes),
            high: ComputedHeightDerivedMax::forced_import(&format!("{name}_high"), height_source.clone(), v, indexes),
            low: ComputedHeightDerivedMin::forced_import(&format!("{name}_low"), height_source.clone(), v, indexes),
            close: ComputedHeightDerivedLast::forced_import(&format!("{name}_close"), height_source, v, indexes),
        }
    }
}
