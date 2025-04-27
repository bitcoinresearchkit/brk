use std::{path::Path, sync::Arc};

use arc_swap::{ArcSwap, Guard};
use memmap2::Mmap;

use crate::{Error, Result, Value};

use super::{StoredIndex, StoredType};

pub trait DynamicVec: Send + Sync {
    type I: StoredIndex;
    type T: StoredType;

    #[inline]
    fn get(&self, index: Self::I) -> Result<Option<Value<Self::T>>> {
        self.get_(index.to_usize()?)
    }
    #[inline]
    fn cached_get(&mut self, index: Self::I) -> Result<Option<Value<Self::T>>> {
        self.cached_get_(index.to_usize()?)
    }
    #[inline]
    fn unwrap_cached_get(&mut self, index: Self::I) -> Option<Self::T> {
        self.cached_get(index).unwrap().map(Value::into_inner)
    }
    #[inline]
    fn double_unwrap_cached_get(&mut self, index: Self::I) -> Self::T {
        self.unwrap_cached_get(index).unwrap()
    }
    #[inline]
    fn get_(&self, index: usize) -> Result<Option<Value<Self::T>>> {
        match self.index_to_pushed_index(index) {
            Ok(index) => {
                if let Some(index) = index {
                    return Ok(self.pushed().get(index).map(Value::Ref));
                }
            }
            Err(Error::IndexTooHigh) => return Ok(None),
            Err(Error::IndexTooLow) => {}
            Err(error) => return Err(error),
        }

        Ok(self
            .get_stored_(index.to_usize()?, self.guard().as_ref().unwrap())?
            .map(Value::Owned))
    }
    fn get_stored_(&self, index: usize, mmap: &Mmap) -> Result<Option<Self::T>>;
    fn last(&self) -> Result<Option<Value<Self::T>>> {
        let len = self.len();
        if len == 0 {
            return Ok(None);
        }
        self.get_(len - 1)
    }
    #[inline]
    fn cached_get_(&mut self, index: usize) -> Result<Option<Value<Self::T>>> {
        match self.index_to_pushed_index(index) {
            Ok(index) => {
                if let Some(index) = index {
                    return Ok(self.pushed().get(index).map(Value::Ref));
                }
            }
            Err(Error::IndexTooHigh) => return Ok(None),
            Err(Error::IndexTooLow) => {}
            Err(error) => return Err(error),
        }

        let mmap = Arc::clone(self.guard().as_ref().unwrap());

        Ok(self
            .cached_get_stored_(index.to_usize()?, &mmap)?
            .map(Value::Owned))
    }
    fn cached_get_stored_(&mut self, index: usize, mmap: &Mmap) -> Result<Option<Self::T>>;
    fn cached_get_last(&mut self) -> Result<Option<Value<Self::T>>> {
        let len = self.len();
        if len == 0 {
            return Ok(None);
        }
        self.cached_get_(len - 1)
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

    fn guard(&self) -> &Option<Guard<Arc<Mmap>>>;
    fn mut_guard(&mut self) -> &mut Option<Guard<Arc<Mmap>>>;

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
    #[inline]
    fn index_to_pushed_index(&self, index: usize) -> Result<Option<usize>> {
        let stored_len = self.stored_len();

        if index >= stored_len {
            let index = index - stored_len;
            if index >= self.pushed_len() {
                Err(Error::IndexTooHigh)
            } else {
                Ok(Some(index))
            }
        } else {
            Err(Error::IndexTooLow)
        }
    }

    fn path(&self) -> &Path;
}
