mod lazy;

pub use lazy::LazyOhlcVecs;

use bitview_compute::{ComputedVecValue, LazyIndexes, PerResolution};
use bitview_traversable::Traversable;
use brk_types::{Cents, Height, OHLCCents, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{CachedBoxedVec, UnaryTransform};

use super::lazy_ohlc::LazyOhlcVec;

const COMPUTE_VERSION: Version = Version::TWO;

macro_rules! define_lazy_ohlc_cents_vecs {
    (
        periods { $($field:ident: $index:ident => $param:ident,)* }
        epochs { $($epoch:ident: $epoch_index:ident => $epoch_param:ident,)* }
    ) => {
        use brk_types::{$($index,)* $($epoch_index,)*};

        #[derive(Clone, Deref, DerefMut, Traversable)]
        #[traversable(merge)]
        pub struct LazyOhlcCentsVecs(
            pub PerResolution<
                $(LazyOhlcVec<$index>,)*
                $(LazyOhlcVec<$epoch_index>,)*
            >,
        );

        impl LazyOhlcCentsVecs {
            pub fn new(
                name: &str,
                version: Version,
                mappings: &bitview_plugin_mappings::Vecs,
                prices: CachedBoxedVec<Height, Cents>,
            ) -> Self {
                let v = version + COMPUTE_VERSION;
                Self(PerResolution {
                    $($field: LazyOhlcVec::new(
                        name, v, prices.clone(), mappings.$field.first_height.clone(),
                    ),)*
                    $($epoch: LazyOhlcVec::new(
                        name, v, prices.clone(), mappings.$epoch.first_height.clone(),
                    ),)*
                })
            }
        }
    };
}

bitview_compute::with_resolution_fields!(define_lazy_ohlc_cents_vecs);

pub trait LazyIndexesFromOhlc<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    fn from_ohlc_indexes<Transform: UnaryTransform<OHLCCents, T>>(
        name: &str,
        version: Version,
        source: &LazyOhlcCentsVecs,
    ) -> Self;
}

impl<T> LazyIndexesFromOhlc<T> for LazyIndexes<T, OHLCCents>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    fn from_ohlc_indexes<Transform: UnaryTransform<OHLCCents, T>>(
        name: &str,
        version: Version,
        source: &LazyOhlcCentsVecs,
    ) -> Self {
        Self(LazyOhlcVecs::from_ohlc_indexes::<Transform>(name, version, source).0)
    }
}
