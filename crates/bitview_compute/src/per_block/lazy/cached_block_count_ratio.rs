use std::{convert::Infallible, marker::PhantomData, sync::Arc};

use brk_types::{Height, StoredU64};
use vecdb::{
    AnyVec, BinaryTransform, Cursor, PrintableIndex, READ_CHUNK_SIZE, ReadableBoxedVec,
    ReadableVec, TypedVec, VecValue, Version, short_type_name,
};

use crate::CachedBlockCountReader;

pub struct LazyRatioWithCachedBlockCount<T, F>
where
    T: VecValue,
{
    name: Arc<str>,
    base_version: Version,
    numerator: ReadableBoxedVec<Height, StoredU64>,
    denominator: CachedBlockCountReader,
    _output: PhantomData<(T, F)>,
}

impl<T, F> LazyRatioWithCachedBlockCount<T, F>
where
    T: VecValue,
{
    pub fn new(
        name: &str,
        version: Version,
        numerator: ReadableBoxedVec<Height, StoredU64>,
        denominator: CachedBlockCountReader,
    ) -> Self {
        Self {
            name: Arc::from(name),
            base_version: version,
            numerator,
            denominator,
            _output: PhantomData,
        }
    }
}

impl<T, F> LazyRatioWithCachedBlockCount<T, F>
where
    T: VecValue,
    F: BinaryTransform<StoredU64, StoredU64, T> + Send + Sync,
{
    fn try_fold_values<B, E>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: impl FnMut(B, T) -> Result<B, E>,
    ) -> Result<B, E> {
        let to = to.min(self.len());
        if from >= to {
            return Ok(init);
        }

        let mut numerators = self.numerator.collect_range_dyn(from, to).into_iter();
        self.denominator
            .try_fold_range_at(from, to, init, |accumulator, denominator| {
                fold(
                    accumulator,
                    F::apply(numerators.next().unwrap(), denominator),
                )
            })
    }
}

impl<T, F> Clone for LazyRatioWithCachedBlockCount<T, F>
where
    T: VecValue,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            base_version: self.base_version,
            numerator: self.numerator.clone(),
            denominator: self.denominator.clone(),
            _output: PhantomData,
        }
    }
}

impl<T, F> AnyVec for LazyRatioWithCachedBlockCount<T, F>
where
    T: VecValue,
    F: BinaryTransform<StoredU64, StoredU64, T> + Send + Sync,
{
    fn version(&self) -> Version {
        self.base_version + self.numerator.version() + self.denominator.version()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn len(&self) -> usize {
        self.numerator.len().min(self.denominator.len())
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

impl<T, F> TypedVec for LazyRatioWithCachedBlockCount<T, F>
where
    T: VecValue,
    F: BinaryTransform<StoredU64, StoredU64, T> + Send + Sync,
{
    type I = Height;
    type T = T;
}

impl<T, F> ReadableVec<Height, T> for LazyRatioWithCachedBlockCount<T, F>
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
        Some(F::apply(
            self.numerator.collect_one_at(index)?,
            self.denominator.cumulative_at(index)?,
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
        let denominators = self.denominator.read_sorted_at(indices);
        let mut numerators = Cursor::new(&*self.numerator);

        out.reserve(indices.len());
        for (&index, denominator) in indices.iter().zip(denominators) {
            let Some(numerator) = numerators.get(index) else {
                continue;
            };
            out.push(F::apply(numerator, denominator));
        }
    }
}
