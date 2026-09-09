use std::ops::{Add, AddAssign};

#[cfg(feature = "storage")]
use bitview_traversable::Traversable;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct UnspendableType<T> {
    /// Uses outputs whose locking script begins with `OP_RETURN`.
    pub op_return: T,
}

impl<T> Add for UnspendableType<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            op_return: self.op_return + rhs.op_return,
        }
    }
}

impl<T> AddAssign for UnspendableType<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.op_return += rhs.op_return;
    }
}
