use brk_types::Cents;

use crate::internal::{PERCENTILES, PERCENTILES_LEN};

use super::CostBasisMap;

#[derive(Clone, Copy, Debug)]
pub struct Percentiles {
    /// Sat-weighted: percentiles by coin count
    pub sat_weighted: [Cents; PERCENTILES_LEN],
    /// USD-weighted: percentiles by invested capital (sats × price)
    pub usd_weighted: [Cents; PERCENTILES_LEN],
}

impl Percentiles {
    /// Compute both sat-weighted and USD-weighted percentiles in two passes over the BTreeMap.
    /// Avoids intermediate Vec allocation by iterating the map directly.
    pub(crate) fn compute_from_map(map: &CostBasisMap) -> Option<Self> {
        if map.is_empty() {
            return None;
        }

        // First pass: compute totals
        let mut total_sats: u64 = 0;
        let mut total_usd: u128 = 0;
        for (&cents, &sats) in map.iter() {
            total_sats += u64::from(sats);
            total_usd += cents.as_u128() * sats.as_u128();
        }

        if total_sats == 0 {
            return None;
        }

        // Precompute targets to avoid repeated multiplication in the inner loop
        let sat_targets: [u64; PERCENTILES_LEN] =
            PERCENTILES.map(|p| total_sats * u64::from(p) / 100);
        let usd_targets: [u128; PERCENTILES_LEN] =
            PERCENTILES.map(|p| total_usd * u128::from(p) / 100);

        let mut sat_weighted = [Cents::ZERO; PERCENTILES_LEN];
        let mut usd_weighted = [Cents::ZERO; PERCENTILES_LEN];
        let mut cumsum_sats: u64 = 0;
        let mut cumsum_usd: u128 = 0;
        let mut sat_idx = 0;
        let mut usd_idx = 0;

        // Second pass: compute percentiles
        for (&cents, &sats) in map.iter() {
            cumsum_sats += u64::from(sats);
            cumsum_usd += cents.as_u128() * sats.as_u128();

            while sat_idx < PERCENTILES_LEN && cumsum_sats >= sat_targets[sat_idx] {
                sat_weighted[sat_idx] = cents.into();
                sat_idx += 1;
            }

            while usd_idx < PERCENTILES_LEN && cumsum_usd >= usd_targets[usd_idx] {
                usd_weighted[usd_idx] = cents.into();
                usd_idx += 1;
            }
        }

        Some(Self {
            sat_weighted,
            usd_weighted,
        })
    }
}
