use brk_core::Sats;

use super::GroupFilter;

#[derive(Default, Clone)]
pub struct GroupedByUpToSize<T> {
    pub _1_000sats: T,
    pub _10_000sats: T,
    pub _1btc: T,
    pub _10btc: T,
    pub _100btc: T,
}

impl<T> GroupedByUpToSize<T> {
    pub fn as_mut_vec(&mut self) -> [&mut T; 5] {
        [
            &mut self._1_000sats,
            &mut self._10_000sats,
            &mut self._1btc,
            &mut self._10btc,
            &mut self._100btc,
        ]
    }
}

impl<T> GroupedByUpToSize<(GroupFilter, T)> {
    pub fn vecs(&self) -> [&T; 5] {
        [
            &self._1_000sats.1,
            &self._10_000sats.1,
            &self._1btc.1,
            &self._10btc.1,
            &self._100btc.1,
        ]
    }
}

impl<T> From<GroupedByUpToSize<T>> for GroupedByUpToSize<(GroupFilter, T)> {
    fn from(value: GroupedByUpToSize<T>) -> Self {
        Self {
            _1_000sats: (GroupFilter::To(1_000), value._1_000sats),
            _10_000sats: (GroupFilter::To(10_000), value._10_000sats),
            _1btc: (GroupFilter::To(usize::from(Sats::ONE_BTC)), value._1btc),
            _10btc: (
                GroupFilter::To(usize::from(10 * Sats::ONE_BTC)),
                value._10btc,
            ),
            _100btc: (
                GroupFilter::To(usize::from(100 * Sats::ONE_BTC)),
                value._100btc,
            ),
        }
    }
}
