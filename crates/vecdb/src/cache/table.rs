use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use super::{Charge, Value};

#[derive(Debug)]
pub(super) struct Table<T> {
    entries: BTreeMap<usize, Value<T>>,
    pub(super) charge: Option<Charge>,
    pub(super) hand: usize,
}

impl<T> Table<T> {
    pub(super) fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            charge: None,
            hand: 0,
        }
    }

    // One conservative formula for std's tree metadata, including inline values.
    // This is a budget allowance, not allocator/RSS instrumentation.
    pub(super) fn entry_bytes() -> usize {
        2 * size_of::<(usize, Value<T>)>() + 32
    }

    pub(super) fn root_bytes() -> usize {
        16 * size_of::<(usize, Value<T>)>() + 128
    }

    pub(super) fn charge_for(len: usize) -> usize {
        if len == 0 {
            0
        } else {
            Self::root_bytes()
                .checked_add(
                    Self::entry_bytes()
                        .checked_mul(len)
                        .expect("cache directory overflow"),
                )
                .expect("cache directory overflow")
        }
    }

    pub(super) fn shrink_charge(&mut self) {
        if self.entries.is_empty() {
            self.entries = BTreeMap::new();
            self.charge = None;
            self.hand = 0;
        } else if let Some(charge) = &mut self.charge {
            charge.shrink_to(Self::charge_for(self.entries.len()));
        }
    }

    pub(super) fn add_charge(&mut self, charge: Charge) {
        if let Some(current) = &mut self.charge {
            current.merge(charge);
        } else {
            self.charge = Some(charge);
        }
        self.shrink_charge();
    }
}

impl<T> Deref for Table<T> {
    type Target = BTreeMap<usize, Value<T>>;
    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<T> DerefMut for Table<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}
