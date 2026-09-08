#[cfg(feature = "storage")]
use bitview_traversable::Traversable;

macro_rules! define_per_resolution {
    (
        periods { $($field:ident: $index:ident => $param:ident,)* }
        epochs { $($epoch:ident: $epoch_index:ident => $epoch_param:ident,)* }
    ) => {
        #[derive(Clone)]
#[cfg_attr(feature = "storage", derive(Traversable))]
        #[cfg_attr(feature = "storage", traversable(merge))]
        pub struct PerResolution<$($param,)* $($epoch_param,)*> {
            $(pub $field: $param,)*
            $(pub $epoch: $epoch_param,)*
        }
    };
}

crate::with_resolution_fields!(define_per_resolution);
