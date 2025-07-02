use std::ops::{Add, AddAssign};

use brk_core::Sats;

use super::GroupFilter;

#[derive(Debug, Default, Clone)]
pub struct GroupedBySizeRange<T> {
    pub _0sats: T,
    pub from_1sat_to_10sats: T,
    pub from_10sats_to_100sats: T,
    pub from_100sats_to_1_000sats: T,
    pub from_1_000sats_to_10_000sats: T,
    pub from_10_000sats_to_100_000sats: T,
    pub from_100_000sats_to_1_000_000sats: T,
    pub from_1_000_000sats_to_10_000_000sats: T,
    pub from_10_000_000sats_to_1btc: T,
    pub from_1btc_to_10btc: T,
    pub from_10btc_to_100btc: T,
    pub from_100btc_to_1_000btc: T,
    pub from_1_000btc_to_10_000btc: T,
    pub from_10_000btc_to_100_000btc: T,
    pub from_100_000btc: T,
}

impl<T> From<GroupedBySizeRange<T>> for GroupedBySizeRange<(GroupFilter, T)> {
    fn from(value: GroupedBySizeRange<T>) -> Self {
        #[allow(clippy::inconsistent_digit_grouping)]
        Self {
            _0sats: (GroupFilter::To(1), value._0sats),
            from_1sat_to_10sats: (GroupFilter::Range(1..10), value.from_1sat_to_10sats),
            from_10sats_to_100sats: (GroupFilter::Range(10..100), value.from_10sats_to_100sats),
            from_100sats_to_1_000sats: (
                GroupFilter::Range(100..1_000),
                value.from_100sats_to_1_000sats,
            ),
            from_1_000sats_to_10_000sats: (
                GroupFilter::Range(1_000..10_000),
                value.from_1_000sats_to_10_000sats,
            ),
            from_10_000sats_to_100_000sats: (
                GroupFilter::Range(10_000..100_000),
                value.from_10_000sats_to_100_000sats,
            ),
            from_100_000sats_to_1_000_000sats: (
                GroupFilter::Range(100_000..1_000_000),
                value.from_100_000sats_to_1_000_000sats,
            ),
            from_1_000_000sats_to_10_000_000sats: (
                GroupFilter::Range(1_000_000..10_000_000),
                value.from_1_000_000sats_to_10_000_000sats,
            ),
            from_10_000_000sats_to_1btc: (
                GroupFilter::Range(10_000_000..1_00_000_000),
                value.from_10_000_000sats_to_1btc,
            ),
            from_1btc_to_10btc: (
                GroupFilter::Range(1_00_000_000..10_00_000_000),
                value.from_1btc_to_10btc,
            ),
            from_10btc_to_100btc: (
                GroupFilter::Range(10_00_000_000..100_00_000_000),
                value.from_10btc_to_100btc,
            ),
            from_100btc_to_1_000btc: (
                GroupFilter::Range(100_00_000_000..1_000_00_000_000),
                value.from_100btc_to_1_000btc,
            ),
            from_1_000btc_to_10_000btc: (
                GroupFilter::Range(1_000_00_000_000..10_000_00_000_000),
                value.from_1_000btc_to_10_000btc,
            ),
            from_10_000btc_to_100_000btc: (
                GroupFilter::Range(10_000_00_000_000..100_000_00_000_000),
                value.from_10_000btc_to_100_000btc,
            ),
            from_100_000btc: (GroupFilter::From(100_000_00_000_000), value.from_100_000btc),
        }
    }
}

