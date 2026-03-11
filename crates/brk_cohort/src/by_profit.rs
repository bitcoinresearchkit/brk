use brk_traversable::Traversable;
use rayon::prelude::*;
use serde::Serialize;

use super::CohortName;

/// "At least X% profit" threshold names (15 thresholds).
pub const PROFIT_NAMES: ByProfit<CohortName> = ByProfit {
    breakeven: CohortName::new("utxos_in_profit", "≥0%", "In Profit (Breakeven+)"),
    _10pct: CohortName::new("utxos_over_10pct_in_profit", "≥10%", "10%+ Profit"),
    _20pct: CohortName::new("utxos_over_20pct_in_profit", "≥20%", "20%+ Profit"),
    _30pct: CohortName::new("utxos_over_30pct_in_profit", "≥30%", "30%+ Profit"),
    _40pct: CohortName::new("utxos_over_40pct_in_profit", "≥40%", "40%+ Profit"),
    _50pct: CohortName::new("utxos_over_50pct_in_profit", "≥50%", "50%+ Profit"),
    _60pct: CohortName::new("utxos_over_60pct_in_profit", "≥60%", "60%+ Profit"),
    _70pct: CohortName::new("utxos_over_70pct_in_profit", "≥70%", "70%+ Profit"),
    _80pct: CohortName::new("utxos_over_80pct_in_profit", "≥80%", "80%+ Profit"),
    _90pct: CohortName::new("utxos_over_90pct_in_profit", "≥90%", "90%+ Profit"),
    _100pct: CohortName::new("utxos_over_100pct_in_profit", "≥100%", "100%+ Profit"),
    _200pct: CohortName::new("utxos_over_200pct_in_profit", "≥200%", "200%+ Profit"),
    _300pct: CohortName::new("utxos_over_300pct_in_profit", "≥300%", "300%+ Profit"),
    _500pct: CohortName::new("utxos_over_500pct_in_profit", "≥500%", "500%+ Profit"),
    _1000pct: CohortName::new("utxos_over_1000pct_in_profit", "≥1000%", "1000%+ Profit"),
};

/// Number of profit thresholds.
pub const PROFIT_COUNT: usize = 15;

impl ByProfit<CohortName> {
    pub const fn names() -> &'static Self {
        &PROFIT_NAMES
    }
}

/// 15 "at least X% profit" aggregate thresholds.
///
/// Each is a prefix sum over the profitability ranges, from most profitable down.
#[derive(Default, Clone, Traversable, Serialize)]
pub struct ByProfit<T> {
    pub breakeven: T,
    pub _10pct: T,
    pub _20pct: T,
    pub _30pct: T,
    pub _40pct: T,
    pub _50pct: T,
    pub _60pct: T,
    pub _70pct: T,
    pub _80pct: T,
    pub _90pct: T,
    pub _100pct: T,
    pub _200pct: T,
    pub _300pct: T,
    pub _500pct: T,
    pub _1000pct: T,
}

impl<T> ByProfit<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(&'static str) -> T,
    {
        let n = &PROFIT_NAMES;
        Self {
            breakeven: create(n.breakeven.id),
            _10pct: create(n._10pct.id),
            _20pct: create(n._20pct.id),
            _30pct: create(n._30pct.id),
            _40pct: create(n._40pct.id),
            _50pct: create(n._50pct.id),
            _60pct: create(n._60pct.id),
            _70pct: create(n._70pct.id),
            _80pct: create(n._80pct.id),
            _90pct: create(n._90pct.id),
            _100pct: create(n._100pct.id),
            _200pct: create(n._200pct.id),
            _300pct: create(n._300pct.id),
            _500pct: create(n._500pct.id),
            _1000pct: create(n._1000pct.id),
        }
    }

    pub fn try_new<F, E>(mut create: F) -> Result<Self, E>
    where
        F: FnMut(&'static str) -> Result<T, E>,
    {
        let n = &PROFIT_NAMES;
        Ok(Self {
            breakeven: create(n.breakeven.id)?,
            _10pct: create(n._10pct.id)?,
            _20pct: create(n._20pct.id)?,
            _30pct: create(n._30pct.id)?,
            _40pct: create(n._40pct.id)?,
            _50pct: create(n._50pct.id)?,
            _60pct: create(n._60pct.id)?,
            _70pct: create(n._70pct.id)?,
            _80pct: create(n._80pct.id)?,
            _90pct: create(n._90pct.id)?,
            _100pct: create(n._100pct.id)?,
            _200pct: create(n._200pct.id)?,
            _300pct: create(n._300pct.id)?,
            _500pct: create(n._500pct.id)?,
            _1000pct: create(n._1000pct.id)?,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [
            &self.breakeven,
            &self._10pct,
            &self._20pct,
            &self._30pct,
            &self._40pct,
            &self._50pct,
            &self._60pct,
            &self._70pct,
            &self._80pct,
            &self._90pct,
            &self._100pct,
            &self._200pct,
            &self._300pct,
            &self._500pct,
            &self._1000pct,
        ]
        .into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
            &mut self.breakeven,
            &mut self._10pct,
            &mut self._20pct,
            &mut self._30pct,
            &mut self._40pct,
            &mut self._50pct,
            &mut self._60pct,
            &mut self._70pct,
            &mut self._80pct,
            &mut self._90pct,
            &mut self._100pct,
            &mut self._200pct,
            &mut self._300pct,
            &mut self._500pct,
            &mut self._1000pct,
        ]
        .into_iter()
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        [
            &mut self.breakeven,
            &mut self._10pct,
            &mut self._20pct,
            &mut self._30pct,
            &mut self._40pct,
            &mut self._50pct,
            &mut self._60pct,
            &mut self._70pct,
            &mut self._80pct,
            &mut self._90pct,
            &mut self._100pct,
            &mut self._200pct,
            &mut self._300pct,
            &mut self._500pct,
            &mut self._1000pct,
        ]
        .into_par_iter()
    }

    /// Access as array for indexed accumulation.
    pub fn as_array_mut(&mut self) -> [&mut T; PROFIT_COUNT] {
        [
            &mut self.breakeven,
            &mut self._10pct,
            &mut self._20pct,
            &mut self._30pct,
            &mut self._40pct,
            &mut self._50pct,
            &mut self._60pct,
            &mut self._70pct,
            &mut self._80pct,
            &mut self._90pct,
            &mut self._100pct,
            &mut self._200pct,
            &mut self._300pct,
            &mut self._500pct,
            &mut self._1000pct,
        ]
    }
}
