use std::{
    any,
    sync::atomic::{AtomicUsize, Ordering},
};

use brk_exit::Exit;
use tempfile::tempdir;
use vecdb::{
    AnyVec, BytesVec, BytesVecValue, Database, EagerVec, ImportableVec, ReadableVec,
    Result as VecdbResult, VecValue, Version,
};

struct CountingSource<T> {
    values: Vec<T>,
    version: Version,
    reads: AtomicUsize,
}

impl<T> CountingSource<T> {
    fn new(values: Vec<T>) -> Self {
        Self {
            values,
            version: Version::ONE,
            reads: AtomicUsize::new(0),
        }
    }
}

impl<T: VecValue> AnyVec for CountingSource<T> {
    fn version(&self) -> Version {
        self.version
    }
    fn name(&self) -> &str {
        "source"
    }
    fn len(&self) -> usize {
        self.values.len()
    }
    fn index_type_to_string(&self) -> &'static str {
        "usize"
    }
    fn region_names(&self) -> Vec<String> {
        vec![]
    }
    fn value_type_to_size_of(&self) -> usize {
        size_of::<T>()
    }
    fn value_type_to_string(&self) -> &'static str {
        any::type_name::<T>()
    }
}

impl<T: VecValue> ReadableVec<usize, T> for CountingSource<T> {
    fn read_into_at(&self, from: usize, to: usize, out: &mut Vec<T>) {
        self.fold_range_at(from, to, (), |(), value| out.push(value));
    }
    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(T)) {
        self.fold_range_at(from, to, (), |(), value| f(value));
    }
    fn fold_range_at<B, F: FnMut(B, T) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> B {
        let end = to.min(self.len());
        self.values[from.min(end)..end]
            .iter()
            .fold(init, |acc, value| {
                self.reads.fetch_add(1, Ordering::Relaxed);
                f(acc, value.clone())
            })
    }
    fn try_fold_range_at<B, E, F: FnMut(B, T) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> Result<B, E> {
        let end = to.min(self.len());
        self.values[from.min(end)..end]
            .iter()
            .try_fold(init, |acc, value| {
                self.reads.fetch_add(1, Ordering::Relaxed);
                f(acc, value.clone())
            })
    }
}

fn lifecycle<T: VecValue, O: BytesVecValue + PartialEq>(
    values: Vec<T>,
    expected: &[O],
    mut compute: impl FnMut(
        &mut EagerVec<BytesVec<usize, O>>,
        usize,
        &CountingSource<T>,
        &Exit,
    ) -> VecdbResult<()>,
) -> VecdbResult<()> {
    let directory = tempdir()?;
    let db = Database::open(directory.path())?;
    let exit = Exit::new();
    let mut source = CountingSource::new(values);
    let mut output = EagerVec::import(&db, "output", Version::ONE)?;
    for from in [0, 3, expected.len(), expected.len() + 4, 1] {
        source.reads.store(0, Ordering::Relaxed);
        compute(&mut output, from, &source, &exit)?;
        assert_eq!(output.collect(), expected);
        if from >= expected.len() {
            assert_eq!(source.reads.load(Ordering::Relaxed), 0);
        }
    }
    drop(output);
    let mut output = EagerVec::import(&db, "output", Version::ONE)?;
    compute(&mut output, 4, &source, &exit)?;
    assert_eq!(output.collect(), expected);
    source.version = Version::TWO;
    source.reads.store(0, Ordering::Relaxed);
    compute(&mut output, expected.len(), &source, &exit)?;
    assert_eq!(output.collect(), expected);
    assert!(source.reads.load(Ordering::Relaxed) > 0);
    Ok(())
}

