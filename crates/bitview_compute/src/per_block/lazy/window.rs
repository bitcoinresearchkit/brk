use std::{iter::once, sync::Arc};

use bitview_traversable::{Traversable, TreeNode, make_leaf};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{
    AnyExportableVec, AnyVec, Formattable, READ_CHUNK_SIZE, ReadableBoxedVec, ReadableVec,
    TypedVec, VecIndex, VecValue, Version, short_type_name,
};

use super::SparseRead;

struct WindowInputs<'a, I, S> {
    at: usize,
    starts: &'a [I],
    previous: &'a [S],
    previous_from: usize,
    inclusive: bool,
}

trait WindowTransform<I, S, T>: Send + Sync {
    fn apply(&self, current: S, previous: S, count: usize) -> T;
    fn append(&self, current: &[S], inputs: &WindowInputs<I, S>, out: &mut Vec<T>);
}

impl<I, S, T, F> WindowTransform<I, S, T> for F
where
    I: VecIndex,
    S: VecValue + Default,
    F: Fn(S, S, usize) -> T + Send + Sync,
{
    fn apply(&self, current: S, previous: S, count: usize) -> T {
        self(current, previous, count)
    }
    fn append(&self, current: &[S], inputs: &WindowInputs<I, S>, out: &mut Vec<T>) {
        out.extend(
            (inputs.at..inputs.at + current.len())
                .zip(current.iter().cloned())
                .zip(inputs.starts)
                .map(|((at, current), start)| {
                    let start = start.to_usize();
                    let ago = if inputs.inclusive {
                        start.checked_sub(1)
                    } else {
                        Some(start)
                    };
                    let previous = ago
                        .map(|i| inputs.previous[i - inputs.previous_from].clone())
                        .unwrap_or_default();
                    self(
                        current,
                        previous,
                        at - start + usize::from(inputs.inclusive),
                    )
                }),
        );
    }
}

/// Lazily combines the current and window-start values of one metric source.
///
/// `window_starts` is index metadata, not a second metric source. Reads are
/// split into current and lookback ranges so long windows do not force reading
/// the entire gap between them.
pub struct LazyWindowVec<I, S, T>
where
    I: VecIndex,
    S: VecValue,
    T: VecValue,
{
    name: Arc<str>,
    base_version: Version,
    source: ReadableBoxedVec<I, S>,
    window_starts: ReadableBoxedVec<I, I>,
    inclusive: bool,
    compute: Arc<dyn WindowTransform<I, S, T>>,
}

impl<I, S, T> LazyWindowVec<I, S, T>
where
    I: VecIndex,
    S: VecValue + Default,
    T: VecValue,
{
    pub fn new(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<I, S>,
        window_starts: impl ReadableVec<I, I> + Clone + 'static,
        inclusive: bool,
        compute: impl Fn(S, S, usize) -> T + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: Arc::from(name),
            base_version: version,
            source,
            window_starts: ReadableBoxedVec::new(window_starts),
            inclusive,
            compute: Arc::new(compute),
        }
    }

    #[inline]
    fn ago_index(&self, start: usize) -> Option<usize> {
        if self.inclusive {
            start.checked_sub(1)
        } else {
            Some(start)
        }
    }

    #[inline]
    fn count(&self, current: usize, start: usize) -> usize {
        if self.inclusive {
            current - start + 1
        } else {
            current - start
        }
    }

    fn try_fold_window<B, E>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: impl FnMut(B, T) -> Result<B, E>,
    ) -> Result<B, E> {
        let mut accumulator = init;
        let window_starts = self.window_starts.snapshot();
        let to = to.min(self.len()).min(window_starts.len());
        if from >= to {
            return Ok(accumulator);
        }

        let starts = &window_starts[from..to];
        let current = self.source.collect_range_dyn(from, to);

        let first_ago = starts
            .iter()
            .find_map(|start| self.ago_index(start.to_usize()));
        let last_ago = starts
            .iter()
            .rev()
            .find_map(|start| self.ago_index(start.to_usize()));
        let ago = first_ago
            .zip(last_ago)
            .map(|(first, last)| self.source.collect_range_dyn(first, last + 1));

        for (offset, (current, start)) in current.into_iter().zip(starts).enumerate() {
            let start = start.to_usize();
            let previous = self
                .ago_index(start)
                .and_then(|index| {
                    ago.as_ref()
                        .zip(first_ago)
                        .map(|(values, first)| values[index - first].clone())
                })
                .unwrap_or_default();
            accumulator = fold(
                accumulator,
                self.compute
                    .apply(current, previous, self.count(from + offset, start)),
            )?;
        }
        Ok(accumulator)
    }

    fn for_each_input(
        &self,
        from: usize,
        to: usize,
        mut each: impl FnMut(&[S], WindowInputs<I, S>),
    ) {
        let window_starts = self.window_starts.snapshot();
        let to = to.min(self.len()).min(window_starts.len());
        if from >= to {
            return;
        }
        let starts = &window_starts[from..to];
        let first = starts
            .iter()
            .find_map(|start| self.ago_index(start.to_usize()));
        let last = starts
            .iter()
            .rev()
            .find_map(|start| self.ago_index(start.to_usize()));
        let (previous_from, previous_to) = first
            .zip(last)
            .map(|(first, last)| (first, last + 1))
            .unwrap_or((0, 0));
        // Do not recursively read the source while it lends current chunks;
        // columnar sources may hold their publication lock in the callback.
        let previous = self.source.collect_range_dyn(previous_from, previous_to);
        self.source.for_each_chunk_at(from, to, &mut |at, current| {
            each(
                current,
                WindowInputs {
                    at,
                    starts: &window_starts[at..at + current.len()],
                    previous: &previous,
                    previous_from,
                    inclusive: self.inclusive,
                },
            );
        });
    }
}

