use std::{convert::Infallible, marker::PhantomData, sync::Arc};

use bitview_traversable::{Traversable, TreeNode, make_leaf};
use brk_types::Height;
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{
    AnyExportableVec, AnyVec, BinaryTransform, CachedBoxedVec, CheckedSub, Formattable,
    PrintableIndex, READ_CHUNK_SIZE, ReadableBoxedVec, ReadableVec, TypedVec, VecIndex, VecValue,
    Version, short_type_name,
};

use super::{SparseRead, rolling_inputs::for_each_rolling_input};

/// Rolling transform derived from one cumulative source and one cached
/// cumulative operand.
///
/// Only `source` may read from disk. The cached operand and window starts are
/// pinned in-memory snapshots shared with their authoritative sources.
pub struct LazyRollingRatioVec<S, C, T, F>
where
    S: VecValue,
    C: VecValue,
    T: VecValue,
{
    name: Arc<str>,
    base_version: Version,
    source: ReadableBoxedVec<Height, S>,
    cached: CachedBoxedVec<Height, C>,
    cached_transform: fn(Height, C) -> C,
    window_starts: CachedBoxedVec<Height, Height>,
    _marker: PhantomData<(T, F)>,
}

impl<S, C, T, F> LazyRollingRatioVec<S, C, T, F>
where
    S: VecValue + CheckedSub + Default,
    C: VecValue + CheckedSub + Default,
    T: VecValue,
    F: BinaryTransform<S, C, T>,
{
    #[inline(always)]
    fn identity_cached(_: Height, value: C) -> C {
        value
    }

    pub fn new(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<Height, S>,
        cached: CachedBoxedVec<Height, C>,
        window_starts: CachedBoxedVec<Height, Height>,
    ) -> Self {
        Self::with_cached_transform(
            name,
            version,
            source,
            cached,
            window_starts,
            Self::identity_cached,
        )
    }

    pub fn with_cached_transform(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<Height, S>,
        cached: CachedBoxedVec<Height, C>,
        window_starts: CachedBoxedVec<Height, Height>,
        cached_transform: fn(Height, C) -> C,
    ) -> Self {
        Self {
            name: Arc::from(name),
            base_version: version,
            source,
            cached,
            cached_transform,
            window_starts,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    fn previous_index(start: Height) -> Option<usize> {
        start.to_usize().checked_sub(1)
    }

    #[inline(always)]
    fn compute(
        &self,
        index: usize,
        previous: Option<usize>,
        source_current: S,
        source_previous: S,
        cached_current: C,
        cached_previous: C,
    ) -> T {
        F::apply(
            source_current
                .checked_sub(source_previous)
                .unwrap_or_default(),
            (self.cached_transform)(Height::from(index), cached_current)
                .checked_sub(
                    previous
                        .map(|index| (self.cached_transform)(Height::from(index), cached_previous))
                        .unwrap_or_default(),
                )
                .unwrap_or_default(),
        )
    }

    fn try_fold_values<B, E>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: impl FnMut(B, T) -> Result<B, E>,
    ) -> Result<B, E> {
        let mut accumulator = Some(Ok(init));
        self.for_each_input(from, to, |at, current, base, previous, cached, starts| {
            accumulator = Some(accumulator.take().unwrap().and_then(|accumulator| {
                self.transformed_values(at, current, base, previous, cached, starts)
                    .try_fold(accumulator, &mut fold)
            }));
        });
        accumulator.unwrap()
    }

    fn for_each_input(
        &self,
        from: usize,
        to: usize,
        mut visit: impl FnMut(usize, &[S], usize, &[S], &[C], &[Height]),
    ) {
        let cached = self.cached.snapshot();
        let starts = self.window_starts.snapshot();
        let to = to
            .min(self.source.len())
            .min(cached.len())
            .min(starts.len());
        if from >= to {
            return;
        }
        for_each_rolling_input(
            &self.source,
            from,
            to,
            &starts[from..to],
            |at, current, base, previous| {
                visit(
                    at,
                    current,
                    base,
                    previous,
                    &cached,
                    &starts[at..at + current.len()],
                );
            },
        );
    }

    fn transformed_values<'a>(
        &'a self,
        at: usize,
        current: &'a [S],
        base: usize,
        previous: &'a [S],
        cached: &'a [C],
        starts: &'a [Height],
    ) -> impl Iterator<Item = T> + 'a {
        current
            .iter()
            .zip(starts)
            .enumerate()
            .map(move |(offset, (current, start))| {
                let index = at + offset;
                let prior = Self::previous_index(*start);
                self.compute(
                    index,
                    prior,
                    current.clone(),
                    prior
                        .map(|i| previous[i - base].clone())
                        .unwrap_or_default(),
                    cached[index].clone(),
                    prior.map(|i| cached[i].clone()).unwrap_or_default(),
                )
            })
    }
}

