use std::io;

use crate::StorableVec;

use super::{StorableVecIndex, StorableVecType};

pub trait AnyStorableVec {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn flush(&mut self) -> io::Result<()>;
}

impl<I, T, const MODE: u8> AnyStorableVec for StorableVec<I, T, MODE>
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

    fn flush(&mut self) -> io::Result<()> {
        self.flush()
    }
}
