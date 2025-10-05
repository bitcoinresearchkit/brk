use std::ops::{Add, AddAssign};

use brk_vecs::{IVecs, TreeNode};
use vecdb::AnyCollectableVec;

use crate::Sats;

use super::GroupFilter;

#[derive(Debug, Default, Clone)]
pub struct ByAmountRange<T> {
    pub _0sats: T,
    pub _1sat_to_10sats: T,
    pub _10sats_to_100sats: T,
    pub _100sats_to_1k_sats: T,
    pub _1k_sats_to_10k_sats: T,
    pub _10k_sats_to_100k_sats: T,
    pub _100k_sats_to_1m_sats: T,
    pub _1m_sats_to_10m_sats: T,
    pub _10m_sats_to_1btc: T,
    pub _1btc_to_10btc: T,
    pub _10btc_to_100btc: T,
    pub _100btc_to_1k_btc: T,
    pub _1k_btc_to_10k_btc: T,
    pub _10k_btc_to_100k_btc: T,
    pub _100k_btc_or_more: T,
}

impl<T> From<ByAmountRange<T>> for ByAmountRange<(GroupFilter, T)> {
    fn from(value: ByAmountRange<T>) -> Self {
        #[allow(clippy::inconsistent_digit_grouping)]
        Self {
            _0sats: (GroupFilter::LowerThan(Sats::_1.into()), value._0sats),
            _1sat_to_10sats: (
                GroupFilter::Range(Sats::_1.into()..Sats::_10.into()),
                value._1sat_to_10sats,
            ),
            _10sats_to_100sats: (
                GroupFilter::Range(Sats::_10.into()..Sats::_100.into()),
                value._10sats_to_100sats,
            ),
            _100sats_to_1k_sats: (
                GroupFilter::Range(Sats::_100.into()..Sats::_1K.into()),
                value._100sats_to_1k_sats,
            ),
            _1k_sats_to_10k_sats: (
                GroupFilter::Range(Sats::_1K.into()..Sats::_10K.into()),
                value._1k_sats_to_10k_sats,
            ),
            _10k_sats_to_100k_sats: (
                GroupFilter::Range(Sats::_10K.into()..Sats::_100K.into()),
                value._10k_sats_to_100k_sats,
            ),
            _100k_sats_to_1m_sats: (
                GroupFilter::Range(Sats::_100K.into()..Sats::_1M.into()),
                value._100k_sats_to_1m_sats,
            ),
            _1m_sats_to_10m_sats: (
                GroupFilter::Range(Sats::_1M.into()..Sats::_10M.into()),
                value._1m_sats_to_10m_sats,
            ),
            _10m_sats_to_1btc: (
                GroupFilter::Range(Sats::_10M.into()..Sats::_1BTC.into()),
                value._10m_sats_to_1btc,
            ),
            _1btc_to_10btc: (
                GroupFilter::Range(Sats::_1BTC.into()..Sats::_10BTC.into()),
                value._1btc_to_10btc,
            ),
            _10btc_to_100btc: (
                GroupFilter::Range(Sats::_10BTC.into()..Sats::_100BTC.into()),
                value._10btc_to_100btc,
            ),
            _100btc_to_1k_btc: (
                GroupFilter::Range(Sats::_100BTC.into()..Sats::_1K_BTC.into()),
                value._100btc_to_1k_btc,
            ),
            _1k_btc_to_10k_btc: (
                GroupFilter::Range(Sats::_1K_BTC.into()..Sats::_10K_BTC.into()),
                value._1k_btc_to_10k_btc,
            ),
            _10k_btc_to_100k_btc: (
                GroupFilter::Range(Sats::_10K_BTC.into()..Sats::_100K_BTC.into()),
                value._10k_btc_to_100k_btc,
            ),
            _100k_btc_or_more: (
                GroupFilter::GreaterOrEqual(Sats::_100K_BTC.into()),
                value._100k_btc_or_more,
            ),
        }
    }
}

