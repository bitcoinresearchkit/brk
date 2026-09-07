use std::{collections::BTreeMap, fs, path::Path};

use bitview_cohort::{AgeRange, AgeRangeId};
use brk_error::Result;
use brk_types::{CentsCompact, Date, Sats, UrpdRaw};

use super::super::{COST_BASIS_PRICE_DIGITS, UTXOCohortState, UTXOStates};
use super::{AgeRangeUrpds, HEADER_LEN};
use crate::state::{CostBasisData, RealizedState, WithCapital};

impl AgeRangeUrpds {
    fn from_states(
        states: &AgeRange<UTXOCohortState<RealizedState, CostBasisData<WithCapital>>>,
    ) -> Self {
        Self {
            entries: AgeRange::par_from_fn(|id| {
                Self::rounded_entries(id.select(states).cost_basis_map())
            }),
        }
    }

    fn write(&self, states_path: &Path, date: Date) -> Result<()> {
        let sections =
            AgeRange::par_try_from_fn(|id| UrpdRaw::serialize_iter(self.get(id).iter().copied()))?;
        let capacity = HEADER_LEN + sections.iter().map(Vec::len).sum::<usize>();
        if capacity > UrpdRaw::MAX_ENCODED_BYTES {
            return Err(Self::invalid("file exceeds snapshot limit"));
        }
        let mut buffer = Self::new_buffer(capacity);
        for (index, id) in AgeRangeId::ALL.iter().copied().enumerate() {
            buffer.extend_from_slice(id.select(&sections));
            Self::set_offset(&mut buffer, index + 1);
        }

        fs::create_dir_all(Self::dir(states_path))?;
        fs::write(Self::path(states_path, date), buffer)?;
        Ok(())
    }

    fn rounded_entries(map: &BTreeMap<CentsCompact, Sats>) -> Vec<(CentsCompact, Sats)> {
        let mut entries = Vec::<(CentsCompact, Sats)>::new();
        for (&price, &sats) in map {
            let price = price.round_to_dollar(COST_BASIS_PRICE_DIGITS);
            if let Some(last) = entries.last_mut()
                && last.0 == price
            {
                last.1 += sats;
            } else {
                entries.push((price, sats));
            }
        }
        entries
    }
}

impl UTXOStates {
    pub fn write_urpds(&self, date: Date, states_path: &Path) -> Result<()> {
        AgeRangeUrpds::from_states(&self.age_range).write(states_path, date)
    }

    pub fn age_range_urpd_entries(
        &self,
        id: AgeRangeId,
    ) -> impl Iterator<Item = (CentsCompact, Sats)> + '_ {
        id.select(&self.age_range)
            .cost_basis_map()
            .iter()
            .map(|(&price, &sats)| (price.round_to_dollar(COST_BASIS_PRICE_DIGITS), sats))
    }
}

#[cfg(test)]
#[path = "../../../../tests/unit/state/utxo/age_range_urpds/write.rs"]
mod tests;