impl<S, C, T, F> Clone for LazyRollingRatioVec<S, C, T, F>
where
    S: VecValue,
    C: VecValue,
    T: VecValue,
{
    fn clone(&self) -> Self {
        Self {
            name: Arc::clone(&self.name),
            base_version: self.base_version,
            source: self.source.clone(),
            cached: self.cached.clone(),
            cached_transform: self.cached_transform,
            window_starts: self.window_starts.clone(),
            _marker: PhantomData,
        }
    }
}

impl<S, C, T, F> AnyVec for LazyRollingRatioVec<S, C, T, F>
where
    S: VecValue + CheckedSub + Default,
    C: VecValue + CheckedSub + Default,
    T: VecValue,
    F: BinaryTransform<S, C, T> + Send + Sync,
{
    fn version(&self) -> Version {
        self.base_version
            + self.source.version()
            + self.cached.version()
            + self.window_starts.version()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn len(&self) -> usize {
        self.source
            .len()
            .min(self.cached.len())
            .min(self.window_starts.len())
    }

    fn index_type_to_string(&self) -> &'static str {
        <Height as PrintableIndex>::to_string()
    }

    fn region_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn value_type_to_size_of(&self) -> usize {
        size_of::<T>()
    }

    fn value_type_to_string(&self) -> &'static str {
        short_type_name::<T>()
    }
}

impl<S, C, T, F> TypedVec for LazyRollingRatioVec<S, C, T, F>
where
    S: VecValue + CheckedSub + Default,
    C: VecValue + CheckedSub + Default,
    T: VecValue,
    F: BinaryTransform<S, C, T> + Send + Sync,
{
    type I = Height;
    type T = T;
}

impl<S, C, T, F> ReadableVec<Height, T> for LazyRollingRatioVec<S, C, T, F>
where
    S: VecValue + CheckedSub + Default,
    C: VecValue + CheckedSub + Default,
    T: VecValue,
    F: BinaryTransform<S, C, T> + Send + Sync,
{
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<T>) {
        buf.reserve(to.min(self.len()).saturating_sub(from));
        self.for_each_input(from, to, |at, current, base, previous, cached, starts| {
            buf.extend(self.transformed_values(at, current, base, previous, cached, starts));
        });
    }

    fn cursor_chunk_size(&self) -> usize {
        self.source.cursor_chunk_size()
    }

    fn for_each_chunk_at(&self, from: usize, to: usize, each: &mut dyn FnMut(usize, &[T])) {
        let size = self.cursor_chunk_size().clamp(1, READ_CHUNK_SIZE);
        let mut output = Vec::new();
        self.for_each_input(from, to, |at, current, base, previous, cached, starts| {
            for (chunk, current) in current.chunks(size).enumerate() {
                let offset = chunk * size;
                output.clear();
                output.extend(self.transformed_values(
                    at + offset,
                    current,
                    base,
                    previous,
                    cached,
                    &starts[offset..offset + current.len()],
                ));
                each(at + offset, &output);
            }
        });
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, each: &mut dyn FnMut(T)) {
        self.for_each_chunk_at(from, to, &mut |_, values| {
            values.iter().cloned().for_each(&mut *each)
        });
    }

    fn fold_range_at<B, G: FnMut(B, T) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: G,
    ) -> B {
        self.try_fold_values(from, to, init, |accumulator, value| {
            Ok::<_, Infallible>(fold(accumulator, value))
        })
        .unwrap()
    }

    fn try_fold_range_at<B, E, G: FnMut(B, T) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        fold: G,
    ) -> Result<B, E> {
        self.try_fold_values(from, to, init, fold)
    }

    fn collect_one_at(&self, index: usize) -> Option<T> {
        if index >= self.len() {
            return None;
        }

        let cached = self.cached.snapshot();
        let window_starts = self.window_starts.snapshot();
        let previous = Self::previous_index(window_starts[index]);
        Some(
            self.compute(
                index,
                previous,
                self.source.collect_one_at(index)?,
                previous
                    .and_then(|index| self.source.collect_one_at(index))
                    .unwrap_or_default(),
                cached[index].clone(),
                previous
                    .map(|index| cached[index].clone())
                    .unwrap_or_default(),
            ),
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

        let indices = &indices[..indices.partition_point(|&index| index < self.len())];
        if indices.is_empty() {
            return;
        }

        let cached = self.cached.snapshot();
        let window_starts = self.window_starts.snapshot();
        let source = SparseRead::new(&*self.source, indices, |index| {
            Self::previous_index(window_starts[index])
        });

        out.reserve(indices.len());
        for (output, &index) in indices.iter().enumerate() {
            let previous = Self::previous_index(window_starts[index]);
            out.push(
                self.compute(
                    index,
                    previous,
                    source.current(output),
                    source.previous(output).unwrap_or_default(),
                    cached[index].clone(),
                    previous
                        .map(|index| cached[index].clone())
                        .unwrap_or_default(),
                ),
            );
        }
    }
}

