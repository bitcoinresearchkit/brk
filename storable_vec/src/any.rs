use std::io;

use crate::{StorableVec, StorableVecIndex, StorableVecType};

pub trait AnyStorableVec {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn unsafe_flush(&mut self) -> io::Result<()>;
}

impl<I, T> AnyStorableVec for StorableVec<I, T>
where
    I: StorableVecIndex,
    T: StorableVecType,
{
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn unsafe_flush(&mut self) -> io::Result<()> {
        self.flush()
    }
}