impl<T> GroupedBySizeRange<T> {
    #[allow(clippy::inconsistent_digit_grouping)]
    pub fn get_mut(&mut self, value: Sats) -> &mut T {
        if value == Sats::ZERO {
            &mut self._0sats
        } else if value < Sats::_10 {
            &mut self.from_1sat_to_10sats
        } else if value < Sats::_100 {
            &mut self.from_10sats_to_100sats
        } else if value < Sats::_1K {
            &mut self.from_100sats_to_1_000sats
        } else if value < Sats::_10K {
            &mut self.from_1_000sats_to_10_000sats
        } else if value < Sats::_100K {
            &mut self.from_10_000sats_to_100_000sats
        } else if value < Sats::_1M {
            &mut self.from_100_000sats_to_1_000_000sats
        } else if value < Sats::_10M {
            &mut self.from_1_000_000sats_to_10_000_000sats
        } else if value < Sats::_1_BTC {
            &mut self.from_10_000_000sats_to_1btc
        } else if value < Sats::_10_BTC {
            &mut self.from_1btc_to_10btc
        } else if value < Sats::_100_BTC {
            &mut self.from_10btc_to_100btc
        } else if value < Sats::_1K_BTC {
            &mut self.from_100btc_to_1_000btc
        } else if value < Sats::_10K_BTC {
            &mut self.from_1_000btc_to_10_000btc
        } else if value < Sats::_100K_BTC {
            &mut self.from_10_000btc_to_100_000btc
        } else {
            &mut self.from_100_000btc
        }
    }

    pub fn as_vec(&self) -> [&T; 15] {
        [
            &self._0sats,
            &self.from_1sat_to_10sats,
            &self.from_10sats_to_100sats,
            &self.from_100sats_to_1_000sats,
            &self.from_1_000sats_to_10_000sats,
            &self.from_10_000sats_to_100_000sats,
            &self.from_100_000sats_to_1_000_000sats,
            &self.from_1_000_000sats_to_10_000_000sats,
            &self.from_10_000_000sats_to_1btc,
            &self.from_1btc_to_10btc,
            &self.from_10btc_to_100btc,
            &self.from_100btc_to_1_000btc,
            &self.from_1_000btc_to_10_000btc,
            &self.from_10_000btc_to_100_000btc,
            &self.from_100_000btc,
        ]
    }

    pub fn as_typed_vec(&self) -> [(Sats, &T); 15] {
        [
            (Sats::ZERO, &self._0sats),
            (Sats::_1, &self.from_1sat_to_10sats),
            (Sats::_10, &self.from_10sats_to_100sats),
            (Sats::_100, &self.from_100sats_to_1_000sats),
            (Sats::_1K, &self.from_1_000sats_to_10_000sats),
            (Sats::_10K, &self.from_10_000sats_to_100_000sats),
            (Sats::_100K, &self.from_100_000sats_to_1_000_000sats),
            (Sats::_1M, &self.from_1_000_000sats_to_10_000_000sats),
            (Sats::_10M, &self.from_10_000_000sats_to_1btc),
            (Sats::_1_BTC, &self.from_1btc_to_10btc),
            (Sats::_10_BTC, &self.from_10btc_to_100btc),
            (Sats::_100_BTC, &self.from_100btc_to_1_000btc),
            (Sats::_1K_BTC, &self.from_1_000btc_to_10_000btc),
            (Sats::_10K_BTC, &self.from_10_000btc_to_100_000btc),
            (Sats::_100K_BTC, &self.from_100_000btc),
        ]
    }

    pub fn as_mut_vec(&mut self) -> [&mut T; 15] {
        [
            &mut self._0sats,
            &mut self.from_1sat_to_10sats,
            &mut self.from_10sats_to_100sats,
            &mut self.from_100sats_to_1_000sats,
            &mut self.from_1_000sats_to_10_000sats,
            &mut self.from_10_000sats_to_100_000sats,
            &mut self.from_100_000sats_to_1_000_000sats,
            &mut self.from_1_000_000sats_to_10_000_000sats,
            &mut self.from_10_000_000sats_to_1btc,
            &mut self.from_1btc_to_10btc,
            &mut self.from_10btc_to_100btc,
            &mut self.from_100btc_to_1_000btc,
            &mut self.from_1_000btc_to_10_000btc,
            &mut self.from_10_000btc_to_100_000btc,
            &mut self.from_100_000btc,
        ]
    }
}

