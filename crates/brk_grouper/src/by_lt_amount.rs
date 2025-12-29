use brk_traversable::Traversable;
use brk_types::Sats;
use rayon::prelude::*;
use serde::Serialize;

use super::{AmountFilter, CohortName, Filter};

/// Lower-than amount thresholds
pub const LT_AMOUNT_THRESHOLDS: ByLowerThanAmount<Sats> = ByLowerThanAmount {
    _10sats: Sats::_10,
    _100sats: Sats::_100,
    _1k_sats: Sats::_1K,
    _10k_sats: Sats::_10K,
    _100k_sats: Sats::_100K,
    _1m_sats: Sats::_1M,
    _10m_sats: Sats::_10M,
    _1btc: Sats::_1BTC,
    _10btc: Sats::_10BTC,
    _100btc: Sats::_100BTC,
    _1k_btc: Sats::_1K_BTC,
    _10k_btc: Sats::_10K_BTC,
    _100k_btc: Sats::_100K_BTC,
};

/// Lower-than amount names
pub const LT_AMOUNT_NAMES: ByLowerThanAmount<CohortName> = ByLowerThanAmount {
    _10sats: CohortName::new("under_10sats", "<10 sats", "Under 10 Sats"),
    _100sats: CohortName::new("under_100sats", "<100 sats", "Under 100 Sats"),
    _1k_sats: CohortName::new("under_1k_sats", "<1k sats", "Under 1K Sats"),
    _10k_sats: CohortName::new("under_10k_sats", "<10k sats", "Under 10K Sats"),
    _100k_sats: CohortName::new("under_100k_sats", "<100k sats", "Under 100K Sats"),
    _1m_sats: CohortName::new("under_1m_sats", "<1M sats", "Under 1M Sats"),
    _10m_sats: CohortName::new("under_10m_sats", "<0.1 BTC", "Under 0.1 BTC"),
    _1btc: CohortName::new("under_1btc", "<1 BTC", "Under 1 BTC"),
    _10btc: CohortName::new("under_10btc", "<10 BTC", "Under 10 BTC"),
    _100btc: CohortName::new("under_100btc", "<100 BTC", "Under 100 BTC"),
    _1k_btc: CohortName::new("under_1k_btc", "<1k BTC", "Under 1K BTC"),
    _10k_btc: CohortName::new("under_10k_btc", "<10k BTC", "Under 10K BTC"),
    _100k_btc: CohortName::new("under_100k_btc", "<100k BTC", "Under 100K BTC"),
};

/// Lower-than amount filters
pub const LT_AMOUNT_FILTERS: ByLowerThanAmount<Filter> = ByLowerThanAmount {
    _10sats: Filter::Amount(AmountFilter::LowerThan(LT_AMOUNT_THRESHOLDS._10sats)),
    _100sats: Filter::Amount(AmountFilter::LowerThan(LT_AMOUNT_THRESHOLDS._100sats)),
    _1k_sats: Filter::Amount(AmountFilter::LowerThan(LT_AMOUNT_THRESHOLDS._1k_sats)),
    _10k_sats: Filter::Amount(AmountFilter::LowerThan(LT_AMOUNT_THRESHOLDS._10k_sats)),
    _100k_sats: Filter::Amount(AmountFilter::LowerThan(LT_AMOUNT_THRESHOLDS._100k_sats)),
    _1m_sats: Filter::Amount(AmountFilter::LowerThan(LT_AMOUNT_THRESHOLDS._1m_sats)),
    _10m_sats: Filter::Amount(AmountFilter::LowerThan(LT_AMOUNT_THRESHOLDS._10m_sats)),
    _1btc: Filter::Amount(AmountFilter::LowerThan(LT_AMOUNT_THRESHOLDS._1btc)),
    _10btc: Filter::Amount(AmountFilter::LowerThan(LT_AMOUNT_THRESHOLDS._10btc)),
    _100btc: Filter::Amount(AmountFilter::LowerThan(LT_AMOUNT_THRESHOLDS._100btc)),
    _1k_btc: Filter::Amount(AmountFilter::LowerThan(LT_AMOUNT_THRESHOLDS._1k_btc)),
    _10k_btc: Filter::Amount(AmountFilter::LowerThan(LT_AMOUNT_THRESHOLDS._10k_btc)),
    _100k_btc: Filter::Amount(AmountFilter::LowerThan(LT_AMOUNT_THRESHOLDS._100k_btc)),
};

