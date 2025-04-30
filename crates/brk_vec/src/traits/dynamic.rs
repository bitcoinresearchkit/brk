use std::path::Path;

use arc_swap::ArcSwap;
use memmap2::Mmap;

use crate::{Result, Value};

use super::{StoredIndex, StoredType};

pub trait DynamicVec: Send + Sync {
    type I: StoredIndex;
    type T: StoredType;
    const SIZE_OF_T: usize = size_of::<Self::T>();

    #[inline]
    fn read(&self, index: Self::I, mmap: &Mmap) -> Result<Option<Self::T>> {
        self.read_(index.to_usize()?, mmap)
    }
    fn read_(&self, index: usize, mmap: &Mmap) -> Result<Option<Self::T>>;

    #[inline]
    fn get_or_read(&self, index: Self::I, mmap: &Mmap) -> Result<Option<Value<Self::T>>> {
        self.get_or_read_(index.to_usize()?, mmap)
    }
    #[inline]
    fn get_or_read_(&self, index: usize, mmap: &Mmap) -> Result<Option<Value<Self::T>>> {
        let stored_len = mmap.len() / Self::SIZE_OF_T;

        if index >= stored_len {
            let pushed = self.pushed();
            let j = index - stored_len;
            if j >= pushed.len() {
                return Ok(None);
            }
            Ok(pushed.get(j).map(Value::Ref))
        } else {
            Ok(self.read_(index, mmap)?.map(Value::Owned))
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.stored_len() + self.pushed_len()
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn mmap(&self) -> &ArcSwap<Mmap>;

    fn stored_len(&self) -> usize;

    fn pushed(&self) -> &[Self::T];
    #[inline]
    fn pushed_len(&self) -> usize {
        self.pushed().len()
    }
    fn mut_pushed(&mut self) -> &mut Vec<Self::T>;
    #[inline]
    fn push(&mut self, value: Self::T) {
        self.mut_pushed().push(value)
    }

    fn path(&self) -> &Path;
}
