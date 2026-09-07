use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{Formattable, LazyVec, UnaryTransform, VecValue};

macro_rules! define_constant_vecs {
    (
        periods { $($field:ident: $index:ident => $param:ident,)* }
        epochs { $($epoch:ident: $epoch_index:ident => $epoch_param:ident,)* }
    ) => {
        use brk_types::{$($index,)* $($epoch_index,)*};

        #[derive(Clone, Traversable)]
        #[traversable(merge)]
        pub struct ConstantVecs<T>
        where
            T: VecValue + Formattable + Serialize + JsonSchema,
        {
            pub height: LazyVec<Height, T, Height, Minute10>,
            $(pub $field: LazyVec<$index, T, $index, Height>,)*
            $(pub $epoch: LazyVec<$epoch_index, T, $epoch_index, Height>,)*
        }

        impl<T: VecValue + Formattable + Serialize + JsonSchema> ConstantVecs<T> {
            pub fn new<F>(name: &str, version: Version, indexes: &crate::IndexSources) -> Self
            where
                F: UnaryTransform<Height, T>
                    $(+ UnaryTransform<$index, T>)*
                    $(+ UnaryTransform<$epoch_index, T>)*,
            {
                Self {
                    height: LazyVec::init(name, version, indexes.height_minute10.clone(), |idx, _| {
                        F::apply(idx)
                    }),
                    $($field: LazyVec::init(
                        name, version, indexes.first_height.$field.clone(),
                        |idx, _: Height| F::apply(idx),
                    ),)*
                    $($epoch: LazyVec::init(
                        name, version, indexes.first_height.$epoch.clone(),
                        |idx, _: Height| F::apply(idx),
                    ),)*
                }
            }
        }
    };
}

crate::with_resolution_fields!(define_constant_vecs);