impl<I, S, T> Clone for LazyWindowVec<I, S, T>
where
    I: VecIndex,
    S: VecValue,
    T: VecValue,
{
    fn clone(&self) -> Self {
        Self {
            name: Arc::clone(&self.name),
            base_version: self.base_version,
            source: self.source.clone(),
            window_starts: self.window_starts.clone(),
            inclusive: self.inclusive,
            compute: Arc::clone(&self.compute),
        }
    }
}

impl<I, S, T> AnyVec for LazyWindowVec<I, S, T>
where
    I: VecIndex,
    S: VecValue,
    T: VecValue,
{
    fn version(&self) -> Version {
        self.base_version + self.source.version() + self.window_starts.version()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn index_type_to_string(&self) -> &'static str {
        I::to_string()
    }

    fn len(&self) -> usize {
        self.source.len()
    }

    fn value_type_to_size_of(&self) -> usize {
        size_of::<T>()
    }

    fn value_type_to_string(&self) -> &'static str {
        short_type_name::<T>()
    }

    fn region_names(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<I, S, T> TypedVec for LazyWindowVec<I, S, T>
where
    I: VecIndex,
    S: VecValue,
    T: VecValue,
{
    type I = I;
    type T = T;
}

impl<I, S, T> ReadableVec<I, T> for LazyWindowVec<I, S, T>
where
    I: VecIndex,
    S: VecValue + Default,
    T: VecValue,
{
    fn cursor_chunk_size(&self) -> usize {
        self.source.cursor_chunk_size()
    }

    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<T>) {
        buf.reserve(to.min(self.len()).saturating_sub(from));
        self.for_each_input(from, to, |current, inputs| {
            self.compute.append(current, &inputs, buf);
        });
    }

    fn for_each_chunk_at(&self, from: usize, to: usize, f: &mut dyn FnMut(usize, &[T])) {
        let chunk_size = self.cursor_chunk_size().clamp(1, READ_CHUNK_SIZE);
        let mut output = Vec::new();
        self.for_each_input(from, to, |current, inputs| {
            for (chunk, current) in current.chunks(chunk_size).enumerate() {
                let offset = chunk * chunk_size;
                let inputs = WindowInputs {
                    at: inputs.at + offset,
                    starts: &inputs.starts[offset..offset + current.len()],
                    ..inputs
                };
                output.clear();
                self.compute.append(current, &inputs, &mut output);
                f(inputs.at, &output);
            }
        });
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(T)) {
        self.for_each_chunk_at(from, to, &mut |_, values| {
            values.iter().cloned().for_each(&mut *f);
        });
    }

    fn fold_range_at<B, F: FnMut(B, T) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> B {
        let mut acc = Some(init);
        self.for_each_chunk_at(from, to, &mut |_, values| {
            acc = Some(values.iter().cloned().fold(acc.take().unwrap(), &mut f));
        });
        acc.unwrap()
    }

    fn try_fold_range_at<B, E, F: FnMut(B, T) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        f: F,
    ) -> Result<B, E> {
        self.try_fold_window(from, to, init, f)
    }

    fn collect_one_at(&self, index: usize) -> Option<T> {
        let window_starts = self.window_starts.snapshot();
        let start = window_starts.get(index)?.to_usize();
        let current = self.source.collect_one_at(index)?;
        let previous = self
            .ago_index(start)
            .and_then(|index| self.source.collect_one_at(index))
            .unwrap_or_default();
        Some(
            self.compute
                .apply(current, previous, self.count(index, start)),
        )
    }

    fn read_sorted_into_at(&self, indices: &[usize], out: &mut Vec<T>) {
        match indices {
            [] => return,
            &[index] => {
                if let Some(value) = self.collect_one_at(index) {
                    out.push(value);
                }
                return;
            }
            _ => {}
        }

        let window_starts = self.window_starts.snapshot();
        let len = self.len().min(window_starts.len());
        let indices = &indices[..indices.partition_point(|&index| index < len)];
        if indices.is_empty() {
            return;
        }

        let source = SparseRead::new(&*self.source, indices, |index| {
            self.ago_index(window_starts[index].to_usize())
        });

        out.reserve(indices.len());
        for (output, &index) in indices.iter().enumerate() {
            let start = window_starts[index].to_usize();
            let previous = source.previous(output).unwrap_or_default();
            out.push(self.compute.apply(
                source.current(output),
                previous,
                self.count(index, start),
            ));
        }
    }
}

