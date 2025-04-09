use crate::{Result, Value};

use super::{StoredIndex, StoredType};

pub trait DynamicVec: Send + Sync {
    type I: StoredIndex;
    type T: StoredType;

    #[inline]
    fn get(&self, index: Self::I) -> Result<Option<Value<Self::T>>> {
        self.get_(index.to_usize()?)
    }
    fn get_(&self, index: usize) -> Result<Option<Value<Self::T>>>;
    fn get_last(&self) -> Result<Option<Value<Self::T>>> {
        let len = self.len();
        if len == 0 {
            return Ok(None);
        }
        self.get_(len - 1)
    }

    #[inline]
    fn len(&self) -> usize {
        self.stored_len() + self.pushed_len()
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

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
}
