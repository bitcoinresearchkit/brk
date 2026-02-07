//! Lazy price wrapper with both USD and sats representations.
//!
//! Both dollars and sats are computed from the same source.

use std::marker::PhantomData;

use brk_traversable::Traversable;
use brk_types::{Dollars, SatsFract, Version};
use derive_more::{Deref, DerefMut};
use vecdb::UnaryTransform;

use super::{ComputedFromDateLast, LazyFromDateLast};
use crate::internal::{ComputedVecValue, DollarsToSatsFract};

/// Lazy price with both USD and sats representations.
///
/// Both are computed from the same source via separate transforms.
/// Derefs to the dollars metric.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyPrice<ST>
where
    ST: ComputedVecValue,
{
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dollars: LazyFromDateLast<Dollars, ST>,
    pub sats: LazyFromDateLast<SatsFract, ST>,
}

/// Composed transform: ST -> Dollars -> SatsFract
pub struct ComposedDollarsToSatsFract<F>(PhantomData<F>);

impl<F, ST> UnaryTransform<ST, SatsFract> for ComposedDollarsToSatsFract<F>
where
    F: UnaryTransform<ST, Dollars>,
{
    #[inline(always)]
    fn apply(source: ST) -> SatsFract {
        DollarsToSatsFract::apply(F::apply(source))
    }
}

impl<ST> LazyPrice<ST>
where
    ST: ComputedVecValue + schemars::JsonSchema + 'static,
{
    pub fn from_source<F: UnaryTransform<ST, Dollars>>(
        name: &str,
        version: Version,
        source: &ComputedFromDateLast<ST>,
    ) -> Self {
        let dollars = LazyFromDateLast::from_source::<F>(name, version, source);
        let sats = LazyFromDateLast::from_source::<ComposedDollarsToSatsFract<F>>(
            &format!("{name}_sats"),
            version,
            source,
        );
        Self { dollars, sats }
    }
}
