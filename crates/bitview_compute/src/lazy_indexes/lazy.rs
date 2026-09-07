use bitview_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{LazyVec, ReadableCloneableVec, UnaryTransform, VecValue};

use crate::{ComputedVecValue, PerResolution};

macro_rules! define_lazy_indexes {
    (
        periods { $($field:ident: $index:ident => $param:ident,)* }
        epochs { $($epoch:ident: $epoch_index:ident => $epoch_param:ident,)* }
    ) => {
        use brk_types::{$($index,)* $($epoch_index,)*};

        #[derive(Clone, Deref, DerefMut, Traversable)]
        #[traversable(transparent)]
        pub struct LazyIndexes<T, S>(
            pub PerResolution<
                $(LazyVec<$index, T, $index, S>,)*
                $(LazyVec<$epoch_index, T, $epoch_index, S>,)*
            >,
        )
        where
            T: ComputedVecValue + PartialOrd + JsonSchema,
            S: VecValue;

        impl<T, S> LazyIndexes<T, S>
        where
            T: ComputedVecValue + PartialOrd + JsonSchema,
            S: VecValue,
        {
            pub fn from_lazy_indexes<Transform, U>(
                name: &str,
                version: Version,
                source: &LazyIndexes<S, U>,
            ) -> Self
            where
                Transform: UnaryTransform<S, T>,
                S: ComputedVecValue + PartialOrd + JsonSchema,
                U: VecValue,
            {
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

crate::with_resolution_fields!(define_lazy_indexes);
