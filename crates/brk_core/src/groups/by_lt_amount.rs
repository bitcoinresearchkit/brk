use crate::Sats;

use super::GroupFilter;

#[derive(Default, Clone)]
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
    pub fn as_mut_vec(&mut self) -> [&mut T; 13] {
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
    }
}

impl<T> ByLowerThanAmount<(GroupFilter, T)> {
    pub fn vecs(&self) -> [&T; 13] {
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
    }
}

impl<T> From<ByLowerThanAmount<T>> for ByLowerThanAmount<(GroupFilter, T)> {
    fn from(value: ByLowerThanAmount<T>) -> Self {
        Self {
            _10sats: (GroupFilter::LowerThan(Sats::_10.into()), value._10sats),
            _100sats: (GroupFilter::LowerThan(Sats::_100.into()), value._100sats),
            _1k_sats: (GroupFilter::LowerThan(Sats::_1K.into()), value._1k_sats),
            _10k_sats: (GroupFilter::LowerThan(Sats::_10K.into()), value._10k_sats),
            _100k_sats: (GroupFilter::LowerThan(Sats::_100K.into()), value._100k_sats),
            _1m_sats: (GroupFilter::LowerThan(Sats::_1M.into()), value._1m_sats),
            _10m_sats: (GroupFilter::LowerThan(Sats::_10M.into()), value._10m_sats),
            _1btc: (GroupFilter::LowerThan(Sats::_1BTC.into()), value._1btc),
            _10btc: (GroupFilter::LowerThan(Sats::_10BTC.into()), value._10btc),
            _100btc: (GroupFilter::LowerThan(Sats::_100BTC.into()), value._100btc),
            _1k_btc: (GroupFilter::LowerThan(Sats::_1K_BTC.into()), value._1k_btc),
            _10k_btc: (
                GroupFilter::LowerThan(Sats::_10K_BTC.into()),
                value._10k_btc,
            ),
            _100k_btc: (
                GroupFilter::LowerThan(Sats::_100K_BTC.into()),
                value._100k_btc,
            ),
        }
    }
}