impl<T> ByAmountRange<T> {
    #[allow(clippy::inconsistent_digit_grouping)]
    pub fn get_mut(&mut self, value: Sats) -> &mut T {
        if value == Sats::ZERO {
            &mut self._0sats
        } else if value < Sats::_10 {
            &mut self._1sat_to_10sats
        } else if value < Sats::_100 {
            &mut self._10sats_to_100sats
        } else if value < Sats::_1K {
            &mut self._100sats_to_1k_sats
        } else if value < Sats::_10K {
            &mut self._1k_sats_to_10k_sats
        } else if value < Sats::_100K {
            &mut self._10k_sats_to_100k_sats
        } else if value < Sats::_1M {
            &mut self._100k_sats_to_1m_sats
        } else if value < Sats::_10M {
            &mut self._1m_sats_to_10m_sats
        } else if value < Sats::_1BTC {
            &mut self._10m_sats_to_1btc
        } else if value < Sats::_10BTC {
            &mut self._1btc_to_10btc
        } else if value < Sats::_100BTC {
            &mut self._10btc_to_100btc
        } else if value < Sats::_1K_BTC {
            &mut self._100btc_to_1k_btc
        } else if value < Sats::_10K_BTC {
            &mut self._1k_btc_to_10k_btc
        } else if value < Sats::_100K_BTC {
            &mut self._10k_btc_to_100k_btc
        } else {
            &mut self._100k_btc_or_more
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [
            &self._0sats,
            &self._1sat_to_10sats,
            &self._10sats_to_100sats,
            &self._100sats_to_1k_sats,
            &self._1k_sats_to_10k_sats,
            &self._10k_sats_to_100k_sats,
            &self._100k_sats_to_1m_sats,
            &self._1m_sats_to_10m_sats,
            &self._10m_sats_to_1btc,
            &self._1btc_to_10btc,
            &self._10btc_to_100btc,
            &self._100btc_to_1k_btc,
            &self._1k_btc_to_10k_btc,
            &self._10k_btc_to_100k_btc,
            &self._100k_btc_or_more,
        ]
        .into_iter()
    }

    pub fn iter_typed(&self) -> impl Iterator<Item = (Sats, &T)> {
        [
            (Sats::ZERO, &self._0sats),
            (Sats::_1, &self._1sat_to_10sats),
            (Sats::_10, &self._10sats_to_100sats),
            (Sats::_100, &self._100sats_to_1k_sats),
            (Sats::_1K, &self._1k_sats_to_10k_sats),
            (Sats::_10K, &self._10k_sats_to_100k_sats),
            (Sats::_100K, &self._100k_sats_to_1m_sats),
            (Sats::_1M, &self._1m_sats_to_10m_sats),
            (Sats::_10M, &self._10m_sats_to_1btc),
            (Sats::_1BTC, &self._1btc_to_10btc),
            (Sats::_10BTC, &self._10btc_to_100btc),
            (Sats::_100BTC, &self._100btc_to_1k_btc),
            (Sats::_1K_BTC, &self._1k_btc_to_10k_btc),
            (Sats::_10K_BTC, &self._10k_btc_to_100k_btc),
            (Sats::_100K_BTC, &self._100k_btc_or_more),
        ]
        .into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [
            &mut self._0sats,
            &mut self._1sat_to_10sats,
            &mut self._10sats_to_100sats,
            &mut self._100sats_to_1k_sats,
            &mut self._1k_sats_to_10k_sats,
            &mut self._10k_sats_to_100k_sats,
            &mut self._100k_sats_to_1m_sats,
            &mut self._1m_sats_to_10m_sats,
            &mut self._10m_sats_to_1btc,
            &mut self._1btc_to_10btc,
            &mut self._10btc_to_100btc,
            &mut self._100btc_to_1k_btc,
            &mut self._1k_btc_to_10k_btc,
            &mut self._10k_btc_to_100k_btc,
            &mut self._100k_btc_or_more,
        ]
        .into_iter()
    }
}

impl<T> ByAmountRange<(GroupFilter, T)> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [
            &self._0sats.1,
            &self._1sat_to_10sats.1,
            &self._10sats_to_100sats.1,
            &self._100sats_to_1k_sats.1,
            &self._1k_sats_to_10k_sats.1,
            &self._10k_sats_to_100k_sats.1,
            &self._100k_sats_to_1m_sats.1,
            &self._1m_sats_to_10m_sats.1,
            &self._10m_sats_to_1btc.1,
            &self._1btc_to_10btc.1,
            &self._10btc_to_100btc.1,
            &self._100btc_to_1k_btc.1,
            &self._1k_btc_to_10k_btc.1,
            &self._10k_btc_to_100k_btc.1,
            &self._100k_btc_or_more.1,
        ]
        .into_iter()
    }
}

impl<T> Add for ByAmountRange<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            _0sats: self._0sats + rhs._0sats,
            _1sat_to_10sats: self._1sat_to_10sats + rhs._1sat_to_10sats,
            _10sats_to_100sats: self._10sats_to_100sats + rhs._10sats_to_100sats,
            _100sats_to_1k_sats: self._100sats_to_1k_sats + rhs._100sats_to_1k_sats,
            _1k_sats_to_10k_sats: self._1k_sats_to_10k_sats + rhs._1k_sats_to_10k_sats,
            _10k_sats_to_100k_sats: self._10k_sats_to_100k_sats + rhs._10k_sats_to_100k_sats,
            _100k_sats_to_1m_sats: self._100k_sats_to_1m_sats + rhs._100k_sats_to_1m_sats,
            _1m_sats_to_10m_sats: self._1m_sats_to_10m_sats + rhs._1m_sats_to_10m_sats,
            _10m_sats_to_1btc: self._10m_sats_to_1btc + rhs._10m_sats_to_1btc,
            _1btc_to_10btc: self._1btc_to_10btc + rhs._1btc_to_10btc,
            _10btc_to_100btc: self._10btc_to_100btc + rhs._10btc_to_100btc,
            _100btc_to_1k_btc: self._100btc_to_1k_btc + rhs._100btc_to_1k_btc,
            _1k_btc_to_10k_btc: self._1k_btc_to_10k_btc + rhs._1k_btc_to_10k_btc,
            _10k_btc_to_100k_btc: self._10k_btc_to_100k_btc + rhs._10k_btc_to_100k_btc,
            _100k_btc_or_more: self._100k_btc_or_more + rhs._100k_btc_or_more,
        }
    }
}

