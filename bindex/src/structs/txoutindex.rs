use std::ops::{Add, AddAssign};

use derive_deref::{Deref, DerefMut};
use snkrj::{direct_repr, Storable, UnsizedStorable};

use super::Vout;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, DerefMut, Default)]
pub struct Txoutindex(u64);
direct_repr!(Txoutindex);

impl Txoutindex {
    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }

    pub fn decremented(self) -> Self {
        Self(*self - 1)
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
