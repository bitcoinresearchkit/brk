use std::marker::PhantomData;

use crate::BinaryTransform;

/// Apply a binary transform with the input order reversed.
pub struct ReverseOperands<F>(PhantomData<F>);

impl<S, D, T, F> BinaryTransform<S, D, T> for ReverseOperands<F>
where
    F: BinaryTransform<D, S, T>,
{
    #[inline]
    fn apply(source: S, operand: D) -> T {
        F::apply(operand, source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Subtract;

    impl BinaryTransform<i32, i32> for Subtract {
        fn apply(left: i32, right: i32) -> i32 {
            left - right
        }
    }

    #[test]
    fn reverses_noncommutative_operands() {
        assert_eq!(ReverseOperands::<Subtract>::apply(2, 9), 7);
    }
}
