use brk_traversable::Traversable;
use brk_types::Sats;

use super::{Filter, Filtered};

#[derive(Default, Clone, Traversable)]
pub struct ByGreatEqualAmount<T> {
    pub _1sat: T,
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
}

impl<T> ByGreatEqualAmount<T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
            &mut self._1sat,
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
        ]
        .into_iter()
    }
}

impl<T> ByGreatEqualAmount<Filtered<T>> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [
            &self._1sat.1,
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
        ]
        .into_iter()
    }
}

impl<T> From<ByGreatEqualAmount<T>> for ByGreatEqualAmount<Filtered<T>> {
    fn from(value: ByGreatEqualAmount<T>) -> Self {
        Self {
            _1sat: (Filter::GreaterOrEqual(Sats::_1.into()), value._1sat).into(),
            _10sats: (Filter::GreaterOrEqual(Sats::_10.into()), value._10sats).into(),
            _100sats: (Filter::GreaterOrEqual(Sats::_100.into()), value._100sats).into(),
            _1k_sats: (Filter::GreaterOrEqual(Sats::_1K.into()), value._1k_sats).into(),
            _10k_sats: (Filter::GreaterOrEqual(Sats::_10K.into()), value._10k_sats).into(),
            _100k_sats: (Filter::GreaterOrEqual(Sats::_100K.into()), value._100k_sats).into(),
            _1m_sats: (Filter::GreaterOrEqual(Sats::_1M.into()), value._1m_sats).into(),
            _10m_sats: (Filter::GreaterOrEqual(Sats::_10M.into()), value._10m_sats).into(),
            _1btc: (Filter::GreaterOrEqual(Sats::_1BTC.into()), value._1btc).into(),
            _10btc: (Filter::GreaterOrEqual(Sats::_10BTC.into()), value._10btc).into(),
            _100btc: (Filter::GreaterOrEqual(Sats::_100BTC.into()), value._100btc).into(),
            _1k_btc: (Filter::GreaterOrEqual(Sats::_1K_BTC.into()), value._1k_btc).into(),
            _10k_btc: (
                Filter::GreaterOrEqual(Sats::_10K_BTC.into()),
                value._10k_btc,
            )
                .into(),
        }
    }
}
