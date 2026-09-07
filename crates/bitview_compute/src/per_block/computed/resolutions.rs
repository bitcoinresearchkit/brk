use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{LazyAggVec, ReadOnlyClone, ReadableBoxedVec, ReadableCloneableVec, VecValue};

use crate::{CACHE_BUDGET, PerResolution};

use super::CoarserIndex;

macro_rules! define_resolutions {
    (
        periods { $($field:ident: $index:ident => $param:ident,)* }
        epochs { $($epoch:ident: $epoch_index:ident => $epoch_param:ident,)* }
    ) => {
        use brk_types::{$($index,)* $($epoch_index,)*};

        #[derive(Clone, Deref, DerefMut, Traversable)]
        #[traversable(transparent)]
        pub struct Resolutions<T>(
            pub PerResolution<
                $(LazyAggVec<$index, Option<T>, Height, Height, T>,)*
                $(LazyAggVec<$epoch_index, T, Height, Height, T, CoarserIndex<$epoch_index>>,)*
            >,
        )
        where
            T: VecValue + PartialOrd + JsonSchema;

        impl<T> Resolutions<T>
        where
            T: VecValue + PartialOrd + JsonSchema + 'static,
        {
            pub fn from_boxed_height_source(
                name: &str,
                height_source: ReadableBoxedVec<Height, T>,
                version: Version,
                indexes: &crate::IndexSources,
            ) -> Self {
                let height_source = CACHE_BUDGET.wrap_boxed(height_source);

                macro_rules! res {
                    ($mapping:expr) => {{
                        let cached = $mapping.clone();
                        let mapping_version = cached.version();
                        LazyAggVec::new(
                            name,
                            version,
                            mapping_version,
                            height_source.clone(),
                            move || cached.snapshot(),
                        )
                    }};
                }

                Self(PerResolution {
                    $($field: res!(indexes.cached_first_height.$field),)*
                    $($epoch: res!(indexes.cached_first_height.$epoch),)*
                })
            }
        }
    };
}

crate::with_resolution_fields!(define_resolutions);

impl<T> ReadOnlyClone for Resolutions<T>
where
    T: VecValue + PartialOrd + JsonSchema,
{
    type ReadOnly = Self;

    fn read_only_clone(&self) -> Self {
        self.clone()
    }
}

impl<T> Resolutions<T>
where
    T: VecValue + PartialOrd + JsonSchema + 'static,
{
    pub fn from_height_source(
        name: &str,
        height_source: impl ReadableCloneableVec<Height, T> + 'static,
        version: Version,
        indexes: &crate::IndexSources,
    ) -> Self {
        Self::from_boxed_height_source(name, ReadableBoxedVec::new(height_source), version, indexes)
    }
}
