use bitview_cohort::UTXOAggregateId;
use brk_error::Result;
use brk_types::UrpdRaw;

use super::AgeRangeUrpds;

impl AgeRangeUrpds {
    pub fn aggregate(&self, id: UTXOAggregateId) -> Result<UrpdRaw> {
        let entries = id
            .age_range_ids()
            .iter()
            .map(|&id| self.get(id))
            .filter(|entries| !entries.is_empty())
            .try_fold(Vec::new(), |left, right| Self::merge_sorted(&left, right))?;
        let raw = UrpdRaw {
            map: entries.into_iter().collect(),
        };
        raw.checked_supply()?;
        Ok(raw)
    }
}

#[cfg(test)]
#[path = "../../../../tests/unit/state/utxo/age_range_urpds/aggregate.rs"]
mod tests;
