use bitview_collections::{PerResolution, with_resolution_fields};
use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{LazyAggVec, ReadOnlyClone, ReadableCloneableVec, ReadableVec, VecValue};

use crate::{CoarserIndex, IndexSources};

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
            /// Build uncached views sharing the reader supplied by the source owner.
            pub fn from_source(
                name: &str,
                height_source: &(impl ReadableCloneableVec<Height, T> + ?Sized),
                version: Version,
                indexes: &IndexSources,
            ) -> Self {
                let height_source = height_source.read_only_boxed_clone();

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
                    $($field: res!(indexes.first_height.$field),)*
                    $($epoch: res!(indexes.first_height.$epoch),)*
                })
            }
        }
    };
}

with_resolution_fields!(define_resolutions);

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
    /// Reuse the same height source (and cache) as every resolution.
    pub fn height_source(&self) -> &(impl ReadableVec<Height, T> + Clone + 'static + use<T>) {
        self.day1.source()
    }
}
