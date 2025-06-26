use crate::Sats;

use super::GroupFilter;

#[derive(Default, Clone)]
pub struct GroupedByFromSize<T> {
    pub _1_000sats: T,
    pub _1btc: T,
    pub _10btc: T,
    pub _100btc: T,
}

impl<T> GroupedByFromSize<T> {
    pub fn as_mut_vec(&mut self) -> [&mut T; 4] {
        [
            &mut self._1_000sats,
            &mut self._1btc,
            &mut self._10btc,
            &mut self._100btc,
        ]
    }
}

impl<T> GroupedByFromSize<(GroupFilter, T)> {
    pub fn vecs(&self) -> [&T; 4] {
        [
            &self._1_000sats.1,
            &self._1btc.1,
            &self._10btc.1,
            &self._100btc.1,
        ]
    }
}

impl<T> From<GroupedByFromSize<T>> for GroupedByFromSize<(GroupFilter, T)> {
    fn from(value: GroupedByFromSize<T>) -> Self {
        Self {
            _1_000sats: (GroupFilter::From(1_000), value._1_000sats),
            _1btc: (GroupFilter::From(usize::from(Sats::ONE_BTC)), value._1btc),
            _10btc: (
                GroupFilter::From(usize::from(10 * Sats::ONE_BTC)),
                value._10btc,
            ),
            _100btc: (
                GroupFilter::From(usize::from(100 * Sats::ONE_BTC)),
                value._100btc,
            ),
        }
    }
}