impl<T> GroupedBySizeRange<(GroupFilter, T)> {
    pub fn vecs(&self) -> [&T; 15] {
        [
            &self._0sats.1,
            &self.from_1sat_to_10sats.1,
            &self.from_10sats_to_100sats.1,
            &self.from_100sats_to_1_000sats.1,
            &self.from_1_000sats_to_10_000sats.1,
            &self.from_10_000sats_to_100_000sats.1,
            &self.from_100_000sats_to_1_000_000sats.1,
            &self.from_1_000_000sats_to_10_000_000sats.1,
            &self.from_10_000_000sats_to_1btc.1,
            &self.from_1btc_to_10btc.1,
            &self.from_10btc_to_100btc.1,
            &self.from_100btc_to_1_000btc.1,
            &self.from_1_000btc_to_10_000btc.1,
            &self.from_10_000btc_to_100_000btc.1,
            &self.from_100_000btc.1,
        ]
    }
}

impl<T> Add for GroupedBySizeRange<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            _0sats: self._0sats + rhs._0sats,
            from_1sat_to_10sats: self.from_1sat_to_10sats + rhs.from_1sat_to_10sats,
            from_10sats_to_100sats: self.from_10sats_to_100sats + rhs.from_10sats_to_100sats,
            from_100sats_to_1_000sats: self.from_100sats_to_1_000sats
                + rhs.from_100sats_to_1_000sats,
            from_1_000sats_to_10_000sats: self.from_1_000sats_to_10_000sats
                + rhs.from_1_000sats_to_10_000sats,
            from_10_000sats_to_100_000sats: self.from_10_000sats_to_100_000sats
                + rhs.from_10_000sats_to_100_000sats,
            from_100_000sats_to_1_000_000sats: self.from_100_000sats_to_1_000_000sats
                + rhs.from_100_000sats_to_1_000_000sats,
            from_1_000_000sats_to_10_000_000sats: self.from_1_000_000sats_to_10_000_000sats
                + rhs.from_1_000_000sats_to_10_000_000sats,
            from_10_000_000sats_to_1btc: self.from_10_000_000sats_to_1btc
                + rhs.from_10_000_000sats_to_1btc,
            from_1btc_to_10btc: self.from_1btc_to_10btc + rhs.from_1btc_to_10btc,
            from_10btc_to_100btc: self.from_10btc_to_100btc + rhs.from_10btc_to_100btc,
            from_100btc_to_1_000btc: self.from_100btc_to_1_000btc + rhs.from_100btc_to_1_000btc,
            from_1_000btc_to_10_000btc: self.from_1_000btc_to_10_000btc
                + rhs.from_1_000btc_to_10_000btc,
            from_10_000btc_to_100_000btc: self.from_10_000btc_to_100_000btc
                + rhs.from_10_000btc_to_100_000btc,
            from_100_000btc: self.from_100_000btc + rhs.from_100_000btc,
        }
    }
}

impl<T> AddAssign for GroupedBySizeRange<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self._0sats += rhs._0sats;
        self.from_1sat_to_10sats += rhs.from_1sat_to_10sats;
        self.from_10sats_to_100sats += rhs.from_10sats_to_100sats;
        self.from_100sats_to_1_000sats += rhs.from_100sats_to_1_000sats;
        self.from_1_000sats_to_10_000sats += rhs.from_1_000sats_to_10_000sats;
        self.from_10_000sats_to_100_000sats += rhs.from_10_000sats_to_100_000sats;
        self.from_100_000sats_to_1_000_000sats += rhs.from_100_000sats_to_1_000_000sats;
        self.from_1_000_000sats_to_10_000_000sats += rhs.from_1_000_000sats_to_10_000_000sats;
        self.from_10_000_000sats_to_1btc += rhs.from_10_000_000sats_to_1btc;
        self.from_1btc_to_10btc += rhs.from_1btc_to_10btc;
        self.from_10btc_to_100btc += rhs.from_10btc_to_100btc;
        self.from_100btc_to_1_000btc += rhs.from_100btc_to_1_000btc;
        self.from_1_000btc_to_10_000btc += rhs.from_1_000btc_to_10_000btc;
        self.from_10_000btc_to_100_000btc += rhs.from_10_000btc_to_100_000btc;
        self.from_100_000btc += rhs.from_100_000btc;
    }
}
