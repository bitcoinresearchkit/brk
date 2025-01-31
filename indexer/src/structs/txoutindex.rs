use std::ops::{Add, AddAssign};

use derive_deref::{Deref, DerefMut};

use super::Vout;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, DerefMut, Default)]
pub struct Txoutindex(u64);

impl Txoutindex {
    pub const COINBASE: Self = Self(u64::MAX);

    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }

    pub fn decremented(self) -> Self {
        Self(*self - 1)
    }

    pub fn is_coinbase(self) -> bool {
        self == Self::COINBASE
    }
}

impl Add<Txoutindex> for Txoutindex {
    type Output = Self;
    fn add(self, rhs: Txoutindex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Vout> for Txoutindex {
    type Output = Self;
    fn add(self, rhs: Vout) -> Self::Output {
        Self(self.0 + u64::from(rhs))
    }
}

impl AddAssign<Txoutindex> for Txoutindex {
    fn add_assign(&mut self, rhs: Txoutindex) {
        self.0 += rhs.0
    }
}

impl From<u64> for Txoutindex {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<Txoutindex> for u64 {
    fn from(value: Txoutindex) -> Self {
        value.0
    }
}

impl From<usize> for Txoutindex {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}
impl From<Txoutindex> for usize {
    fn from(value: Txoutindex) -> Self {
        value.0 as usize
    }
}
