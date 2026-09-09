use std::result::Result;

use super::{super::CompressionStrategy, ReadWriteCompressedVec};
use crate::{AnyStoredVec, CompressedIoSource, ReadableVec, VecIndex, VecValue};

impl<I, T, S> ReadableVec<I, T> for ReadWriteCompressedVec<I, T, S>
where
    I: VecIndex,
    T: VecValue,
    S: CompressionStrategy<T>,
{
    #[inline(always)]
    fn cursor_chunk_size(&self) -> usize {
        Self::PER_PAGE
    }

    #[inline(always)]
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<T>) {
        let len = self.base.len();
        let from = from.min(len);
        let to = to.min(len);
        if from >= to {
            return;
        }

        buf.reserve(to - from);
        let stored_len = self.stored_len();

        if from < stored_len {
            let stored_to = to.min(stored_len);
            if Self::prefers_mmap(self.region(), &self.pages, from, stored_to) {
                let reader = self.create_reader();
                let pages = self.pages.read();
                Self::read_stored_pages_into(&reader, &pages, from, stored_to, buf);
            } else {
                CompressedIoSource::new(self, from, stored_to).read_into(buf);
            }
        }

        if to > stored_len {
            let push_from = from.max(stored_len);
            let pushed = self.base.pushed();
            let start = push_from - stored_len;
            let end = (to - stored_len).min(pushed.len());
            buf.extend_from_slice(&pushed[start..end]);
        }
    }

    fn read_sorted_into_at(&self, indices: &[usize], out: &mut Vec<T>) {
        let len = self.base.len();
        let indices = &indices[..indices.partition_point(|&i| i < len)];
        let stored_len = self.stored_len();
        let split = indices.partition_point(|&i| i < stored_len);
        out.reserve(indices.len());
        Self::read_sorted_stored_into(
            self.region(),
            &self.pages,
            stored_len,
            &indices[..split],
            out,
        );
        if split < indices.len() {
            let pushed = self.base.pushed();
            out.extend(
                indices[split..]
                    .iter()
                    .map(|&i| pushed[i - stored_len].clone()),
            );
        }
    }

    #[inline]
    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(T)) {
        self.fold_range_at(from, to, (), |(), v| f(v));
    }

    #[inline]
    fn fold_range_at<B, F: FnMut(B, T) -> B>(&self, from: usize, to: usize, init: B, mut f: F) -> B
    where
        Self: Sized,
    {
        let len = self.base.len();
        let from = from.min(len);
        let to = to.min(len);
        if from >= to {
            return init;
        }

        let stored_len = self.stored_len();

        if to <= stored_len {
            return self.fold_source(from, to, init, f);
        }

        let mut acc = init;
        if from < stored_len {
            acc = self.fold_source(from, stored_len, acc, &mut f);
        }
        self.base.fold_pushed(from, to, acc, f)
    }

    #[inline]
    fn try_fold_range_at<B, E, F: FnMut(B, T) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> Result<B, E>
    where
        Self: Sized,
    {
        let len = self.base.len();
        let from = from.min(len);
        let to = to.min(len);
        if from >= to {
            return Ok(init);
        }

        let stored_len = self.stored_len();

        if to <= stored_len {
            return self.try_fold_source(from, to, init, f);
        }

        let mut acc = init;
        if from < stored_len {
            acc = self.try_fold_source(from, stored_len, acc, &mut f)?;
        }
        self.base.try_fold_pushed(from, to, acc, f)
    }
}
