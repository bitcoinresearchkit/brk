use super::{CacheBudget, Charge};

#[derive(Debug)]
pub(super) struct Buffer<T> {
    pub(super) values: Vec<T>,
    // Field order releases the values before their budget reservation.
    charge: Option<Charge>,
}

impl<T> Buffer<T> {
    pub(super) fn new(values: Vec<T>) -> Self {
        Self {
            values,
            charge: None,
        }
    }

    pub(super) fn bytes(&self) -> Option<usize> {
        self.values
            .capacity()
            .checked_mul(size_of::<T>())?
            .checked_add(size_of::<Self>() + 2 * size_of::<usize>())
    }

    pub(super) fn admit(&mut self, budget: &'static CacheBudget) -> bool {
        if self.charge.is_none() {
            let Some(bytes) = self.bytes() else {
                return false;
            };
            self.charge = budget.reserve(bytes);
        }
        self.charge.is_some()
    }

    pub(super) fn is_charged(&self) -> bool {
        self.charge.is_some()
    }

    pub(super) fn reserve(&mut self, len: usize, limit: usize) -> bool {
        if len <= self.values.capacity() {
            return true;
        }
        let capacity = len
            .checked_next_power_of_two()
            .unwrap_or(len)
            .min(limit.max(len));
        let Some(extra) = (capacity - self.values.capacity()).checked_mul(size_of::<T>()) else {
            return false;
        };
        if let Some(charge) = &mut self.charge
            && !charge.grow(extra)
        {
            return false;
        }
        self.values.reserve_exact(capacity - self.values.len());
        debug_assert_eq!(self.values.capacity(), capacity);
        true
    }

    pub(super) fn truncate(&mut self, len: usize) {
        self.values.truncate(len);
        if len < self.values.capacity().div_ceil(4) {
            self.values.shrink_to_fit();
            let bytes = self.bytes().expect("cache buffer size overflow");
            if let Some(charge) = &mut self.charge {
                charge.shrink_to(bytes);
            }
        }
    }
}
