use std::ops::Mul;

use brk_exit::Exit;

use super::super::EagerVec;
use crate::{CheckedSub, ReadableVec, Result, StoredVec, VecValue};

impl<V> EagerVec<V>
where
    V: StoredVec,
{
    pub fn compute_subtract(
        &mut self,
        max_from: V::I,
        subtracted: &impl ReadableVec<V::I, V::T>,
        subtracter: &impl ReadableVec<V::I, V::T>,
        exit: &Exit,
    ) -> Result<()>
    where
        V::T: CheckedSub,
    {
        self.compute_transform2(
            max_from,
            subtracted,
            subtracter,
            |(i, v1, v2, ..)| {
                (
                    i,
                    v1.checked_sub(v2)
                        .expect("subtraction underflow in compute_subtract"),
                )
            },
            exit,
        )
    }

    pub fn compute_multiply<A, B>(
        &mut self,
        max_from: V::I,
        multiplied: &impl ReadableVec<V::I, A>,
        multiplier: &impl ReadableVec<V::I, B>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecValue,
        B: VecValue,
        V::T: From<A> + Mul<B, Output = V::T>,
    {
        self.compute_transform2(
            max_from,
            multiplied,
            multiplier,
            |(i, v1, v2, ..)| (i, V::T::from(v1) * v2),
            exit,
        )
    }
}
