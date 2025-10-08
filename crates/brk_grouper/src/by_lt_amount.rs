use brk_structs::Sats;
use brk_traversable::Traversable;

use super::{Filter, Filtered};

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

impl<T> ByLowerThanAmount<Filtered<T>> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [
            &self._10sats.1,
            &self._100sats.1,
            &self._1k_sats.1,
            &self._10k_sats.1,
            &self._100k_sats.1,
            &self._1m_sats.1,
            &self._10m_sats.1,
            &self._1btc.1,
            &self._10btc.1,
            &self._100btc.1,
            &self._1k_btc.1,
            &self._10k_btc.1,
            &self._100k_btc.1,
        ]
        .into_iter()
    }
}

impl<T> From<ByLowerThanAmount<T>> for ByLowerThanAmount<Filtered<T>> {
    fn from(value: ByLowerThanAmount<T>) -> Self {
        Self {
            _10sats: (Filter::LowerThan(Sats::_10.into()), value._10sats).into(),
            _100sats: (Filter::LowerThan(Sats::_100.into()), value._100sats).into(),
            _1k_sats: (Filter::LowerThan(Sats::_1K.into()), value._1k_sats).into(),
            _10k_sats: (Filter::LowerThan(Sats::_10K.into()), value._10k_sats).into(),
            _100k_sats: (Filter::LowerThan(Sats::_100K.into()), value._100k_sats).into(),
            _1m_sats: (Filter::LowerThan(Sats::_1M.into()), value._1m_sats).into(),
            _10m_sats: (Filter::LowerThan(Sats::_10M.into()), value._10m_sats).into(),
            _1btc: (Filter::LowerThan(Sats::_1BTC.into()), value._1btc).into(),
            _10btc: (Filter::LowerThan(Sats::_10BTC.into()), value._10btc).into(),
            _100btc: (Filter::LowerThan(Sats::_100BTC.into()), value._100btc).into(),
            _1k_btc: (Filter::LowerThan(Sats::_1K_BTC.into()), value._1k_btc).into(),
            _10k_btc: (Filter::LowerThan(Sats::_10K_BTC.into()), value._10k_btc).into(),
            _100k_btc: (Filter::LowerThan(Sats::_100K_BTC.into()), value._100k_btc).into(),
        }
    }
}
