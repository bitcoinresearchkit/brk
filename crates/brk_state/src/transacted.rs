use std::{
    collections::BTreeMap,
    mem,
    ops::{Add, AddAssign},
};

use brk_core::{OutputType, Sats};

use super::{OutputsByType, SupplyState};

#[derive(Default)]
pub struct Transacted {
    pub spendable_supply: SupplyState,
    pub by_type: OutputsByType<SupplyState>,
    pub by_size_group: BTreeMap<usize, SupplyState>,
}

impl Transacted {
    #[allow(clippy::inconsistent_digit_grouping)]
    pub fn iterate(&mut self, value: Sats, _type: OutputType) {
        let supply = SupplyState { utxos: 1, value };

        *self.by_type.get_mut(_type) += &supply;

        if _type.is_unspendable() {
            return;
        }

        self.spendable_supply += &supply;

        let _value = usize::from(value);

        // Need to be in sync with by_size !! but plenty fast (I think)
        if _value == 0 {
            *self.by_size_group.entry(0).or_default() += &supply;
        } else if _value < 10 {
            *self.by_size_group.entry(1).or_default() += &supply;
        } else if _value < 100 {
            *self.by_size_group.entry(10).or_default() += &supply;
        } else if _value < 1_000 {
            *self.by_size_group.entry(100).or_default() += &supply;
        } else if _value < 10_000 {
            *self.by_size_group.entry(1_000).or_default() += &supply;
        } else if _value < 100_000 {
            *self.by_size_group.entry(10_000).or_default() += &supply;
        } else if _value < 1_000_000 {
            *self.by_size_group.entry(100_000).or_default() += &supply;
        } else if _value < 10_000_000 {
            *self.by_size_group.entry(1_000_000).or_default() += &supply;
        } else if _value < 1_00_000_000 {
            *self.by_size_group.entry(10_000_000).or_default() += &supply;
        } else if _value < 10_00_000_000 {
            *self.by_size_group.entry(1_00_000_000).or_default() += &supply;
        } else if _value < 100_00_000_000 {
            *self.by_size_group.entry(10_00_000_000).or_default() += &supply;
        } else if _value < 1_000_00_000_000 {
            *self.by_size_group.entry(100_00_000_000).or_default() += &supply;
        } else if _value < 10_000_00_000_000 {
            *self.by_size_group.entry(1_000_00_000_000).or_default() += &supply;
        } else if _value < 100_000_00_000_000 {
            *self.by_size_group.entry(10_000_00_000_000).or_default() += &supply;
        } else {
            *self.by_size_group.entry(100_000_00_000_000).or_default() += &supply;
        }
    }

    fn merge_by_size(
        first: BTreeMap<usize, SupplyState>,
        second: BTreeMap<usize, SupplyState>,
    ) -> BTreeMap<usize, SupplyState> {
        let (mut source, to_consume) = if first.len() > second.len() {
            (first, second)
        } else {
            (second, first)
        };
        to_consume.into_iter().for_each(|(k, v)| {
            *source.entry(k).or_default() += &v;
        });
        source
    }
}

impl Add for Transacted {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            spendable_supply: self.spendable_supply + rhs.spendable_supply,
            by_type: self.by_type + rhs.by_type,
            by_size_group: Self::merge_by_size(self.by_size_group, rhs.by_size_group),
        }
    }
}

impl AddAssign for Transacted {
    fn add_assign(&mut self, rhs: Self) {
        self.by_size_group =
            Self::merge_by_size(mem::take(&mut self.by_size_group), rhs.by_size_group);
        self.spendable_supply += &rhs.spendable_supply;
        self.by_type += rhs.by_type;
    }
}