impl<S, C, T, F> Traversable for LazyRollingRatioVec<S, C, T, F>
where
    S: VecValue + CheckedSub + Default,
    C: VecValue + CheckedSub + Default,
    T: VecValue + Formattable + Serialize + JsonSchema,
    F: BinaryTransform<S, C, T> + Send + Sync,
{
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        std::iter::once(self as &dyn AnyExportableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        make_leaf::<Height, T, _>(self)
    }
}

#[cfg(test)]
mod tests {
    use brk_types::{PartsPerMillion32, Sats};
    use vecdb::{
        AnyStoredVec, CachedVec, Database, EagerVec, ImportableVec, PcoVec, ReadableCloneableVec,
        ReadableVec, WritableVec,
    };

    use super::*;
    use crate::RatioSats;

    fn double(_: Height, value: Sats) -> Sats {
        value + value
    }

    #[test]
    fn derives_rolling_ratios_from_one_source_and_cached_denominator() {
        let directory = tempfile::tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let mut source: EagerVec<PcoVec<Height, Sats>> =
            EagerVec::forced_import(&db, "source", Version::ONE).unwrap();
        let mut denominator: EagerVec<PcoVec<Height, Sats>> =
            EagerVec::forced_import(&db, "denominator", Version::ONE).unwrap();
        let mut starts: EagerVec<PcoVec<Height, Height>> =
            EagerVec::forced_import(&db, "starts", Version::ONE).unwrap();

        for value in [10, 30, 60, 100] {
            source.push(Sats::new(value));
        }
        for value in [20, 50, 90, 140] {
            denominator.push(Sats::new(value));
        }
        for value in [0, 0, 1, 2] {
            starts.push(Height::new(value));
        }
        source.write().unwrap();
        denominator.write().unwrap();
        starts.write().unwrap();

        let denominator = CachedVec::wrap(denominator);
        let starts = CachedVec::wrap(starts);
        let ratio = LazyRollingRatioVec::<
            Sats,
            Sats,
            PartsPerMillion32,
            RatioSats<PartsPerMillion32>,
        >::new(
            "ratio",
            Version::ONE,
            source.read_only_boxed_clone(),
            denominator.read_only_cached_boxed_clone(),
            starts.read_only_cached_boxed_clone(),
        );

        assert_eq!(
            ratio.collect_range(Height::ZERO, Height::new(4)),
            vec![
                PartsPerMillion32::from(0.5),
                PartsPerMillion32::from(0.6),
                PartsPerMillion32::from(50.0 / 70.0),
                PartsPerMillion32::from(70.0 / 90.0),
            ],
        );
        assert_eq!(
            ratio.collect_range(Height::new(3), Height::new(4)),
            [PartsPerMillion32::from(70.0 / 90.0)],
        );
        assert_eq!(
            ratio.read_sorted_at(&[0, 2, 2, 3, 4]),
            [
                PartsPerMillion32::from(0.5),
                PartsPerMillion32::from(50.0 / 70.0),
                PartsPerMillion32::from(50.0 / 70.0),
                PartsPerMillion32::from(70.0 / 90.0),
            ],
        );
        assert_eq!(
            ratio.read_sorted_at(&[3]),
            [PartsPerMillion32::from(70.0 / 90.0)],
        );

        let transformed = LazyRollingRatioVec::<
            Sats,
            Sats,
            PartsPerMillion32,
            RatioSats<PartsPerMillion32>,
        >::with_cached_transform(
            "transformed",
            Version::ONE,
            source.read_only_boxed_clone(),
            denominator.read_only_cached_boxed_clone(),
            starts.read_only_cached_boxed_clone(),
            double,
        );
        assert_eq!(
            transformed.collect_range(Height::ZERO, Height::new(4)),
            vec![
                PartsPerMillion32::from(0.25),
                PartsPerMillion32::from(0.3),
                PartsPerMillion32::from(25.0 / 70.0),
                PartsPerMillion32::from(35.0 / 90.0),
            ],
        );
        assert_eq!(
            transformed.read_sorted_at(&[0, 2, 2, 3, 4]),
            [
                PartsPerMillion32::from(0.25),
                PartsPerMillion32::from(25.0 / 70.0),
                PartsPerMillion32::from(25.0 / 70.0),
                PartsPerMillion32::from(35.0 / 90.0),
            ],
        );
    }
}
