use std::thread;

mod _trait;
mod cohorts_states;
mod counters;
mod date_data_vec;

pub use _trait::*;

use allocative::Allocative;
pub use cohorts_states::*;
use counters::*;
use date_data_vec::*;

use crate::utils::log;

#[derive(Default, Allocative)]
pub struct States {
    pub address_counters: Counters,
    pub date_data_vec: DateDataVec,
    pub address_cohorts_durable_states: Option<AddressCohortsDurableStates>,
    pub utxo_cohorts_durable_states: Option<UTXOCohortsDurableStates>,
}

impl States {
    pub fn import() -> color_eyre::Result<Self> {
        let date_data_vec_handle = thread::spawn(DateDataVec::import);

        let address_counters = Counters::import()?;

        let date_data_vec = date_data_vec_handle.join().unwrap()?;

        Ok(Self {
            address_cohorts_durable_states: None,
            address_counters,
            date_data_vec,
            utxo_cohorts_durable_states: None,
        })
    }

    pub fn reset(&mut self, include_addresses: bool) {
        log("Reseting all states...");

        let _ = self.date_data_vec.reset();

        self.utxo_cohorts_durable_states = None;

        if include_addresses {
            let _ = self.address_counters.reset();

            self.address_cohorts_durable_states = None;
        }
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        thread::scope(|s| {
            s.spawn(|| self.address_counters.export().unwrap());
            s.spawn(|| self.date_data_vec.export().unwrap());
        });

        Ok(())
    }
}
