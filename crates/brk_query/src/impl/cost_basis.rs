use std::{fs, path::PathBuf};

use brk_error::{Error, Result};
use brk_types::{
    CostBasisBucket, CostBasisDistribution, CostBasisFormatted, CostBasisValue, Date, DateIndex,
};
use vecdb::IterableVec;

use crate::Query;

impl Query {
    /// List available cohorts for cost basis distribution.
    pub fn cost_basis_cohorts(&self) -> Result<Vec<String>> {
        let states_path = &self.computer().distribution.states_path;

        let mut cohorts: Vec<String> = fs::read_dir(states_path)?
            .filter_map(|entry| {
                let name = entry.ok()?.file_name().into_string().ok()?;
                let cohort = name.strip_prefix("utxo_")?.strip_suffix("_cost_basis")?;
                states_path
                    .join(&name)
                    .join("by_date")
                    .exists()
                    .then(|| cohort.to_string())
            })
            .collect();

        cohorts.sort();
        Ok(cohorts)
    }

    fn cost_basis_dir(&self, cohort: &str) -> Result<PathBuf> {
        let dir = self
            .computer()
            .distribution
            .states_path
            .join(format!("utxo_{cohort}_cost_basis/by_date"));

        if !dir.exists() {
            return Err(Error::NotFound(format!("Unknown cohort '{cohort}'")));
        }

        Ok(dir)
    }

    /// Get the cost basis distribution for a cohort on a specific date.
    pub fn cost_basis_distribution(
        &self,
        cohort: &str,
        date: Date,
    ) -> Result<CostBasisDistribution> {
        let path = self.cost_basis_dir(cohort)?.join(date.to_string());

        if !path.exists() {
            return Err(Error::NotFound(format!(
                "No data for cohort '{cohort}' on {date}"
            )));
        }

        CostBasisDistribution::deserialize(&fs::read(&path)?)
    }

    /// List available dates for a cohort's cost basis distribution.
    pub fn cost_basis_dates(&self, cohort: &str) -> Result<Vec<Date>> {
        let dir = self.cost_basis_dir(cohort)?;

        let mut dates: Vec<Date> = fs::read_dir(&dir)?
            .filter_map(|entry| entry.ok()?.file_name().to_str()?.parse().ok())
            .collect();

        dates.sort();
        Ok(dates)
    }

    /// Get the formatted cost basis distribution.
    pub fn cost_basis_formatted(
        &self,
        cohort: &str,
        date: Date,
        bucket: CostBasisBucket,
        value: CostBasisValue,
    ) -> Result<CostBasisFormatted> {
        let distribution = self.cost_basis_distribution(cohort, date)?;
        let dateindex =
            DateIndex::try_from(date).map_err(|e| Error::Parse(e.to_string()))?;
        let price = self
            .computer()
            .price
            .as_ref()
            .ok_or_else(|| Error::NotFound("Price data not available".to_string()))?;
        let spot = *price
            .cents
            .split
            .dateindex
            .close
            .iter()
            .get(dateindex)
            .ok_or_else(|| Error::NotFound(format!("No price data for {date}")))?;
        Ok(distribution.format(bucket, value, spot))
    }
}
