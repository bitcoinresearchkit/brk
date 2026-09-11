use std::convert::Infallible;

use crate::{AnyVec, READ_CHUNK_SIZE, ReadableVec, SparseRead, VecIndex, VecValue};

use super::{DeltaOp, LazyDeltaVec};

impl<I, S, T, Op> ReadableVec<I, T> for LazyDeltaVec<I, S, T, Op>
where
    I: VecIndex,
    S: VecValue,
    T: VecValue,
    Op: DeltaOp<S, T>,
{
    fn cursor_chunk_size(&self) -> usize {
        self.source.cursor_chunk_size()
    }

    #[inline]
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<T>) {
        let to = to.min(self.len());
        let starts = self.window_starts.collect_range_dyn(from, to);
        if from >= to {
            return;
        }
        buf.reserve(to - from);
        self.with_source_ranges(from, to, &starts, |current, previous_from, previous| {
            buf.extend(Self::transformed_values(
                from,
                current,
                &starts,
                previous_from,
                previous,
            ));
        });
    }

    fn for_each_chunk_at(&self, from: usize, to: usize, f: &mut dyn FnMut(usize, &[T])) {
        let to = to.min(self.len());
        let starts = self.window_starts.collect_range_dyn(from, to);
        if from >= to {
            return;
        }
        let chunk_size = self.cursor_chunk_size().clamp(1, READ_CHUNK_SIZE);
        let mut output = Vec::new();
        self.with_source_ranges(from, to, &starts, |current, previous_from, previous| {
            for (chunk, current) in current.chunks(chunk_size).enumerate() {
                let at = from + chunk * chunk_size;
                output.clear();
                output.extend(Self::transformed_values(
                    at,
                    current,
                    &starts[at - from..at - from + current.len()],
                    previous_from,
                    previous,
                ));
                f(at, &output);
            }
        });
    }

    #[inline]
    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(T)) {
        let to = to.min(self.len());
        let starts = self.window_starts.collect_range_dyn(from, to);
        if from >= to {
            return;
        }
        self.bulk_for_each(from, to, &starts, f);
    }

    #[inline]
    fn fold_range_at<B, F: FnMut(B, T) -> B>(&self, from: usize, to: usize, init: B, mut f: F) -> B
    where
        Self: Sized,
    {
        let to = to.min(self.len());
        let starts = self.window_starts.collect_range_dyn(from, to);
        if from >= to {
            return init;
        }
        self.bulk_try_fold(from, to, &starts, init, |acc, v| {
            Ok::<_, Infallible>(f(acc, v))
        })
        .unwrap_or_else(|e: Infallible| match e {})
    }

    #[inline]
    fn try_fold_range_at<B, E, F: FnMut(B, T) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        f: F,
    ) -> Result<B, E>
    where
        Self: Sized,
    {
        let to = to.min(self.len());
        let starts = self.window_starts.collect_range_dyn(from, to);
        if from >= to {
            return Ok(init);
        }
        self.bulk_try_fold(from, to, &starts, init, f)
    }

    #[inline]
    fn collect_one_at(&self, index: usize) -> Option<T> {
        if index >= self.len() {
            return None;
        }
        let start = self.window_starts.collect_one_at(index)?.to_usize();
        let current = self.source.collect_one_at(index)?;
        let ago = match Op::ago_index(start) {
            Some(idx) => self.source.collect_one_at(idx)?,
            None => Op::ago_default(),
        };
        Some(Op::combine(current, ago, Op::count(index, start)))
    }

    fn read_sorted_into_at(&self, indices: &[usize], out: &mut Vec<T>) {
        if indices.is_empty() {
            return;
        }
        let len = self.len();
        let indices = &indices[..indices.partition_point(|&index| index < len)];
        let starts = self.window_starts.read_sorted_at(indices);
        let mut starts_iter = starts.iter();
        let values = SparseRead::new(&*self.source, indices, |_| {
            Op::ago_index(starts_iter.next().unwrap().to_usize())
        });
        out.reserve(indices.len());
        for (slot, &index) in indices.iter().enumerate() {
            let start = starts[slot].to_usize();
            out.push(Op::combine(
                values.current(slot),
                values.previous(slot).unwrap_or_else(Op::ago_default),
                Op::count(index, start),
            ));
        }
    }
}
