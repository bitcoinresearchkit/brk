use std::ops::{Add, AddAssign};

use brk_traversable::Traversable;

#[derive(Default, Clone, Debug, Traversable)]
pub struct ByUnspendableType<T> {
    pub opreturn: T,
}

impl<T> ByUnspendableType<T> {
    pub fn as_vec(&self) -> [&T; 1] {
        [&self.opreturn]
    }
}

impl<T> Add for ByUnspendableType<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            opreturn: self.opreturn + rhs.opreturn,
        }
    }
}

impl<T> AddAssign for ByUnspendableType<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.opreturn += rhs.opreturn;
    }
}
