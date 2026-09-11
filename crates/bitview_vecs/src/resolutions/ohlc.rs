use bitview_collections::{PerResolution, with_resolution_fields};
use bitview_traversable::Traversable;
use brk_types::{Cents, Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::ReadableBoxedVec;

use crate::{IndexSources, LazyOhlcVec};

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
                mappings: &IndexSources,
                prices: ReadableBoxedVec<Height, Cents>,
            ) -> Self {
                let v = version + COMPUTE_VERSION;
                Self(PerResolution {
                    $($field: LazyOhlcVec::new(
                        name, v, &prices, &mappings.first_height.$field,
                    ),)*
                    $($epoch: LazyOhlcVec::new(
                        name, v, &prices, &mappings.first_height.$epoch,
                    ),)*
                })
            }
        }
    };
}

with_resolution_fields!(define_lazy_ohlc_cents_vecs);
