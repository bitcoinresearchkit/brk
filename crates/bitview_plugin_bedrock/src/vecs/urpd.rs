use std::path::PathBuf;

use bitview_cohort::AgeRangeId;
use bitview_plugin_coinflow::Vecs as CoinflowVecs;
use bitview_plugin_cointime::Vecs as CointimeVecs;
use bitview_plugin_distribution::Vecs as DistributionVecs;
use brk_error::Result;
use brk_types::{Cohort, Date, Day1, UrpdRaw, UrpdWeight};
use vecdb::{ReadableVec, StorageMode};

use super::Vecs;
use crate::DayUrpds;

impl<M: StorageMode> Vecs<M> {
    /// Directory containing a persisted aggregate Bedrock-weighted URPD.
    pub fn urpd_dir(&self, weight: UrpdWeight, cohort: &Cohort) -> PathBuf {
        UrpdRaw::dir(&self.states_path, &DayUrpds::weighted_name(weight, cohort))
    }

    /// Read a persisted aggregate Bedrock-weighted URPD.
    pub fn urpd_raw(&self, weight: UrpdWeight, cohort: &Cohort, date: Date) -> Result<UrpdRaw> {
        let bytes = self.urpd_raw_bytes(weight, cohort, date)?;
        UrpdRaw::deserialize_exact(&bytes)
    }

    /// Capture a persisted weighted snapshot under the producer's publication guard.
    pub fn urpd_raw_bytes(
        &self,
        weight: UrpdWeight,
        cohort: &Cohort,
        date: Date,
    ) -> Result<Vec<u8>> {
        UrpdRaw::read_bytes(
            &self.states_path,
            &DayUrpds::weighted_name(weight, cohort),
            date,
        )
    }

    /// Resolve one age-range cohort's scalar Bedrock weight for a day.
    pub fn urpd_weight<N: StorageMode>(
        &self,
        distribution: &DistributionVecs<N>,
        cointime: &CointimeVecs<N>,
        coinflow: &CoinflowVecs<N>,
        cohort: &Cohort,
        day: Day1,
        weight: UrpdWeight,
    ) -> Option<f64> {
        let cohort_id = cohort.strip_prefix("utxos_")?;
        let age = AgeRangeId::ALL
            .iter()
            .copied()
            .find(|age| age.name().id == cohort_id)?;

        if weight == UrpdWeight::Raw {
            return Some(1.0);
        }

        let supplies = &distribution.cohorts.supply.total.cohorts.utxo.age.range;
        let supply = age.select(supplies).sats.day1.collect_one(day).flatten()?;

        match weight {
            UrpdWeight::Raw => Some(1.0),
            UrpdWeight::Cointime => {
                let cohort = age.select(&cointime.age_range.activity.wakefulness);
                Self::resolve_age_value(cohort.day1.collect_one(day).flatten(), supply)
                    .map(|value| value.clamp(0.0, 1.0))
            }
            UrpdWeight::Coinflow => {
                let cohort = age.select(&coinflow.age_range.spending_exposure.mobility);
                Self::resolve_age_value(cohort.day1.0.collect_one(day).flatten(), supply)
                    .map(|value| value.clamp(0.0, 1.0))
            }
        }
    }
}
