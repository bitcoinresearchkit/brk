use std::ops::{Add, AddAssign};

#[derive(Default, Clone)]
pub struct OutputsByUnspendableType<T> {
    pub op_return: T,
}

impl<T> OutputsByUnspendableType<T> {
    pub fn as_vec(&self) -> [&T; 1] {
        [&self.op_return]
    }
}

impl<T> Add for OutputsByUnspendableType<T>
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

impl<T> AddAssign for OutputsByUnspendableType<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.op_return += rhs.op_return;
    }
}