impl<I, S, T> Traversable for LazyWindowVec<I, S, T>
where
    I: VecIndex,
    S: VecValue + Default,
    T: VecValue + Formattable + Serialize + JsonSchema,
{
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        once(self as &dyn AnyExportableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        make_leaf::<I, T, _>(self)
    }
}

#[cfg(test)]
mod tests {
    use brk_types::{Height, StoredU64, Version};
    use tempfile::tempdir;
    use vecdb::{
        AnyStoredVec, CachedVec, Database, EagerVec, ImportableVec, PcoVec, ReadableCloneableVec,
        ReadableVec, WritableVec,
    };

    use super::LazyWindowVec;

    #[test]
    fn sorted_reads_batch_inclusive_and_exclusive_windows() {
        let directory = tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let mut source: EagerVec<PcoVec<Height, StoredU64>> =
            EagerVec::forced_import(&db, "source", Version::ONE).unwrap();
        let mut starts: EagerVec<PcoVec<Height, Height>> =
            EagerVec::forced_import(&db, "starts", Version::ONE).unwrap();

        for value in [10_u64, 30, 60, 100] {
            source.push(StoredU64::from(value));
        }
        for value in [0, 0, 1, 2] {
            starts.push(Height::new(value));
        }
        source.write().unwrap();
        starts.write().unwrap();

        let starts = CachedVec::wrap(starts);
        let compute = |current: StoredU64, previous: StoredU64, count: usize| {
            StoredU64::from((*current - *previous) + count as u64 * 1_000)
        };
        let exclusive = LazyWindowVec::new(
            "exclusive",
            Version::ONE,
            source.read_only_boxed_clone(),
            starts.read_only_cached_boxed_clone(),
            false,
            compute,
        );
        let inclusive = LazyWindowVec::new(
            "inclusive",
            Version::ONE,
            source.read_only_boxed_clone(),
            starts.read_only_cached_boxed_clone(),
            true,
            compute,
        );

        assert_eq!(
            exclusive.collect_range_at(0, 4),
            [0_u64, 1_020, 1_030, 1_040].map(StoredU64::from),
        );
        assert_eq!(
            exclusive.read_sorted_at(&[0, 2, 2, 3, 4]),
            [0_u64, 1_030, 1_030, 1_040].map(StoredU64::from),
        );
        assert_eq!(
            exclusive.read_sorted_at(&[3]),
            [1_040_u64].map(StoredU64::from),
        );
        assert_eq!(
            inclusive.collect_range_at(0, 4),
            [1_010_u64, 2_030, 2_050, 2_070].map(StoredU64::from),
        );
        assert_eq!(
            inclusive.read_sorted_at(&[0, 2, 2, 3, 4]),
            [1_010_u64, 2_050, 2_050, 2_070].map(StoredU64::from),
        );
    }
}
