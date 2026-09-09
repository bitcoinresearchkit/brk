#[cfg(feature = "storage")]
use bitview_traversable::Traversable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::CohortName;

/// Names for total positive profit and 13 strict profit thresholds.
pub const PROFIT_NAMES: Profit<CohortName> = Profit {
    total: CohortName::new("utxos_in_profit", "Total", "In Profit"),
    _10pct: CohortName::new("utxos_over_10pct_in_profit", ">10%", "Over 10% in Profit"),
    _20pct: CohortName::new("utxos_over_20pct_in_profit", ">20%", "Over 20% in Profit"),
    _30pct: CohortName::new("utxos_over_30pct_in_profit", ">30%", "Over 30% in Profit"),
    _40pct: CohortName::new("utxos_over_40pct_in_profit", ">40%", "Over 40% in Profit"),
    _50pct: CohortName::new("utxos_over_50pct_in_profit", ">50%", "Over 50% in Profit"),
    _60pct: CohortName::new("utxos_over_60pct_in_profit", ">60%", "Over 60% in Profit"),
    _70pct: CohortName::new("utxos_over_70pct_in_profit", ">70%", "Over 70% in Profit"),
    _80pct: CohortName::new("utxos_over_80pct_in_profit", ">80%", "Over 80% in Profit"),
    _90pct: CohortName::new("utxos_over_90pct_in_profit", ">90%", "Over 90% in Profit"),
    _100pct: CohortName::new(
        "utxos_over_100pct_in_profit",
        ">100%",
        "Over 100% in Profit",
    ),
    _200pct: CohortName::new(
        "utxos_over_200pct_in_profit",
        ">200%",
        "Over 200% in Profit",
    ),
    _300pct: CohortName::new(
        "utxos_over_300pct_in_profit",
        ">300%",
        "Over 300% in Profit",
    ),
    _500pct: CohortName::new(
        "utxos_over_500pct_in_profit",
        ">500%",
        "Over 500% in Profit",
    ),
};

/// Number of profit thresholds.
pub const PROFIT_COUNT: usize = 14;

impl Profit<CohortName> {
    pub const fn names() -> &'static Self {
        &PROFIT_NAMES
    }
}

/// Total positive profit and 13 "more than X% profit" aggregate thresholds.
///
/// Each is a prefix sum over the profitability ranges, from most profitable down.
#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct Profit<T> {
    /// Uses UTXOs whose creation price is below the represented block's spot
    /// price.
    pub total: T,
    /// Uses UTXOs whose represented-block spot price is more than 10% above
    /// creation price.
    pub _10pct: T,
    /// Uses UTXOs whose represented-block spot price is more than 20% above
    /// creation price.
    pub _20pct: T,
    /// Uses UTXOs whose represented-block spot price is more than 30% above
    /// creation price.
    pub _30pct: T,
    /// Uses UTXOs whose represented-block spot price is more than 40% above
    /// creation price.
    pub _40pct: T,
    /// Uses UTXOs whose represented-block spot price is more than 50% above
    /// creation price.
    pub _50pct: T,
    /// Uses UTXOs whose represented-block spot price is more than 60% above
    /// creation price.
    pub _60pct: T,
    /// Uses UTXOs whose represented-block spot price is more than 70% above
    /// creation price.
    pub _70pct: T,
    /// Uses UTXOs whose represented-block spot price is more than 80% above
    /// creation price.
    pub _80pct: T,
    /// Uses UTXOs whose represented-block spot price is more than 90% above
    /// creation price.
    pub _90pct: T,
    /// Uses UTXOs whose represented-block spot price is more than 100% above
    /// creation price.
    pub _100pct: T,
    /// Uses UTXOs whose represented-block spot price is more than 200% above
    /// creation price.
    pub _200pct: T,
    /// Uses UTXOs whose represented-block spot price is more than 300% above
    /// creation price.
    pub _300pct: T,
    /// Uses UTXOs whose represented-block spot price is more than 500% above
    /// creation price.
    pub _500pct: T,
}

define_cohort_id!(
    ProfitId for Profit {
        Total => total,
        Over10Pct => _10pct,
        Over20Pct => _20pct,
        Over30Pct => _30pct,
        Over40Pct => _40pct,
        Over50Pct => _50pct,
        Over60Pct => _60pct,
        Over70Pct => _70pct,
        Over80Pct => _80pct,
        Over90Pct => _90pct,
        Over100Pct => _100pct,
        Over200Pct => _200pct,
        Over300Pct => _300pct,
        Over500Pct => _500pct,
    }
);
