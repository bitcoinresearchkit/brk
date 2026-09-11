use std::{slice, sync::Arc};

use super::{Buffer, CacheBudget};

/// No allocation or reference count for isolated cached values.
#[derive(Clone, Debug)]
pub(super) enum Value<T> {
    One(T),
    Many(Arc<Buffer<T>>),
}

impl<T: Clone> Value<T> {
    pub(super) fn from_vec(mut values: Vec<T>) -> Self {
        assert!(!values.is_empty());
        if values.len() == 1 {
            Self::One(values.pop().unwrap())
        } else {
            Self::Many(Arc::new(Buffer::new(values)))
        }
    }

    pub(super) fn slice(&self) -> &[T] {
        match self {
            Self::One(value) => slice::from_ref(value),
            Self::Many(buffer) => &buffer.values,
        }
    }

    pub(super) fn len(&self) -> usize {
        self.slice().len()
    }

    pub(super) fn get(&self, index: usize) -> Option<T> {
        match self {
            Self::One(value) => (index == 0).then(|| value.clone()),
            Self::Many(buffer) => buffer.values.get(index).cloned(),
        }
    }

    pub(super) fn clipped(&self, from: usize, to: usize) -> Self {
        Self::from_vec(self.slice()[from..to].to_vec())
    }

    pub(super) fn allocation_bytes(&self) -> Option<usize> {
        match self {
            Self::One(_) => Some(0),
            Self::Many(buffer) => buffer.bytes(),
        }
    }

    pub(super) fn admit(&mut self, budget: &'static CacheBudget) -> bool {
        match self {
            Self::One(_) => true,
            Self::Many(buffer) if buffer.is_charged() => true,
            Self::Many(buffer) => Arc::get_mut(buffer).is_some_and(|buffer| buffer.admit(budget)),
        }
    }

    pub(super) fn mutable(&mut self) -> &mut Buffer<T> {
        if !matches!(self, Self::Many(buffer) if Arc::strong_count(buffer) == 1) {
            *self = Self::Many(Arc::new(Buffer::new(self.slice().to_vec())));
        }
        match self {
            Self::Many(buffer) => Arc::get_mut(buffer).expect("cache buffer is privately owned"),
            Self::One(_) => unreachable!(),
        }
    }

    pub(super) fn truncate(mut self, len: usize) -> Self {
        assert!(len > 0 && len <= self.len());
        if len == self.len() {
            return self;
        }
        if len == 1 {
            return Self::One(self.slice()[0].clone());
        }
        if let Self::Many(buffer) = &mut self
            && let Some(buffer) = Arc::get_mut(buffer)
        {
            buffer.truncate(len);
            return self;
        }
        self.clipped(0, len)
    }
}
