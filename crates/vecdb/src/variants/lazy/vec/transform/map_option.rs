use std::marker::PhantomData;

use crate::UnaryTransform;

pub struct MapOption<F>(PhantomData<F>);

impl<F, S, T> UnaryTransform<Option<S>, Option<T>> for MapOption<F>
where
    F: UnaryTransform<S, T>,
{
    #[inline(always)]
    fn apply(value: Option<S>) -> Option<T> {
        value.map(F::apply)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Negate;

    #[test]
    fn maps_present_values_and_preserves_absence() {
        assert_eq!(MapOption::<Negate>::apply(Some(4i32)), Some(-4));
        assert_eq!(MapOption::<Negate>::apply(None::<i32>), None);
    }
}
