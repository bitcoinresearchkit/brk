use std::{convert::Infallible, marker::PhantomData, sync::Arc};

use brk_types::{Height, StoredU64};
use vecdb::{
    AnyVec, BinaryTransform, CachedBoxedVec, CheckedSub, PrintableIndex, READ_CHUNK_SIZE,
    ReadableBoxedVec, ReadableVec, TypedVec, VecIndex, VecValue, Version, short_type_name,
};

use super::SparseRead;
use crate::CachedBlockCountReader;

pub struct LazyRollingRatioWithCachedBlockCount<T, F>
where
    T: VecValue,
{
    name: Arc<str>,
    base_version: Version,
    numerator: ReadableBoxedVec<Height, StoredU64>,
    denominator: CachedBlockCountReader,
    window_starts: CachedBoxedVec<Height, Height>,
    _output: PhantomData<(T, F)>,
}

impl<T, F> LazyRollingRatioWithCachedBlockCount<T, F>
where
    T: VecValue,
{
    pub fn new(
        name: &str,
        version: Version,
        numerator: ReadableBoxedVec<Height, StoredU64>,
        denominator: CachedBlockCountReader,
        window_starts: CachedBoxedVec<Height, Height>,
    ) -> Self {
        Self {
            name: Arc::from(name),
            base_version: version,
            numerator,
            denominator,
            window_starts,
            _output: PhantomData,
        }
    }
}

impl<T, F> LazyRollingRatioWithCachedBlockCount<T, F>
where
    T: VecValue,
    F: BinaryTransform<StoredU64, StoredU64, T>,
{
    #[inline(always)]
    fn previous_index(start: Height) -> Option<usize> {
        start.to_usize().checked_sub(1)
    }

    fn try_fold_values<B, E>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: impl FnMut(B, T) -> Result<B, E>,
    ) -> Result<B, E> {
        let window_starts = self.window_starts.snapshot();
        let to = to
            .min(self.numerator.len())
            .min(self.denominator.len())
            .min(window_starts.len());
        if from >= to {
            return Ok(init);
        }

        let starts = &window_starts[from..to];
        let first_previous = starts.iter().find_map(|start| Self::previous_index(*start));
        let last_previous = starts
            .iter()
            .rev()
            .find_map(|start| Self::previous_index(*start));

        if let Some((first_previous, last_previous)) = first_previous.zip(last_previous)
            && last_previous + 1 >= from
        {
            let read_from = first_previous.min(from);
            let numerators = self.numerator.collect_range_dyn(read_from, to);
            let mut offset = 0;
            return self.denominator.try_fold_rolling_sum(
                from,
                starts,
                init,
                |accumulator, denominator| {
                    let index = from + offset;
                    let current = numerators[index - read_from];
                    let previous = Self::previous_index(starts[offset])
                        .map(|previous| numerators[previous - read_from])
                        .unwrap_or_default();
                    let value = F::apply(
                        current.checked_sub(previous).unwrap_or_default(),
                        denominator,
                    );
                    offset += 1;
                    fold(accumulator, value)
                },
            );
        }

        let current = self.numerator.collect_range_dyn(from, to);
        let previous = first_previous
            .zip(last_previous)
            .map(|(first, last)| (first, self.numerator.collect_range_dyn(first, last + 1)));
        let mut offset = 0;

        self.denominator
            .try_fold_rolling_sum(from, starts, init, |accumulator, denominator| {
                let previous = Self::previous_index(starts[offset])
                    .and_then(|index| {
                        previous
                            .as_ref()
                            .map(|(first, values)| values[index - first])
                    })
                    .unwrap_or_default();
                let value = F::apply(
                    current[offset].checked_sub(previous).unwrap_or_default(),
                    denominator,
                );
                offset += 1;
                fold(accumulator, value)
            })
    }

    fn for_each_output(&self, from: usize, to: usize, mut each: impl FnMut(usize, &[T])) {
        let size = READ_CHUNK_SIZE;
        let mut at = from;
        let mut output = Vec::new();
        self.try_fold_values(from, to, (), |(), value| {
            output.push(value);
            if output.len() == size {
                each(at, &output);
                at += output.len();
                output.clear();
            }
            Ok::<_, Infallible>(())
        })
        .unwrap();
        if !output.is_empty() {
            each(at, &output);
        }
    }
}

impl<T, F> Clone for LazyRollingRatioWithCachedBlockCount<T, F>
where
    T: VecValue,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            base_version: self.base_version,
            numerator: self.numerator.clone(),
            denominator: self.denominator.clone(),
            window_starts: self.window_starts.clone(),
            _output: PhantomData,
        }
    }
}

