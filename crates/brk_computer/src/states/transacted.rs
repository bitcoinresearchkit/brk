use std::ops::{Add, AddAssign};

use brk_structs::{ByAmountRange, GroupedByType, OutputType, Sats};

use super::SupplyState;

#[derive(Default, Debug)]
pub struct Transacted {
    pub spendable_supply: SupplyState,
    pub by_type: GroupedByType<SupplyState>,
    pub by_size_group: ByAmountRange<SupplyState>,
}

impl Transacted {
    #[allow(clippy::inconsistent_digit_grouping)]
    pub fn iterate(&mut self, value: Sats, _type: OutputType) {
        let supply = SupplyState {
            utxo_count: 1,
            value,
        };

        *self.by_type.get_mut(_type) += &supply;

        if _type.is_unspendable() {
            return;
        }

        self.spendable_supply += &supply;

        *self.by_size_group.get_mut(value) += &supply;
    }
}

impl Add for Transacted {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            spendable_supply: self.spendable_supply + rhs.spendable_supply,
            by_type: self.by_type + rhs.by_type,
            by_size_group: self.by_size_group + rhs.by_size_group,
        }
    }
}

impl AddAssign for Transacted {
    fn add_assign(&mut self, rhs: Self) {
        self.by_size_group += rhs.by_size_group;
        self.spendable_supply += &rhs.spendable_supply;
        self.by_type += rhs.by_type;
    }
}
