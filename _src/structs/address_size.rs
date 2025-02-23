use allocative::Allocative;

use super::Amount;

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Allocative)]
pub enum AddressSize {
    Empty,
    Plankton,
    Shrimp,
    Crab,
    Fish,
    Shark,
    Whale,
    Humpback,
    Megalodon,
}

impl AddressSize {
    pub fn from_amount(amount: Amount) -> Self {
        match amount.to_sat() {
            0 => Self::Empty,
            1..=9_999_999 => Self::Plankton,
            10_000_000..=99_999_999 => Self::Shrimp,
            100_000_000..=999_999_999 => Self::Crab,
            1_000_000_000..=9_999_999_999 => Self::Fish,
            10_000_000_000..=99_999_999_999 => Self::Shark,
            100_000_000_000..=999_999_999_999 => Self::Whale,
            1_000_000_000_000..=9_999_999_999_999 => Self::Humpback,
            10_000_000_000_000..=u64::MAX => Self::Megalodon,
        }
    }
}
