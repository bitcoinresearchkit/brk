use brk_exit::Exit;

use super::super::EagerVec;
use crate::{
    AnyVec, ReadableVec, Result, SaturatingAdd, StoredVec, VecIndex, VecValue, WritableVec,
    unlikely,
};

impl<V> EagerVec<V>
where
    V: StoredVec,
{
    pub fn compute_sum_from_indexes<A, B>(
        &mut self,
        max_from: V::I,
        first_indexes: &impl ReadableVec<V::I, A>,
        indexes_count: &impl ReadableVec<V::I, B>,
        source: &(impl ReadableVec<A, V::T> + Sized),
        exit: &Exit,
    ) -> Result<()>
    where
        V::T: Default + SaturatingAdd,
        A: VecIndex + VecValue,
        B: VecValue,
        usize: From<B>,
    {
        self.compute_init(
            first_indexes.version() + indexes_count.version() + source.version(),
            max_from,
            exit,
            |this| {
                let skip = this.len();
                let source_end = indexes_count.len();
                let end = this.batch_end(source_end);
                if skip >= end {
                    return Ok(());
                }

                let pos = if skip < first_indexes.len() {
                    first_indexes.collect_one_at(skip).unwrap().to_usize()
                } else {
                    return Ok(());
                };

                let counts_batch: Vec<usize> = indexes_count
                    .collect_range_at(skip, end)
                    .into_iter()
                    .map(usize::from)
                    .collect();
                let total_count: usize = counts_batch.iter().sum();

                let mut group_idx = 0usize;

                // Skip leading zero-count groups
                while group_idx < counts_batch.len() && counts_batch[group_idx] == 0 {
                    this.push(V::T::default());
                    group_idx += 1;
                }

                if group_idx < counts_batch.len() {
                    let mut remaining = counts_batch[group_idx];

                    source.fold_range_at(
                        pos,
                        pos + total_count,
                        V::T::default(),
                        |sum, val: V::T| {
                            let sum = sum.saturating_add(val);
                            remaining -= 1;
                            if unlikely(remaining == 0) {
                                this.push(sum);
                                group_idx += 1;
                                while group_idx < counts_batch.len() && counts_batch[group_idx] == 0
                                {
                                    this.push(V::T::default());
                                    group_idx += 1;
                                }
                                if group_idx < counts_batch.len() {
                                    remaining = counts_batch[group_idx];
                                }
                                V::T::default()
                            } else {
                                sum
                            }
                        },
                    );
                }

                Ok(())
            },
        )
    }
}
