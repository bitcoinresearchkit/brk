use brk_vecs::{IVecs, TreeNode};
use vecdb::AnyCollectableVec;

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

impl<T> ByLowerThanAmount<(GroupFilter, T)> {
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

impl<T: IVecs> IVecs for ByLowerThanAmount<(GroupFilter, T)> {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            [
                ("10sats", &self._10sats),
                ("100sats", &self._100sats),
                ("1k_sats", &self._1k_sats),
                ("10k_sats", &self._10k_sats),
                ("100k_sats", &self._100k_sats),
                ("1m_sats", &self._1m_sats),
                ("10m_sats", &self._10m_sats),
                ("1btc", &self._1btc),
                ("10btc", &self._10btc),
                ("100btc", &self._100btc),
                ("1k_btc", &self._1k_btc),
                ("10k_btc", &self._10k_btc),
                ("100k_btc", &self._100k_btc),
            ]
            .into_iter()
            .map(|(name, (_, field))| (name.to_string(), field.to_tree_node()))
            .collect(),
        )
    }

    fn iter(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec>> =
            Box::new(self._10sats.1.iter());
        iter = Box::new(iter.chain(self._100sats.1.iter()));
        iter = Box::new(iter.chain(self._1k_sats.1.iter()));
        iter = Box::new(iter.chain(self._10k_sats.1.iter()));
        iter = Box::new(iter.chain(self._100k_sats.1.iter()));
        iter = Box::new(iter.chain(self._1m_sats.1.iter()));
        iter = Box::new(iter.chain(self._10m_sats.1.iter()));
        iter = Box::new(iter.chain(self._1btc.1.iter()));
        iter = Box::new(iter.chain(self._10btc.1.iter()));
        iter = Box::new(iter.chain(self._100btc.1.iter()));
        iter = Box::new(iter.chain(self._1k_btc.1.iter()));
        iter = Box::new(iter.chain(self._10k_btc.1.iter()));
        iter = Box::new(iter.chain(self._100k_btc.1.iter()));
        iter
    }
}