#[test]
fn integer_compute_paths_preserve_resume_and_version_reset() -> VecdbResult<()> {
    let values = vec![2_u64, 8, 1, 9, 4, 3, 7, 6];
    lifecycle(
        values.clone(),
        &[2_u64, 10, 11, 20, 24, 27, 34, 40],
        |out, from, source, exit| out.compute_cumulative(from, source, exit),
    )?;
    lifecycle(
        values.clone(),
        &[4_u64, 20, 22, 40, 48, 54, 68, 80],
        |out, from, source, exit| {
            out.compute_cumulative_transformed_binary(from, source, source, |a, b| a + b, exit)
        },
    )?;
    for sources in 1..=4 {
        let expected: Vec<_> = values.iter().map(|value| value * sources as u64).collect();
        lifecycle(
            values.clone(),
            &expected,
            |out, from, source, exit| match sources {
                1 => out.compute_transform(from, source, |(i, value, _)| (i, value), exit),
                2 => out.compute_transform2_batched(
                    from,
                    source,
                    source,
                    3,
                    |(i, a, b, _)| (i, a + b),
                    exit,
                ),
                3 => out.compute_transform3(
                    from,
                    source,
                    source,
                    source,
                    |(i, a, b, c, _)| (i, a + b + c),
                    exit,
                ),
                4 => out.compute_transform4(
                    from,
                    source,
                    source,
                    source,
                    source,
                    |(i, a, b, c, d, _)| (i, a + b + c + d),
                    exit,
                ),
                _ => unreachable!(),
            },
        )?;
    }
    Ok(())
}

#[test]
fn sma_preserves_resume_and_version_reset() -> VecdbResult<()> {
    let values = vec![2_f32, 8., 1., 9., 4., 3., 7., 6.];
    let mut sma = Vec::new();
    let mut previous = 0.0;
    for (i, &value) in values.iter().enumerate() {
        previous = if i >= 3 {
            previous + (value - values[i - 3]) / 3.0
        } else {
            (previous * i as f32 + value) / (i + 1) as f32
        };
        sma.push(previous);
    }
    lifecycle(values.clone(), &sma, |out, from, source, exit| {
        out.compute_sma(from, source, 3, exit, None)
    })?;
    Ok(())
}

#[test]
fn transforms_clamp_uneven_and_empty_sources() -> VecdbResult<()> {
    let directory = tempdir()?;
    let db = Database::open(directory.path())?;
    let exit = Exit::new();
    let first = CountingSource::new(vec![1_u64; 8]);
    let mut second = CountingSource::new(vec![2_u64; 5]);
    let third = CountingSource::new(vec![3_u64; 7]);
    let fourth = CountingSource::new(vec![4_u64; 6]);
    for arity in 2..=4 {
        let mut out: EagerVec<BytesVec<usize, u64>> =
            EagerVec::import(&db, &format!("transform_{arity}"), Version::ONE)?;
        let mut visit = Vec::new();
        match arity {
            2 => out.compute_transform2_batched(
                0,
                &first,
                &second,
                3,
                |(i, a, b, out)| {
                    assert_eq!(i, out.len());
                    visit.push(i);
                    (i, a + b)
                },
                &exit,
            )?,
            3 => out.compute_transform3(
                0,
                &first,
                &second,
                &third,
                |(i, a, b, c, out)| {
                    assert_eq!(i, out.len());
                    visit.push(i);
                    (i, a + b + c)
                },
                &exit,
            )?,
            4 => out.compute_transform4(
                0,
                &first,
                &second,
                &third,
                &fourth,
                |(i, a, b, c, d, out)| {
                    assert_eq!(i, out.len());
                    visit.push(i);
                    (i, a + b + c + d)
                },
                &exit,
            )?,
            _ => unreachable!(),
        }
        assert_eq!(visit, [0, 1, 2, 3, 4]);
        assert_eq!(out.collect(), vec![arity * (arity + 1) / 2; 5]);
    }
    second.values.clear();
    let mut out: EagerVec<BytesVec<usize, u64>> = EagerVec::import(&db, "empty", Version::ONE)?;
    out.compute_transform4(
        0,
        &first,
        &second,
        &third,
        &fourth,
        |_| panic!("empty source callback"),
        &exit,
    )?;
    assert!(out.is_empty());
    Ok(())
}
