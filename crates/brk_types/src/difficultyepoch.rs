use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div},
};

use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

use super::Height;

pub const BLOCKS_PER_DIFF_EPOCHS: u32 = 2016;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize, Pco,
)]
pub struct DifficultyEpoch(u16);

impl From<u16> for DifficultyEpoch {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<usize> for DifficultyEpoch {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<DifficultyEpoch> for usize {
    #[inline]
    fn from(value: DifficultyEpoch) -> Self {
        value.0 as usize
    }
}

impl Add for DifficultyEpoch {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl AddAssign for DifficultyEpoch {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Add<usize> for DifficultyEpoch {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl Div<usize> for DifficultyEpoch {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self::from(self.0 as usize / rhs)
    }
}

impl From<Height> for DifficultyEpoch {
    #[inline]
    fn from(value: Height) -> Self {
        Self((u32::from(value) / BLOCKS_PER_DIFF_EPOCHS) as u16)
    }
}

impl CheckedSub for DifficultyEpoch {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl PrintableIndex for DifficultyEpoch {
    fn to_string() -> &'static str {
        "difficultyepoch"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["difficulty", "difficultyepoch"]
    }
}

impl std::fmt::Display for DifficultyEpoch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for DifficultyEpoch {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
