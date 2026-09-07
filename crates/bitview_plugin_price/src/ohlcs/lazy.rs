use bitview_compute::PerResolution;
use bitview_traversable::Traversable;
use brk_types::{OHLCCents, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{BytesVecValue, Formattable, LazyVec, ReadableCloneableVec, UnaryTransform};

use super::LazyOhlcCentsVecs;

macro_rules! define_lazy_ohlc_vecs {
    (
        periods { $($field:ident: $index:ident => $param:ident,)* }
        epochs { $($epoch:ident: $epoch_index:ident => $epoch_param:ident,)* }
    ) => {
        use brk_types::{$($index,)* $($epoch_index,)*};

        #[derive(Clone, Deref, DerefMut, Traversable)]
        #[traversable(merge)]
        pub struct LazyOhlcVecs<T, S>(
            pub PerResolution<
                $(LazyVec<$index, T, $index, S>,)*
                $(LazyVec<$epoch_index, T, $epoch_index, S>,)*
            >,
        )
        where
            T: BytesVecValue + Formattable + Serialize + JsonSchema,
            S: BytesVecValue;

        impl<T> LazyOhlcVecs<T, OHLCCents>
        where
            T: BytesVecValue + Formattable + Serialize + JsonSchema,
        {
            pub fn from_ohlc_indexes<Transform: UnaryTransform<OHLCCents, T>>(
                name: &str,
                version: Version,
                source: &LazyOhlcCentsVecs,
            ) -> Self {
                Self(PerResolution {
                    $($field: LazyVec::transformed::<Transform>(
                        name, version, source.$field.read_only_boxed_clone(),
                    ),)*
                    $($epoch: LazyVec::transformed::<Transform>(
                        name, version, source.$epoch.read_only_boxed_clone(),
                    ),)*
                })
            }
        }
    };
}

bitview_compute::with_resolution_fields!(define_lazy_ohlc_vecs);
