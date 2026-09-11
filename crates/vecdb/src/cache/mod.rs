//! Source-owned range retention. Cache buffers never escape a read operation.

mod budget;
mod budgeted;
mod buffer;
mod charge;
mod directory;
mod none;
mod plan;
mod policy;
mod request;
mod shared;
mod table;
mod value;

pub use budget::CacheBudget;
pub use budgeted::Budgeted;
pub use none::NoCache;
pub use policy::CachePolicy;

use buffer::Buffer;
use charge::Charge;
pub(crate) use request::Request;
use shared::Shared;
use table::Table;
use value::Value;

use std::{
    ops::Range,
    result::Result as FoldResult,
    sync::{Arc, atomic::Ordering::Relaxed},
};

use crate::{READ_CHUNK_SIZE, Result, VecValue};

/// One cache shared by a source and all of its read-only views.
#[derive(Debug)]
pub struct Cache<T: VecValue> {
    shared: Arc<Shared<T>>,
}

impl<T: VecValue> Clone for Cache<T> {
    fn clone(&self) -> Self {
        Self {
            shared: self.shared.clone(),
        }
    }
}

impl<T: VecValue> Cache<T> {
    pub(crate) fn new(budget: &'static CacheBudget) -> Self {
        let shared = Arc::new(Shared::new(budget));
        let reclaim: Arc<dyn budget::Reclaim> = shared.clone();
        budget.register(Arc::downgrade(&reclaim));
        Self { shared }
    }

    pub(crate) fn revision(&self) -> u64 {
        self.shared.revision.load(Relaxed)
    }

    pub(crate) fn try_read_range(
        &self,
        from: usize,
        to: usize,
        len: impl FnOnce() -> usize,
        out: &mut Vec<T>,
    ) -> bool {
        let Some(_gate) = self.shared.gate.try_read_recursive() else {
            return false;
        };
        self.shared.healthy.load(Relaxed)
            && from <= to
            && to <= len()
            && self.shared.copy(Request::Range(from, to), out)
    }

    pub(crate) fn read_scope<R>(&self, read: impl FnOnce() -> R) -> R {
        let _gate = self.shared.gate.read_recursive();
        assert!(self.shared.healthy.load(Relaxed), "unpublished source");
        read()
    }

    /// A failed source write stays closed until a successful repair/update.
    pub(crate) fn update<R>(
        &self,
        from: usize,
        replaces: bool,
        write: impl FnOnce() -> Result<R>,
    ) -> Result<R> {
        let _gate = self.shared.gate.write();
        self.shared.healthy.store(false, Relaxed);
        if replaces {
            self.shared.revision.fetch_add(1, Relaxed);
        }
        self.shared.invalidate_from(from);
        let result = write();
        if result.is_ok() {
            self.shared.healthy.store(true, Relaxed);
        }
        result
    }

    /// The loader returns sorted, non-overlapping source-selected ranges that
    /// cover every miss. Physical page size is deliberately not a cache concern.
    pub(crate) fn read(
        &self,
        request: Request<'_>,
        out: &mut Vec<T>,
        load: impl FnMut(&[Range<usize>]) -> Vec<(usize, Vec<T>)>,
    ) {
        self.read_request(request, out, load, self.admissible(request));
    }

    pub(crate) fn get(
        &self,
        index: usize,
        load: impl FnMut(&[Range<usize>]) -> Vec<(usize, Vec<T>)>,
    ) -> T {
        {
            let table = self.shared.table.read();
            if let Some((&start, values)) = table.range(..=index).next_back()
                && let Some(value) = values.get(index - start)
            {
                return value;
            }
        }
        let mut out = Vec::with_capacity(1);
        self.read(Request::Sorted(&[index]), &mut out, load);
        out.pop()
            .expect("source must cover a valid requested index")
    }