impl<T> AddAssign for ByAmountRange<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self._0sats += rhs._0sats;
        self._1sat_to_10sats += rhs._1sat_to_10sats;
        self._10sats_to_100sats += rhs._10sats_to_100sats;
        self._100sats_to_1k_sats += rhs._100sats_to_1k_sats;
        self._1k_sats_to_10k_sats += rhs._1k_sats_to_10k_sats;
        self._10k_sats_to_100k_sats += rhs._10k_sats_to_100k_sats;
        self._100k_sats_to_1m_sats += rhs._100k_sats_to_1m_sats;
        self._1m_sats_to_10m_sats += rhs._1m_sats_to_10m_sats;
        self._10m_sats_to_1btc += rhs._10m_sats_to_1btc;
        self._1btc_to_10btc += rhs._1btc_to_10btc;
        self._10btc_to_100btc += rhs._10btc_to_100btc;
        self._100btc_to_1k_btc += rhs._100btc_to_1k_btc;
        self._1k_btc_to_10k_btc += rhs._1k_btc_to_10k_btc;
        self._10k_btc_to_100k_btc += rhs._10k_btc_to_100k_btc;
        self._100k_btc_or_more += rhs._100k_btc_or_more;
    }
}

impl<T: IVecs> IVecs for ByAmountRange<(GroupFilter, T)> {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            [
                ("0sats", &self._0sats),
                ("1sat_to_10sats", &self._1sat_to_10sats),
                ("10sats_to_100sats", &self._10sats_to_100sats),
                ("100sats_to_1k_sats", &self._100sats_to_1k_sats),
                ("1k_sats_to_10k_sats", &self._1k_sats_to_10k_sats),
                ("10k_sats_to_100k_sats", &self._10k_sats_to_100k_sats),
                ("100k_sats_to_1m_sats", &self._100k_sats_to_1m_sats),
                ("1m_sats_to_10m_sats", &self._1m_sats_to_10m_sats),
                ("10m_sats_to_1btc", &self._10m_sats_to_1btc),
                ("1btc_to_10btc", &self._1btc_to_10btc),
                ("10btc_to_100btc", &self._10btc_to_100btc),
                ("100btc_to_1k_btc", &self._100btc_to_1k_btc),
                ("1k_btc_to_10k_btc", &self._1k_btc_to_10k_btc),
                ("10k_btc_to_100k_btc", &self._10k_btc_to_100k_btc),
                ("100k_btc_or_more", &self._100k_btc_or_more),
            ]
            .into_iter()
            .map(|(name, (_, field))| (name.to_string(), field.to_tree_node()))
            .collect(),
        )
    }

    fn iter(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec>> =
            Box::new(self._0sats.1.iter());
        iter = Box::new(iter.chain(self._1sat_to_10sats.1.iter()));
        iter = Box::new(iter.chain(self._10sats_to_100sats.1.iter()));
        iter = Box::new(iter.chain(self._100sats_to_1k_sats.1.iter()));
        iter = Box::new(iter.chain(self._1k_sats_to_10k_sats.1.iter()));
        iter = Box::new(iter.chain(self._10k_sats_to_100k_sats.1.iter()));
        iter = Box::new(iter.chain(self._100k_sats_to_1m_sats.1.iter()));
        iter = Box::new(iter.chain(self._1m_sats_to_10m_sats.1.iter()));
        iter = Box::new(iter.chain(self._10m_sats_to_1btc.1.iter()));
        iter = Box::new(iter.chain(self._1btc_to_10btc.1.iter()));
        iter = Box::new(iter.chain(self._10btc_to_100btc.1.iter()));
        iter = Box::new(iter.chain(self._100btc_to_1k_btc.1.iter()));
        iter = Box::new(iter.chain(self._1k_btc_to_10k_btc.1.iter()));
        iter = Box::new(iter.chain(self._10k_btc_to_100k_btc.1.iter()));
        iter = Box::new(iter.chain(self._100k_btc_or_more.1.iter()));
        iter
    }
}
