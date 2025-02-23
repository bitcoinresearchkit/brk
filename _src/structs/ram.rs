use derive_deref::Deref;
use memory_stats::memory_stats;
use sysinfo::System;

use crate::structs::Config;

#[allow(clippy::upper_case_acronyms)]
#[derive(Deref)]
pub struct RAM(System);

impl RAM {
    pub fn new() -> Self {
        Self(System::new_all())
    }

    pub fn max_exceeded(&self, config: &Config) -> bool {
        let ram_used = memory_stats().unwrap().physical_mem as f64;

        if let Some(max_ram) = config.max_ram {
            (ram_used / 1_000_000_000.0) > max_ram
        } else {
            let ram_total = self.total_memory() as f64;
            ram_used / ram_total > 0.5
        }
    }
}
