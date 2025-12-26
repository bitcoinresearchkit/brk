use std::ops::{Add, AddAssign};

use brk_traversable::Traversable;
use brk_types::Sats;
use rayon::prelude::*;

use super::{AmountFilter, Filter};

/// Bucket index for amount ranges. Use for cheap comparisons and direct lookups.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AmountBucket(u8);

impl AmountBucket {
    /// Returns (self, other) if buckets differ, None if same.
    /// Use with `ByAmountRange::get_mut_by_bucket` to avoid recomputing.
    #[inline(always)]
    pub fn transition_to(self, other: Self) -> Option<(Self, Self)> {
        if self != other {
            Some((self, other))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn index(self) -> u8 {
        self.0
    }
}

impl From<Sats> for AmountBucket {
    #[inline(always)]
    fn from(value: Sats) -> Self {
        Self(match value {
            v if v < Sats::_1 => 0,
            v if v < Sats::_10 => 1,
            v if v < Sats::_100 => 2,
            v if v < Sats::_1K => 3,
            v if v < Sats::_10K => 4,
            v if v < Sats::_100K => 5,
            v if v < Sats::_1M => 6,
            v if v < Sats::_10M => 7,
            v if v < Sats::_1BTC => 8,
            v if v < Sats::_10BTC => 9,
            v if v < Sats::_100BTC => 10,
            v if v < Sats::_1K_BTC => 11,
            v if v < Sats::_10K_BTC => 12,
            v if v < Sats::_100K_BTC => 13,
            _ => 14,
        })
    }
}

/// Check if two amounts are in different buckets. O(1).
#[inline(always)]
pub fn amounts_in_different_buckets(a: Sats, b: Sats) -> bool {
    AmountBucket::from(a) != AmountBucket::from(b)
}

#[derive(Debug, Default, Clone, Traversable)]
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

impl<T> ByAmountRange<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter) -> T,
    {
        Self {
            _0sats: create(Filter::Amount(AmountFilter::Range(Sats::ZERO..Sats::_1))),
            _1sat_to_10sats: create(Filter::Amount(AmountFilter::Range(Sats::_1..Sats::_10))),
            _10sats_to_100sats: create(Filter::Amount(AmountFilter::Range(Sats::_10..Sats::_100))),
            _100sats_to_1k_sats: create(Filter::Amount(AmountFilter::Range(Sats::_100..Sats::_1K))),
            _1k_sats_to_10k_sats: create(Filter::Amount(AmountFilter::Range(
                Sats::_1K..Sats::_10K,
            ))),
            _10k_sats_to_100k_sats: create(Filter::Amount(AmountFilter::Range(
                Sats::_10K..Sats::_100K,
            ))),
            _100k_sats_to_1m_sats: create(Filter::Amount(AmountFilter::Range(
                Sats::_100K..Sats::_1M,
            ))),
            _1m_sats_to_10m_sats: create(Filter::Amount(AmountFilter::Range(
                Sats::_1M..Sats::_10M,
            ))),
            _10m_sats_to_1btc: create(Filter::Amount(AmountFilter::Range(Sats::_10M..Sats::_1BTC))),
            _1btc_to_10btc: create(Filter::Amount(AmountFilter::Range(
                Sats::_1BTC..Sats::_10BTC,
            ))),
            _10btc_to_100btc: create(Filter::Amount(AmountFilter::Range(
                Sats::_10BTC..Sats::_100BTC,
            ))),
            _100btc_to_1k_btc: create(Filter::Amount(AmountFilter::Range(
                Sats::_100BTC..Sats::_1K_BTC,
            ))),
            _1k_btc_to_10k_btc: create(Filter::Amount(AmountFilter::Range(
                Sats::_1K_BTC..Sats::_10K_BTC,
            ))),
            _10k_btc_to_100k_btc: create(Filter::Amount(AmountFilter::Range(
                Sats::_10K_BTC..Sats::_100K_BTC,
            ))),
            _100k_btc_or_more: create(Filter::Amount(AmountFilter::Range(
                Sats::_100K_BTC..Sats::MAX,
            ))),
        }
    }

    #[inline(always)]
    pub fn get(&self, value: Sats) -> &T {
        match AmountBucket::from(value).0 {
            0 => &self._0sats,
            1 => &self._1sat_to_10sats,
            2 => &self._10sats_to_100sats,
            3 => &self._100sats_to_1k_sats,
            4 => &self._1k_sats_to_10k_sats,
            5 => &self._10k_sats_to_100k_sats,
            6 => &self._100k_sats_to_1m_sats,
            7 => &self._1m_sats_to_10m_sats,
            8 => &self._10m_sats_to_1btc,
            9 => &self._1btc_to_10btc,
            10 => &self._10btc_to_100btc,
            11 => &self._100btc_to_1k_btc,
            12 => &self._1k_btc_to_10k_btc,
            13 => &self._10k_btc_to_100k_btc,
            _ => &self._100k_btc_or_more,
        }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, value: Sats) -> &mut T {
        self.get_mut_by_bucket(AmountBucket::from(value))
    }

    /// Get mutable reference by pre-computed bucket index.
    /// Use with `AmountBucket::transition_to` to avoid recomputing bucket.
    #[inline(always)]
    pub fn get_mut_by_bucket(&mut self, bucket: AmountBucket) -> &mut T {
        match bucket.0 {
            0 => &mut self._0sats,
            1 => &mut self._1sat_to_10sats,
            2 => &mut self._10sats_to_100sats,
            3 => &mut self._100sats_to_1k_sats,
            4 => &mut self._1k_sats_to_10k_sats,
            5 => &mut self._10k_sats_to_100k_sats,
            6 => &mut self._100k_sats_to_1m_sats,
            7 => &mut self._1m_sats_to_10m_sats,
            8 => &mut self._10m_sats_to_1btc,
            9 => &mut self._1btc_to_10btc,
            10 => &mut self._10btc_to_100btc,
            11 => &mut self._100btc_to_1k_btc,
            12 => &mut self._1k_btc_to_10k_btc,
            13 => &mut self._10k_btc_to_100k_btc,
            _ => &mut self._100k_btc_or_more,
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

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
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
        .into_par_iter()
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
