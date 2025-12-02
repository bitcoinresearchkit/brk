use brk_traversable::Traversable;
use brk_types::Sats;

use super::{AmountFilter, Filter};

#[derive(Default, Clone, Traversable)]
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

impl<T> ByLowerThanAmount<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter) -> T,
    {
        Self {
            _10sats: create(Filter::Amount(AmountFilter::LowerThan(Sats::_10))),
            _100sats: create(Filter::Amount(AmountFilter::LowerThan(Sats::_100))),
            _1k_sats: create(Filter::Amount(AmountFilter::LowerThan(Sats::_1K))),
            _10k_sats: create(Filter::Amount(AmountFilter::LowerThan(Sats::_10K))),
            _100k_sats: create(Filter::Amount(AmountFilter::LowerThan(Sats::_100K))),
            _1m_sats: create(Filter::Amount(AmountFilter::LowerThan(Sats::_1M))),
            _10m_sats: create(Filter::Amount(AmountFilter::LowerThan(Sats::_10M))),
            _1btc: create(Filter::Amount(AmountFilter::LowerThan(Sats::_1BTC))),
            _10btc: create(Filter::Amount(AmountFilter::LowerThan(Sats::_10BTC))),
            _100btc: create(Filter::Amount(AmountFilter::LowerThan(Sats::_100BTC))),
            _1k_btc: create(Filter::Amount(AmountFilter::LowerThan(Sats::_1K_BTC))),
            _10k_btc: create(Filter::Amount(AmountFilter::LowerThan(Sats::_10K_BTC))),
            _100k_btc: create(Filter::Amount(AmountFilter::LowerThan(Sats::_100K_BTC))),
        }
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
}
