use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{ReadableBoxedVec, ReadableCloneableVec, UnaryTransform, VecValue};

use crate::{ComputedVecValue, PerResolution, Resolutions};

use super::{LazyTransformLast, MapOption};

macro_rules! define_derived_resolutions {
    (
        periods { $($field:ident: $index:ident => $param:ident,)* }
        epochs { $($epoch:ident: $epoch_index:ident => $epoch_param:ident,)* }
    ) => {
        use brk_types::{$($index,)* $($epoch_index,)*};

        #[derive(Clone, Deref, DerefMut, Traversable)]
        #[traversable(transparent)]
        pub struct DerivedResolutions<T, S1T = T>(
            pub PerResolution<
                $(LazyTransformLast<$index, Option<T>, Option<S1T>>,)*
                $(LazyTransformLast<$epoch_index, T, S1T>,)*
            >,
        )
        where
            T: VecValue + PartialOrd + JsonSchema,
            S1T: VecValue;

        impl<T, S1T> DerivedResolutions<T, S1T>
        where
            T: VecValue + PartialOrd + JsonSchema + 'static,
            S1T: VecValue + PartialOrd + JsonSchema,
        {
            pub fn from_derived_computed<F: UnaryTransform<S1T, T>>(
                name: &str,
                version: Version,
                source: &Resolutions<S1T>,
            ) -> Self {
                Self::from_sources::<F>(
                    name, version,
                    PerResolution {
                        $($field: source.$field.read_only_boxed_clone(),)*
                        $($epoch: source.$epoch.read_only_boxed_clone(),)*
                    },
                )
            }

            pub fn from_lazy<F, S2T>(
                name: &str,
                version: Version,
                source: &DerivedResolutions<S1T, S2T>,
            ) -> Self
            where
                F: UnaryTransform<S1T, T>,
                S2T: ComputedVecValue + JsonSchema,
            {
                Self::from_sources::<F>(
                    name, version,
                    PerResolution {
                        $($field: source.$field.read_only_boxed_clone(),)*
                        $($epoch: source.$epoch.read_only_boxed_clone(),)*
                    },
                )
            }

            fn from_sources<F: UnaryTransform<S1T, T>>(
                name: &str,
                version: Version,
                source: PerResolution<
                    $(ReadableBoxedVec<$index, Option<S1T>>,)*
                    $(ReadableBoxedVec<$epoch_index, S1T>,)*
                >,
            ) -> Self {
                Self(PerResolution {
                    $($field: LazyTransformLast::from_boxed::<MapOption<F>>(
                        name, version, source.$field,
                    ),)*
                    $($epoch: LazyTransformLast::from_boxed::<F>(
                        name, version, source.$epoch,
                    ),)*
                })
            }
        }
    };
}

crate::with_resolution_fields!(define_derived_resolutions);

impl<T, S1T> DerivedResolutions<T, S1T>
where
    T: VecValue + PartialOrd + JsonSchema + 'static,
    S1T: VecValue + PartialOrd + JsonSchema,
{
    pub fn from_height_source<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        height_source: ReadableBoxedVec<Height, S1T>,
        indexes: &crate::IndexSources,
    ) -> Self {
        let derived = Resolutions::from_boxed_height_source(name, height_source, version, indexes);
        Self::from_derived_computed::<F>(name, version, &derived)
    }
}
