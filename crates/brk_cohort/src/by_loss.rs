use brk_traversable::Traversable;
use rayon::prelude::*;
use serde::Serialize;

use super::CohortName;

/// "At least X% loss" threshold names (10 thresholds).
pub const LOSS_NAMES: ByLoss<CohortName> = ByLoss {
    breakeven: CohortName::new("utxos_in_loss", "<0%", "In Loss (Below Breakeven)"),
    _10pct: CohortName::new("utxos_over_10pct_in_loss", "≥10%L", "10%+ Loss"),
    _20pct: CohortName::new("utxos_over_20pct_in_loss", "≥20%L", "20%+ Loss"),
    _30pct: CohortName::new("utxos_over_30pct_in_loss", "≥30%L", "30%+ Loss"),
    _40pct: CohortName::new("utxos_over_40pct_in_loss", "≥40%L", "40%+ Loss"),
    _50pct: CohortName::new("utxos_over_50pct_in_loss", "≥50%L", "50%+ Loss"),
    _60pct: CohortName::new("utxos_over_60pct_in_loss", "≥60%L", "60%+ Loss"),
    _70pct: CohortName::new("utxos_over_70pct_in_loss", "≥70%L", "70%+ Loss"),
    _80pct: CohortName::new("utxos_over_80pct_in_loss", "≥80%L", "80%+ Loss"),
    _90pct: CohortName::new("utxos_over_90pct_in_loss", "≥90%L", "90%+ Loss"),
};

/// Number of loss thresholds.
pub const LOSS_COUNT: usize = 10;

impl ByLoss<CohortName> {
    pub const fn names() -> &'static Self {
        &LOSS_NAMES
    }
}

/// 10 "at least X% loss" aggregate thresholds.
///
/// Each is a suffix sum over the profitability ranges, from most loss-making up.
#[derive(Default, Clone, Traversable, Serialize)]
pub struct ByLoss<T> {
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
}

impl<T> ByLoss<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(&'static str) -> T,
    {
        let n = &LOSS_NAMES;
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
        }
    }

    pub fn try_new<F, E>(mut create: F) -> Result<Self, E>
    where
        F: FnMut(&'static str) -> Result<T, E>,
    {
        let n = &LOSS_NAMES;
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
        ]
        .into_par_iter()
    }

    /// Access as array for indexed accumulation.
    pub fn as_array_mut(&mut self) -> [&mut T; LOSS_COUNT] {
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
        ]
    }
}
