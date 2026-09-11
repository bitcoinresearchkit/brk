use std::{
    mem,
    sync::atomic::{AtomicBool, AtomicU64},
};

use parking_lot::{Mutex, RwLock};

use super::{CacheBudget, Table, budget::Reclaim};
use crate::VecValue;

#[derive(Debug)]
pub(super) struct Shared<T> {
    pub(super) gate: RwLock<()>,
    pub(super) fill: Mutex<()>,
    pub(super) table: RwLock<Table<T>>,
    pub(super) budget: &'static CacheBudget,
    pub(super) healthy: AtomicBool,
    pub(super) revision: AtomicU64,
}

impl<T> Shared<T> {
    pub(super) fn new(budget: &'static CacheBudget) -> Self {
        Self {
            gate: RwLock::new(()),
            fill: Mutex::new(()),
            table: RwLock::new(Table::new()),
            budget,
            healthy: AtomicBool::new(true),
            revision: AtomicU64::new(0),
        }
    }

    fn clear_entries(&self) {
        let old = mem::replace(&mut *self.table.write(), Table::new());
        drop(old);
    }
}

impl<T: VecValue> Reclaim for Shared<T> {
    fn evict_one(&self) -> bool {
        let Some(mut table) = self.table.try_write() else {
            return false;
        };
        let key = table
            .range(table.hand..)
            .next()
            .or_else(|| table.first_key_value())
            .map(|(&key, _)| key);
        let Some(key) = key else { return false };
        let removed = table.remove(&key);
        table.hand = key.checked_add(1).unwrap_or(0);
        table.shrink_charge();
        drop(table);
        drop(removed);
        true
    }

    fn clear(&self) {
        self.clear_entries();
    }
}
