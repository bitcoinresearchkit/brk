use brk_types::{CentsUnsigned, CentsUnsignedCompact, Sats};

use crate::internal::{PERCENTILES, PERCENTILES_LEN};

#[derive(Clone, Copy, Debug)]
pub struct Percentiles {
    /// Sat-weighted: percentiles by coin count
    pub sat_weighted: [CentsUnsigned; PERCENTILES_LEN],
    /// USD-weighted: percentiles by invested capital (sats × price)
    pub usd_weighted: [CentsUnsigned; PERCENTILES_LEN],
}

impl Percentiles {
    /// Compute both sat-weighted and USD-weighted percentiles in a single pass.
    /// Takes an iterator over (price, sats) pairs, assumed sorted by price ascending.
    pub fn compute(iter: impl Iterator<Item = (CentsUnsignedCompact, Sats)>) -> Option<Self> {
        // Collect to allow two passes: one for totals, one for percentiles
        let entries: Vec<_> = iter.collect();
        if entries.is_empty() {
            return None;
        }

        // Compute totals
        let mut total_sats: u64 = 0;
        let mut total_usd: u128 = 0;
        for &(cents, sats) in &entries {
            total_sats += u64::from(sats);
            total_usd += cents.as_u128() * sats.as_u128();
        }

        if total_sats == 0 {
            return None;
        }

        let mut sat_weighted = [CentsUnsigned::ZERO; PERCENTILES_LEN];
        let mut usd_weighted = [CentsUnsigned::ZERO; PERCENTILES_LEN];
        let mut cumsum_sats: u64 = 0;
        let mut cumsum_usd: u128 = 0;
        let mut sat_idx = 0;
        let mut usd_idx = 0;

        for (cents, sats) in entries {
            cumsum_sats += u64::from(sats);
            cumsum_usd += cents.as_u128() * sats.as_u128();

            while sat_idx < PERCENTILES_LEN
                && cumsum_sats >= total_sats * u64::from(PERCENTILES[sat_idx]) / 100
            {
                sat_weighted[sat_idx] = cents.into();
                sat_idx += 1;
            }

            while usd_idx < PERCENTILES_LEN
                && cumsum_usd >= total_usd * u128::from(PERCENTILES[usd_idx]) / 100
            {
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
