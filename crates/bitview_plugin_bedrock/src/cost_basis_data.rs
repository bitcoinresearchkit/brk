use brk_types::{
    Cents, CentsCompact, CostBasisByPercentile, PERCENTILES_LEN, PartsPerMillion32, Sats, UrpdRaw,
};

use crate::SupplyDensity;

#[derive(Debug, PartialEq)]
pub struct CostBasisData {
    pub prices: CostBasisByPercentile,
    pub supply_density: SupplyDensity<PartsPerMillion32>,
    pub supply_density_10pct: SupplyDensity<PartsPerMillion32>,
}

impl CostBasisData {
    pub fn from_entries(
        entries: impl Iterator<Item = (CentsCompact, Sats)> + Clone,
        spot: Cents,
    ) -> Self {
        Self {
            prices: UrpdRaw::cost_basis_percentile_prices_from_entries(entries.clone()),
            supply_density: SupplyDensity::from_entries::<5>(entries.clone(), spot),
            supply_density_10pct: SupplyDensity::from_entries::<10>(entries, spot),
        }
    }
}

impl Default for CostBasisData {
    fn default() -> Self {
        Self {
            prices: CostBasisByPercentile {
                per_coin: [Cents::NAN; PERCENTILES_LEN],
                per_dollar: [Cents::NAN; PERCENTILES_LEN],
            },
            supply_density: SupplyDensity::NAN,
            supply_density_10pct: SupplyDensity::NAN,
        }
    }
}
