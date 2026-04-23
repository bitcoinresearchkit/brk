use std::{fs, path::PathBuf};

use brk_error::{Error, Result};
use brk_types::{Cohort, Date, Day1, Urpd, UrpdAggregation, UrpdRaw};
use vecdb::ReadableOptionVec;

use crate::Query;

impl Query {
    /// Available cohorts for URPD.
    pub fn urpd_cohorts(&self) -> Result<Vec<Cohort>> {
        let states_path = &self.computer().distribution.states_path;

        let mut cohorts: Vec<Cohort> = fs::read_dir(states_path)?
            .filter_map(|entry| {
                let name = entry.ok()?.file_name().into_string().ok()?;
                states_path
                    .join(&name)
                    .join("urpd")
                    .exists()
                    .then(|| Cohort::from(name))
            })
            .collect();

        cohorts.sort_by_key(|a| a.to_string());

        Ok(cohorts)
    }

    pub(crate) fn urpd_dir(&self, cohort: &str) -> Result<PathBuf> {
        let dir = self
            .computer()
            .distribution
            .states_path
            .join(cohort)
            .join("urpd");

        if !dir.exists() {
            let valid = self
                .urpd_cohorts()
                .unwrap_or_default()
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(Error::NotFound(format!(
                "Unknown cohort '{cohort}'. Available: {valid}"
            )));
        }

        Ok(dir)
    }

    /// Available dates for a cohort.
    pub fn urpd_dates(&self, cohort: &Cohort) -> Result<Vec<Date>> {
        let dir = self.urpd_dir(cohort)?;

        let mut dates: Vec<Date> = fs::read_dir(&dir)?
            .filter_map(|entry| entry.ok()?.file_name().to_str()?.parse().ok())
            .collect();

        dates.sort();
        Ok(dates)
    }

    /// Raw URPD data for a cohort on a specific date.
    pub fn urpd_raw(&self, cohort: &Cohort, date: Date) -> Result<UrpdRaw> {
        let path = self.urpd_dir(cohort)?.join(date.to_string());

        if !path.exists() {
            return Err(Error::NotFound(format!(
                "No URPD for cohort '{cohort}' on {date}"
            )));
        }

        UrpdRaw::deserialize(&fs::read(&path)?)
    }

    /// URPD for a cohort on a specific date.
    pub fn urpd_at(&self, cohort: &Cohort, date: Date, agg: UrpdAggregation) -> Result<Urpd> {
        let raw = self.urpd_raw(cohort, date)?;
        let day1 = Day1::try_from(date).map_err(|e| Error::Parse(e.to_string()))?;
        let close = self
            .computer()
            .prices
            .split
            .close
            .cents
            .day1
            .collect_one_flat(day1)
            .ok_or_else(|| Error::NotFound(format!("No price data for {date}")))?;
        Ok(Urpd::build(cohort.clone(), date, close, &raw, agg))
    }

    /// URPD for the most recently available date in a cohort.
    pub fn urpd_latest(&self, cohort: &Cohort, agg: UrpdAggregation) -> Result<Urpd> {
        let dates = self.urpd_dates(cohort)?;
        let date = *dates
            .last()
            .ok_or_else(|| Error::NotFound(format!("No URPD available for cohort '{cohort}'")))?;
        self.urpd_at(cohort, date, agg)
    }
}