    #[inline]
    pub(crate) fn try_fold<B, E>(
        &self,
        from: usize,
        to: usize,
        init: B,
        load: impl FnMut(&[Range<usize>]) -> Vec<(usize, Vec<T>)>,
        mut fold: impl FnMut(B, T) -> FoldResult<B, E>,
    ) -> FoldResult<B, E> {
        let mut acc = Some(init);
        self.try_for_each_chunk(from, to, load, |_, values| {
            acc = Some(
                values
                    .iter()
                    .cloned()
                    .try_fold(acc.take().unwrap(), &mut fold)?,
            );
            Ok(())
        })?;
        Ok(acc.unwrap())
    }

    fn admissible(&self, request: Request<'_>) -> bool {
        match request {
            Request::Range(from, to) => to
                .saturating_sub(from)
                .checked_mul(size_of::<T>())
                .and_then(|bytes| bytes.checked_add(Table::<T>::charge_for(1)))
                .is_some_and(|bytes| bytes <= self.shared.budget.limit()),
            _ => true,
        }
    }

    fn read_request(
        &self,
        request: Request<'_>,
        out: &mut Vec<T>,
        mut load: impl FnMut(&[Range<usize>]) -> Vec<(usize, Vec<T>)>,
        retain: bool,
    ) {
        if self.shared.copy(request, out) {
            return;
        }
        let _fill = self.shared.fill.lock();
        if self.shared.copy(request, out) {
            return;
        }
        let cached = self.shared.borrowed(request);
        let missing = plan::missing(request, &cached);
        let loaded: Vec<_> = load(&missing)
            .into_iter()
            .filter(|(_, values)| !values.is_empty())
            .map(|(start, values)| (start, Value::from_vec(values)))
            .collect();
        plan::copy(request, &cached, &loaded, out);
        if !retain {
            return;
        }

        // Keep exactly the missing ranges, not decoder overread. Whole matching
        // buffers move into the cache without another allocation or copy.
        let exact = loaded.len() == missing.len()
            && loaded.iter().zip(&missing).all(|((start, values), range)| {
                *start == range.start && values.len() == range.len()
            });
        let offers = if exact {
            loaded
        } else {
            missing
                .into_iter()
                .map(|range| {
                    let mut values = Vec::with_capacity(range.len());
                    plan::copy(
                        Request::Range(range.start, range.end),
                        &[],
                        &loaded,
                        &mut values,
                    );
                    (range.start, Value::from_vec(values))
                })
                .collect()
        };
        // Drop temporary reader handles before merging, so an unleased cached
        // buffer can extend in place without copy-on-write.
        drop(cached);
        for (start, values) in offers {
            self.shared.insert(start, values);
        }
    }

    pub(crate) fn try_for_each_chunk<E>(
        &self,
        from: usize,
        to: usize,
        mut load: impl FnMut(&[Range<usize>]) -> Vec<(usize, Vec<T>)>,
        mut each: impl FnMut(usize, &[T]) -> FoldResult<(), E>,
    ) -> FoldResult<(), E> {
        if from >= to {
            return Ok(());
        }
        let cached = self.shared.borrowed(Request::Range(from, to));
        let retain = self.admissible(Request::Range(from, to));
        let mut at = from;
        let mut scratch = Vec::new();
        for (start, values) in &cached {
            let gap_end = (*start).min(to);
            while at < gap_end {
                let end = at.saturating_add(READ_CHUNK_SIZE).min(gap_end);
                scratch.clear();
                self.read_request(Request::Range(at, end), &mut scratch, &mut load, retain);
                each(at, &scratch)?;
                at = end;
            }
            let end = (*start + values.len()).min(to);
            if at < end {
                each(at, &values.slice()[at - *start..end - *start])?;
                at = end;
            }
        }
        while at < to {
            let end = at.saturating_add(READ_CHUNK_SIZE).min(to);
            scratch.clear();
            self.read_request(Request::Range(at, end), &mut scratch, &mut load, retain);
            each(at, &scratch)?;
            at = end;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
