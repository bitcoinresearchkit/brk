use std::ops::{Add, AddAssign};

#[derive(Default, Clone, Debug)]
pub struct GroupedByUnspendableType<T> {
    pub opreturn: T,
}

impl<T> GroupedByUnspendableType<T> {
    pub fn as_vec(&self) -> [&T; 1] {
        [&self.opreturn]
    }
}

impl<T> Add for GroupedByUnspendableType<T>
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

impl<T> AddAssign for GroupedByUnspendableType<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.opreturn += rhs.opreturn;
    }
}
