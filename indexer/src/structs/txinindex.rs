use std::ops::{Add, AddAssign};

use derive_deref::{Deref, DerefMut};

use super::Vout;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, DerefMut, Default)]
pub struct Txinindex(u64);

impl Txinindex {
    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }

    pub fn decremented(self) -> Self {
        Self(*self - 1)
    }
}

impl Add<Txinindex> for Txinindex {
    type Output = Self;
    fn add(self, rhs: Txinindex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Vout> for Txinindex {
    type Output = Self;
    fn add(self, rhs: Vout) -> Self::Output {
        Self(self.0 + u64::from(rhs))
    }
}

impl AddAssign<Txinindex> for Txinindex {
    fn add_assign(&mut self, rhs: Txinindex) {
        self.0 += rhs.0
    }
}

impl From<u64> for Txinindex {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<Txinindex> for u64 {
    fn from(value: Txinindex) -> Self {
        value.0
    }
}

impl From<usize> for Txinindex {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}
impl From<Txinindex> for usize {
    fn from(value: Txinindex) -> Self {
        value.0 as usize
    }
}
