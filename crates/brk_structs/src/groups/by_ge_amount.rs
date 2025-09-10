use crate::Sats;

use super::GroupFilter;

#[derive(Default, Clone)]
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

impl<T> ByGreatEqualAmount<(GroupFilter, T)> {
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

impl<T> From<ByGreatEqualAmount<T>> for ByGreatEqualAmount<(GroupFilter, T)> {
    fn from(value: ByGreatEqualAmount<T>) -> Self {
        Self {
            _1sat: (GroupFilter::GreaterOrEqual(Sats::_1.into()), value._1sat),
            _10sats: (GroupFilter::GreaterOrEqual(Sats::_10.into()), value._10sats),
            _100sats: (
                GroupFilter::GreaterOrEqual(Sats::_100.into()),
                value._100sats,
            ),
            _1k_sats: (
                GroupFilter::GreaterOrEqual(Sats::_1K.into()),
                value._1k_sats,
            ),
            _10k_sats: (
                GroupFilter::GreaterOrEqual(Sats::_10K.into()),
                value._10k_sats,
            ),
            _100k_sats: (
                GroupFilter::GreaterOrEqual(Sats::_100K.into()),
                value._100k_sats,
            ),
            _1m_sats: (
                GroupFilter::GreaterOrEqual(Sats::_1M.into()),
                value._1m_sats,
            ),
            _10m_sats: (
                GroupFilter::GreaterOrEqual(Sats::_10M.into()),
                value._10m_sats,
            ),
            _1btc: (GroupFilter::GreaterOrEqual(Sats::_1BTC.into()), value._1btc),
            _10btc: (
                GroupFilter::GreaterOrEqual(Sats::_10BTC.into()),
                value._10btc,
            ),
            _100btc: (
                GroupFilter::GreaterOrEqual(Sats::_100BTC.into()),
                value._100btc,
            ),
            _1k_btc: (
                GroupFilter::GreaterOrEqual(Sats::_1K_BTC.into()),
                value._1k_btc,
            ),
            _10k_btc: (
                GroupFilter::GreaterOrEqual(Sats::_10K_BTC.into()),
                value._10k_btc,
            ),
        }
    }
}