impl<T, F> AnyVec for LazyRollingRatioWithCachedBlockCount<T, F>
where
    T: VecValue,
    F: BinaryTransform<StoredU64, StoredU64, T> + Send + Sync,
{
    fn version(&self) -> Version {
        self.base_version
            + self.numerator.version()
            + self.denominator.version()
            + self.window_starts.version()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn len(&self) -> usize {
        self.numerator
            .len()
            .min(self.denominator.len())
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

impl<T, F> TypedVec for LazyRollingRatioWithCachedBlockCount<T, F>
where
    T: VecValue,
    F: BinaryTransform<StoredU64, StoredU64, T> + Send + Sync,
{
    type I = Height;
    type T = T;
}

impl<T, F> ReadableVec<Height, T> for LazyRollingRatioWithCachedBlockCount<T, F>
where
    T: VecValue,
    F: BinaryTransform<StoredU64, StoredU64, T> + Send + Sync,
{
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<T>) {
        buf.reserve(to.min(self.len()).saturating_sub(from));
        self.try_fold_values(from, to, (), |(), value| {
            buf.push(value);
            Ok::<_, Infallible>(())
        })
        .unwrap();
    }

    fn for_each_chunk_at(&self, from: usize, to: usize, each: &mut dyn FnMut(usize, &[T])) {
        self.for_each_output(from, to, each);
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, each: &mut dyn FnMut(T)) {
        self.for_each_output(from, to, |_, values| {
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

        let start = self.window_starts.snapshot()[index];
        let previous = Self::previous_index(start);
        Some(F::apply(
            self.numerator
                .collect_one_at(index)?
                .checked_sub(
                    previous
                        .and_then(|index| self.numerator.collect_one_at(index))
                        .unwrap_or_default(),
                )
                .unwrap_or_default(),
            self.denominator.rolling_sum_at(start.to_usize(), index)?,
        ))
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

        let window_starts = self.window_starts.snapshot();
        let numerator = SparseRead::new(&*self.numerator, indices, |index| {
            Self::previous_index(window_starts[index])
        });

        out.reserve(indices.len());
        self.denominator.for_each_sorted_rolling_sum(
            indices,
            &window_starts,
            |output, denominator| {
                out.push(F::apply(
                    numerator
                        .current(output)
                        .checked_sub(numerator.previous(output).unwrap_or_default())
                        .unwrap_or_default(),
                    denominator,
                ));
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use brk_types::{PartsPerMillion32, StoredU16};
    use vecdb::{
        AnyStoredVec, CachedVec, Database, EagerVec, ImportableVec, PcoVec, ReadableCloneableVec,
        WritableVec,
    };

    use super::*;
    use crate::{LazyRatioWithCachedBlockCount, RatioU64};

    #[test]
    fn derives_cumulative_and_rolling_ratios_from_compact_counts() {
        let directory = tempfile::tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let mut numerator: EagerVec<PcoVec<Height, StoredU64>> =
            EagerVec::forced_import(&db, "numerator", Version::ONE).unwrap();
        let mut denominator: EagerVec<PcoVec<Height, StoredU16>> =
            EagerVec::forced_import(&db, "denominator", Version::ONE).unwrap();
        let mut starts: EagerVec<PcoVec<Height, Height>> =
            EagerVec::forced_import(&db, "starts", Version::ONE).unwrap();

        for value in [10_u64, 30, 60, 100] {
            numerator.push(StoredU64::from(value));
        }
        for value in [20_u16, 30, 40, 50] {
            denominator.push(StoredU16::new(value));
        }
        for value in [0, 0, 1, 2] {
            starts.push(Height::new(value));
        }
        numerator.write().unwrap();
        denominator.write().unwrap();
        starts.write().unwrap();

        let denominator = CachedBlockCountReader::new(
            CachedVec::wrap(denominator).read_only_cached_boxed_clone(),
        );
        let cumulative =
            LazyRatioWithCachedBlockCount::<PartsPerMillion32, RatioU64<PartsPerMillion32>>::new(
                "cumulative",
                Version::ONE,
                numerator.read_only_boxed_clone(),
                denominator.clone(),
            );
        let starts = CachedVec::wrap(starts);
        let rolling = LazyRollingRatioWithCachedBlockCount::<
            PartsPerMillion32,
            RatioU64<PartsPerMillion32>,
        >::new(
            "rolling",
            Version::ONE,
            numerator.read_only_boxed_clone(),
            denominator,
            starts.read_only_cached_boxed_clone(),
        );

        assert_eq!(
            cumulative.collect_range_at(0, 4),
            [0.5, 0.6, 2.0 / 3.0, 5.0 / 7.0].map(PartsPerMillion32::from)
        );
        assert_eq!(
            cumulative.read_sorted_at(&[0, 2, 2, 4]),
            [0.5, 2.0 / 3.0, 2.0 / 3.0].map(PartsPerMillion32::from)
        );
        assert_eq!(
            rolling.collect_range_at(0, 4),
            [0.5, 0.6, 5.0 / 7.0, 7.0 / 9.0].map(PartsPerMillion32::from)
        );
        assert_eq!(
            rolling.read_sorted_at(&[0, 2, 2, 3, 4]),
            [0.5, 5.0 / 7.0, 5.0 / 7.0, 7.0 / 9.0].map(PartsPerMillion32::from)
        );
        assert_eq!(
            rolling.read_sorted_at(&[3]),
            [7.0 / 9.0].map(PartsPerMillion32::from)
        );
    }
}
