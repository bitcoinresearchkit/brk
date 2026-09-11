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
}

impl<T: VecValue> Reclaim for Shared<T> {
    fn try_clear(&self) {
        let old = self
            .table
            .try_write()
            .map(|mut table| mem::replace(&mut *table, Table::new()));
        drop(old);
    }

    fn clear(&self) {
        let old = mem::replace(&mut *self.table.write(), Table::new());
        drop(old);
    }
}
