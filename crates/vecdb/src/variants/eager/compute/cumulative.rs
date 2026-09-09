use std::ops::AddAssign;

use brk_exit::Exit;

use super::super::EagerVec;
use crate::{AnyVec, ReadableVec, Result, StoredVec, VecValue, WritableVec};

impl<V> EagerVec<V>
where
    V: StoredVec,
{
    /// Compute cumulative sum from a source vec.
    ///
    /// Each value in the result is the sum of all values from the source up to
    /// and including that index.
    pub fn compute_cumulative<S>(
        &mut self,
        max_from: V::I,
        source: &impl ReadableVec<V::I, S>,
        exit: &Exit,
    ) -> Result<()>
    where
        S: VecValue + Into<V::T>,
        V::T: Default + AddAssign + Copy,
    {
        self.compute_init(source.version(), max_from, exit, |this| {
            let skip = this.len();
            let end = this.batch_end(source.len());
            if skip >= end {
                return Ok(());
            }

            let mut cumulative_val = if skip > 0 {
                this.collect_one_at(skip - 1).unwrap()
            } else {
                V::T::default()
            };

            source.try_fold_range_at(skip, end, (), |(), v: S| {
                cumulative_val += v.into();
                this.push(cumulative_val);
                Ok(())
            })
        })
    }

    /// Compute cumulative sum from a custom binary transform of two source vecs.
    ///
    /// Each value in the result is the cumulative sum of `transform(source1[i], source2[i])`
    /// for all indices up to and including i.
    pub fn compute_cumulative_transformed_binary<S1, S2, F>(
        &mut self,
        max_from: V::I,
        source1: &impl ReadableVec<V::I, S1>,
        source2: &impl ReadableVec<V::I, S2>,
        mut transform: F,
        exit: &Exit,
    ) -> Result<()>
    where
        S1: VecValue,
        S2: VecValue,
        V::T: Default + AddAssign + Copy,
        F: FnMut(S1, S2) -> V::T,
    {
        let target_len = source1.len().min(source2.len());

        self.compute_init(
            source1.version() + source2.version(),
            max_from,
            exit,
            |this| {
                let skip = this.len();
                let end = this.batch_end(target_len);
                if skip >= end {
                    return Ok(());
                }

                let mut cumulative_val = if skip > 0 {
                    this.collect_one_at(skip - 1).unwrap()
                } else {
                    V::T::default()
                };

                let batch2 = source2.collect_range_at(skip, end);
                let mut iter2 = batch2.into_iter();

                source1.try_fold_range_at(skip, end, (), |(), v1: S1| {
                    let v2 = iter2.next().unwrap();
                    cumulative_val += transform(v1, v2);
                    this.push(cumulative_val);
                    Ok(())
                })
            },
        )
    }
}
