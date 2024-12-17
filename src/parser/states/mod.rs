use std::{fs, thread};

mod _trait;
mod cohorts_states;
mod counters;
mod date_data_vec;

pub use _trait::*;

use allocative::Allocative;
pub use cohorts_states::*;
use counters::*;
use date_data_vec::*;
use log::info;

use crate::structs::Config;

#[derive(Default, Allocative)]
pub struct States {
    pub address_counters: Counters,
    pub date_data_vec: DateDataVec,
    pub address_cohorts_durable_states: Option<AddressCohortsDurableStates>,
    pub utxo_cohorts_durable_states: Option<UTXOCohortsDurableStates>,
}

impl States {
    pub fn import(config: &Config) -> color_eyre::Result<Self> {
        fs::create_dir_all(config.path_states())?;

        let date_data_vec = DateDataVec::import(config)?;

        let address_counters = Counters::import(config)?;

        Ok(Self {
            address_cohorts_durable_states: None,
            address_counters,
            date_data_vec,
            utxo_cohorts_durable_states: None,
        })
    }

    pub fn reset(&mut self, config: &Config, include_addresses: bool) {
        info!("Reseting all states...");

        let _ = self.date_data_vec.reset(config);

        self.utxo_cohorts_durable_states = None;

        if include_addresses {
            let _ = self.address_counters.reset(config);

            self.address_cohorts_durable_states = None;
        }
    }

    pub fn export(&self, config: &Config) -> color_eyre::Result<()> {
        thread::scope(|s| {
            s.spawn(|| self.address_counters.export(config).unwrap());
            s.spawn(|| self.date_data_vec.export(config).unwrap());
        });

        Ok(())
    }
}