#[derive(Default, Clone, Traversable, Serialize)]
pub struct ByLowerThanAmount<T> {
    pub _10sats: T,
    pub _100sats: T,
    pub _1k_sats: T,
    pub _10k_sats: T,
    pub _100k_sats: T,
    pub _1m_sats: T,
    pub _10m_sats: T,
    pub _1btc: T,
    pub _10btc: T,
    pub _100btc: T,
    pub _1k_btc: T,
    pub _10k_btc: T,
    pub _100k_btc: T,
}

impl ByLowerThanAmount<CohortName> {
    pub const fn names() -> &'static Self {
        &LT_AMOUNT_NAMES
    }
}

impl<T> ByLowerThanAmount<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter, &'static str) -> T,
    {
        let f = LT_AMOUNT_FILTERS;
        let n = LT_AMOUNT_NAMES;
        Self {
            _10sats: create(f._10sats.clone(), n._10sats.id),
            _100sats: create(f._100sats.clone(), n._100sats.id),
            _1k_sats: create(f._1k_sats.clone(), n._1k_sats.id),
            _10k_sats: create(f._10k_sats.clone(), n._10k_sats.id),
            _100k_sats: create(f._100k_sats.clone(), n._100k_sats.id),
            _1m_sats: create(f._1m_sats.clone(), n._1m_sats.id),
            _10m_sats: create(f._10m_sats.clone(), n._10m_sats.id),
            _1btc: create(f._1btc.clone(), n._1btc.id),
            _10btc: create(f._10btc.clone(), n._10btc.id),
            _100btc: create(f._100btc.clone(), n._100btc.id),
            _1k_btc: create(f._1k_btc.clone(), n._1k_btc.id),
            _10k_btc: create(f._10k_btc.clone(), n._10k_btc.id),
            _100k_btc: create(f._100k_btc.clone(), n._100k_btc.id),
        }
    }

    pub fn try_new<F, E>(mut create: F) -> Result<Self, E>
    where
        F: FnMut(Filter, &'static str) -> Result<T, E>,
    {
        let f = LT_AMOUNT_FILTERS;
        let n = LT_AMOUNT_NAMES;
        Ok(Self {
            _10sats: create(f._10sats.clone(), n._10sats.id)?,
            _100sats: create(f._100sats.clone(), n._100sats.id)?,
            _1k_sats: create(f._1k_sats.clone(), n._1k_sats.id)?,
            _10k_sats: create(f._10k_sats.clone(), n._10k_sats.id)?,
            _100k_sats: create(f._100k_sats.clone(), n._100k_sats.id)?,
            _1m_sats: create(f._1m_sats.clone(), n._1m_sats.id)?,
            _10m_sats: create(f._10m_sats.clone(), n._10m_sats.id)?,
            _1btc: create(f._1btc.clone(), n._1btc.id)?,
            _10btc: create(f._10btc.clone(), n._10btc.id)?,
            _100btc: create(f._100btc.clone(), n._100btc.id)?,
            _1k_btc: create(f._1k_btc.clone(), n._1k_btc.id)?,
            _10k_btc: create(f._10k_btc.clone(), n._10k_btc.id)?,
            _100k_btc: create(f._100k_btc.clone(), n._100k_btc.id)?,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [
            &self._10sats,
            &self._100sats,
            &self._1k_sats,
            &self._10k_sats,
            &self._100k_sats,
            &self._1m_sats,
            &self._10m_sats,
            &self._1btc,
            &self._10btc,
            &self._100btc,
            &self._1k_btc,
            &self._10k_btc,
            &self._100k_btc,
        ]
        .into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
            &mut self._10sats,
            &mut self._100sats,
            &mut self._1k_sats,
            &mut self._10k_sats,
            &mut self._100k_sats,
            &mut self._1m_sats,
            &mut self._10m_sats,
            &mut self._1btc,
            &mut self._10btc,
            &mut self._100btc,
            &mut self._1k_btc,
            &mut self._10k_btc,
            &mut self._100k_btc,
        ]
        .into_iter()
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        [
            &mut self._10sats,
            &mut self._100sats,
            &mut self._1k_sats,
            &mut self._10k_sats,
            &mut self._100k_sats,
            &mut self._1m_sats,
            &mut self._10m_sats,
            &mut self._1btc,
            &mut self._10btc,
            &mut self._100btc,
            &mut self._1k_btc,
            &mut self._10k_btc,
            &mut self._100k_btc,
        ]
        .into_par_iter()
    }
}
