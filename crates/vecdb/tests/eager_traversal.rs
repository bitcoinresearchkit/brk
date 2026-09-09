use std::{
    any,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use brk_exit::Exit;
use tempfile::tempdir;
use vecdb::{
    AnyVec, BytesVec, BytesVecValue, Database, EagerVec, Error, ImportableVec, ReadableVec,
    Result as VecdbResult, VecValue, Version,
};

struct CountingSource<T> {
    values: Vec<T>,
    version: Version,
    reads: AtomicUsize,
    fallible_folds: AtomicUsize,
}

impl<T> CountingSource<T> {
    fn new(values: Vec<T>) -> Self {
        Self {
            values,
            version: Version::ONE,
            reads: AtomicUsize::new(0),
            fallible_folds: AtomicUsize::new(0),
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
        self.fallible_folds.fetch_add(1, Ordering::Relaxed);
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
        |out, from, source, exit| out.compute_cumulative_binary(from, source, source, exit),
    )?;
    lifecycle(
        values.clone(),
        &[0_usize, 1, 1, 2, 1, 1, 1, 2],
        |out, from, source, exit| out.compute_rolling_count(from, source, 3, |v| *v > 4, exit),
    )?;
    lifecycle(
        values.clone(),
        &[2_u64, 8, 8, 9, 9, 9, 7, 7],
        |out, from, source, exit| out.compute_max(from, source, 3, exit),
    )?;
    lifecycle(
        values.clone(),
        &[2_u64, 2, 1, 1, 1, 3, 3, 3],
        |out, from, source, exit| out.compute_min(from, source, 3, exit),
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
fn floating_compute_paths_preserve_resume_and_version_reset() -> VecdbResult<()> {
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
        out.compute_sma(from, source, 3, exit)
    })?;
    lifecycle(
        values.clone(),
        &[2., 5., 2., 8., 4., 4., 4., 6.],
        |out, from, source, exit| out.compute_rolling_median(from, source, 3, exit),
    )?;
    for (rma, k) in [(false, 0.5_f32), (true, 1.0_f32 / 3.0)] {
        let mut expected = Vec::new();
        let mut previous = 0.0;
        for (i, &value) in values.iter().enumerate() {
            previous = if i >= 3 {
                value * k + previous * (1.0 - k)
            } else {
                (previous * i as f32 + value) / (i + 1) as f32
            };
            expected.push(previous);
        }
        lifecycle(values.clone(), &expected, |out, from, source, exit| {
            if rma {
                out.compute_rma(from, source, 3, exit)
            } else {
                out.compute_ema(from, source, 3, exit)
            }
        })?;
    }
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

#[derive(Debug, Default)]
struct TrackedValue {
    value: i64,
    clones: Arc<AtomicUsize>,
}

impl Clone for TrackedValue {
    fn clone(&self) -> Self {
        self.clones.fetch_add(1, Ordering::Relaxed);
        Self {
            value: self.value,
            clones: Arc::clone(&self.clones),
        }
    }
}

impl From<TrackedValue> for i64 {
    fn from(value: TrackedValue) -> Self {
        value.value
    }
}

#[test]
fn lookback_consumes_owned_previous_values_without_extra_clones() -> VecdbResult<()> {
    let directory = tempdir()?;
    let db = Database::open(directory.path())?;
    let exit = Exit::new();
    for window in [0, 1, 5, 32] {
        let clones = Arc::new(AtomicUsize::new(0));
        let mut source = CountingSource::new(
            (0..16)
                .map(|value| TrackedValue {
                    value,
                    clones: Arc::clone(&clones),
                })
                .collect(),
        );
        let name = format!("change_{window}");
        let expected: Vec<i64> = (0..16)
            .map(|i| if i < window { 0 } else { window as i64 })
            .collect();
        let mut out: EagerVec<BytesVec<usize, i64>> = EagerVec::import(&db, &name, Version::ONE)?;
        for from in [0, 6, 16] {
            clones.store(0, Ordering::Relaxed);
            source.reads.store(0, Ordering::Relaxed);
            out.compute_change(from, &source, window, &exit)?;
            assert_eq!(out.collect(), expected);
            assert_eq!(
                clones.load(Ordering::Relaxed),
                source.reads.load(Ordering::Relaxed)
            );
        }
        drop(out);
        let mut out: EagerVec<BytesVec<usize, i64>> = EagerVec::import(&db, &name, Version::ONE)?;
        source.version = Version::TWO;
        clones.store(0, Ordering::Relaxed);
        source.reads.store(0, Ordering::Relaxed);
        out.compute_change(16, &source, window, &exit)?;
        assert_eq!(out.collect(), expected);
        assert!(source.reads.load(Ordering::Relaxed) > 0);
        assert_eq!(
            clones.load(Ordering::Relaxed),
            source.reads.load(Ordering::Relaxed)
        );
    }
    Ok(())
}

#[test]
fn checked_sum_keeps_its_fallible_early_exit() -> VecdbResult<()> {
    let directory = tempdir()?;
    let db = Database::open(directory.path())?;
    let source = CountingSource::new(vec![1_u64, 2, 3]);
    let mut out: EagerVec<BytesVec<usize, u64>> = EagerVec::import(&db, "sum", Version::ONE)?;
    assert!(matches!(
        out.compute_sum(0, &source, 0, &Exit::new()),
        Err(Error::Underflow)
    ));
    assert_eq!(source.fallible_folds.load(Ordering::Relaxed), 1);
    assert!(out.is_empty());
    Ok(())
}
