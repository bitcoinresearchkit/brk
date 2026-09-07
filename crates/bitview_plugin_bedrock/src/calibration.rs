use std::cmp::Ordering;

use brk_types::Day1;
use vecdb::{ReadableVec, VecValue};

use super::{ModeId, Modes, Percentiles, Thresholds, WeightedModes};

const MINIMUM_DAYS: usize = 365;
const PERCENTILES: Percentiles<f64> = Percentiles {
    pct95: 0.95,
    pct98: 0.98,
    pct99: 0.99,
    pct99_5: 0.995,
    pct99_9: 0.999,
};

pub struct Calibration {
    histories: Modes<Vec<f64>>,
}

impl Calibration {
    pub fn from_sources<T, U>(
        raw: &impl ReadableVec<Day1, Option<T>>,
        weighted: &WeightedModes<&dyn ReadableVec<Day1, Option<U>>>,
        end: usize,
    ) -> Self
    where
        T: VecValue,
        U: VecValue,
        f64: From<T> + From<U>,
    {
        let histories = Modes::from_fn(|mode| match mode.weighted() {
            None => Self::history(raw, end),
            Some(id) => Self::history(*weighted.select(id), end),
        });
        Self { histories }
    }

    pub fn loss_shares<T, U>(
        raw: &impl ReadableVec<Day1, Option<T>>,
        weighted: &WeightedModes<&dyn ReadableVec<Day1, Option<U>>>,
        day: Day1,
    ) -> Modes<Option<f64>>
    where
        T: VecValue,
        U: VecValue,
        f64: From<T> + From<U>,
    {
        Modes::from_fn(|mode| match mode.weighted() {
            None => Self::loss_share(raw, day),
            Some(id) => Self::loss_share(*weighted.select(id), day),
        })
    }

    pub fn thresholds(&self, current: &Modes<Option<f64>>) -> Thresholds {
        Thresholds::from_fn(|mode| {
            let history = self.histories.select(mode);
            (current.select(mode).is_some() && history.len() >= MINIMUM_DAYS).then(|| {
                Percentiles::from_fn(|percentile| {
                    Self::quantile(history, *percentile.select(&PERCENTILES))
                        .expect("non-empty history")
                })
            })
        })
    }

    pub fn observe(&mut self, shares: Modes<Option<f64>>) {
        for mode in ModeId::ALL {
            let history = self.histories.select_mut(mode);
            if let Some(share) = *shares.select(mode) {
                Self::insert_sorted(history, share.clamp(0.0, 1.0));
            }
        }
    }

    fn history<T>(source: &(impl ReadableVec<Day1, Option<T>> + ?Sized), end: usize) -> Vec<f64>
    where
        T: VecValue,
        f64: From<T>,
    {
        let mut history = Vec::with_capacity(end);
        source.for_each_range_dyn_at(0, end, &mut |value| {
            if let Some(value) = value.map(f64::from).filter(|value| value.is_finite()) {
                history.push(value);
            }
        });
        history.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        history
    }

    fn loss_share<T>(
        source: &(impl ReadableVec<Day1, Option<T>> + ?Sized),
        day: Day1,
    ) -> Option<f64>
    where
        T: VecValue,
        f64: From<T>,
    {
        source
            .collect_one(day)
            .flatten()
            .map(f64::from)
            .filter(|value| value.is_finite())
    }

    fn quantile(sorted: &[f64], percentile: f64) -> Option<f64> {
        if sorted.is_empty() {
            return None;
        }
        let position = percentile.clamp(0.0, 1.0) * (sorted.len() - 1) as f64;
        let lower = position.floor() as usize;
        let upper = position.ceil() as usize;
        let fraction = position - lower as f64;
        Some(sorted[lower] * (1.0 - fraction) + sorted[upper] * fraction)
    }

    fn insert_sorted(values: &mut Vec<f64>, value: f64) {
        let index = values
            .binary_search_by(|candidate| candidate.partial_cmp(&value).unwrap_or(Ordering::Less))
            .unwrap_or_else(|index| index);
        values.insert(index, value);
    }
}

#[cfg(test)]
mod tests {
    use super::{Calibration, MINIMUM_DAYS};
    use crate::Modes;

    #[test]
    fn source_construction_selects_each_mode_and_filters_missing_values() {
        use brk_types::{Day1, StoredF64, Version};
        use vecdb::{
            AnyStoredVec, Database, EagerVec, ImportableVec, LazyVec, PcoVec, ReadableCloneableVec,
            ReadableVec, WritableVec,
        };

        use crate::{ModeId, WeightedModes};

        let directory = tempfile::tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let source = |mode: ModeId| {
            let mut values: EagerVec<PcoVec<Day1, StoredF64>> =
                EagerVec::forced_import(&db, mode.name(), Version::ONE).unwrap();
            let value = mode as u8 as f64 / 16.0;
            for value in [value + 0.125, f64::NAN, value] {
                values.push(StoredF64::from(value));
            }
            values.write().unwrap();
            LazyVec::init(
                mode.name(),
                Version::ONE,
                values.read_only_boxed_clone(),
                |_, v| Some(v),
            )
        };
        let raw = source(ModeId::Raw);
        let weighted = WeightedModes::from_fn(|id| source(id.mode()));
        let sources = WeightedModes::from_fn(|id| {
            weighted.select(id) as &dyn ReadableVec<Day1, Option<StoredF64>>
        });
        let calibration = Calibration::from_sources(&raw, &sources, 3);
        let shares = Calibration::loss_shares(&raw, &sources, Day1::from(2));
        for mode in ModeId::ALL {
            let value = mode as u8 as f64 / 16.0;
            assert_eq!(calibration.histories.select(mode), &[value, value + 0.125]);
            assert_eq!(*shares.select(mode), Some(value));
        }
        assert!(
            Calibration::loss_shares(&raw, &sources, Day1::from(1))
                .iter()
                .all(Option::is_none)
        );
        assert!(
            Calibration::loss_shares(&raw, &sources, Day1::from(3))
                .iter()
                .all(Option::is_none)
        );
    }

    #[test]
    fn quantile_linearly_interpolates() {
        assert_eq!(Calibration::quantile(&[0.0, 1.0], 0.95), Some(0.95));
        assert_eq!(Calibration::quantile(&[], 0.95), None);
    }

    #[test]
    fn missing_share_does_not_update_history() {
        let mut calibration = Calibration {
            histories: Modes::from_fn(|_| Vec::new()),
        };
        let shares = Modes::from_fn(|_| None);
        assert!(calibration.thresholds(&shares).iter().all(Option::is_none));
        calibration.observe(shares);
        assert!(calibration.histories.iter().all(Vec::is_empty));
    }

    #[test]
    fn a_year_of_history_enables_thresholds() {
        let calibration = Calibration {
            histories: Modes::from_fn(|_| vec![0.5; MINIMUM_DAYS]),
        };
        let shares = Modes::from_fn(|_| Some(0.5));
        let thresholds = calibration.thresholds(&shares);
        assert!(thresholds.iter().all(|values| values.is_some()));
        assert!(
            thresholds
                .iter()
                .all(|values| values.as_ref().unwrap().pct95 == 0.5)
        );
    }
}
