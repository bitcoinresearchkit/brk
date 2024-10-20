mod all_metadata;
mod cohort;
mod cohort_metadata;

use allocative::Allocative;
use itertools::Itertools;
use rayon::prelude::*;

use crate::{
    states::SplitByAddressCohort,
    structs::{BiMap, Config, Height},
    Date,
};

use self::{all_metadata::AllAddressesMetadataDataset, cohort::CohortDataset};

use super::{AnyDataset, AnyDatasets, ComputeData, InsertData, MinInitialStates};

#[derive(Allocative)]
pub struct AddressDatasets {
    min_initial_states: MinInitialStates,

    metadata: AllAddressesMetadataDataset,

    pub cohorts: SplitByAddressCohort<CohortDataset>,
}

impl AddressDatasets {
    pub fn import(parent_path: &str, config: &Config) -> color_eyre::Result<Self> {
        let mut cohorts = SplitByAddressCohort::<CohortDataset>::default();

        cohorts
            .as_vec()
            .into_par_iter()
            .map(|(_, id)| (id, CohortDataset::import(parent_path, id, config)))
            .collect::<Vec<_>>()
            .into_iter()
            .try_for_each(|(id, dataset)| -> color_eyre::Result<()> {
                *cohorts.get_mut_from_id(&id) = dataset?;
                Ok(())
            })?;

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            metadata: AllAddressesMetadataDataset::import(parent_path, config)?,

            cohorts,
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_datasets(&s, config));

        Ok(s)
    }

    pub fn insert(&mut self, insert_data: &InsertData) {
        self.metadata.insert(insert_data);

        self.cohorts
            .as_mut_vec()
            .into_iter()
            .for_each(|(cohort, _)| cohort.insert(insert_data))
    }

    pub fn needs_durable_states(&self, height: Height, date: Date) -> bool {
        let needs_insert_utxo = self.needs_insert_utxo(height, date);
        let needs_insert_capitalization = self.needs_insert_capitalization(height, date);
        let needs_insert_supply = self.needs_insert_supply(height, date);
        let needs_one_shot_states = self.needs_one_shot_states(height, date);

        needs_insert_utxo
            || needs_insert_capitalization
            || needs_insert_supply
            || needs_one_shot_states
    }

    pub fn needs_one_shot_states(&self, height: Height, date: Date) -> bool {
        self.needs_insert_price_paid(height, date) || self.needs_insert_unrealized(height, date)
    }

    // pub fn needs_sent_states(&self, height: Height, date: WNaiveDate) -> bool {
    //     self.needs_insert_input(height, date) || self.needs_insert_realized(height, date)
    // }

    pub fn needs_insert_utxo(&self, height: Height, date: Date) -> bool {
        self.cohorts
            .as_vec()
            .iter()
            .any(|(dataset, _)| dataset.needs_insert_utxo(height, date))
    }

    pub fn needs_insert_capitalization(&self, height: Height, date: Date) -> bool {
        self.cohorts
            .as_vec()
            .iter()
            .any(|(dataset, _)| dataset.needs_insert_capitalization(height, date))
    }

    pub fn needs_insert_supply(&self, height: Height, date: Date) -> bool {
        self.cohorts
            .as_vec()
            .iter()
            .any(|(dataset, _)| dataset.needs_insert_supply(height, date))
    }

    pub fn needs_insert_price_paid(&self, height: Height, date: Date) -> bool {
        self.cohorts
            .as_vec()
            .iter()
            .any(|(dataset, _)| dataset.needs_insert_price_paid(height, date))
    }

    // pub fn needs_insert_realized(&self, height: Height, date: WNaiveDate) -> bool {
    //     self.cohorts
    //         .as_vec()
    //         .iter()
    //         .any(|(dataset, _)| dataset.needs_insert_realized(height, date))
    // }

    pub fn needs_insert_unrealized(&self, height: Height, date: Date) -> bool {
        self.cohorts
            .as_vec()
            .iter()
            .any(|(dataset, _)| dataset.needs_insert_unrealized(height, date))
    }

    // pub fn needs_insert_input(&self, height: Height, date: WNaiveDate) -> bool {
    //     self.cohorts
    //         .as_vec()
    //         .iter()
    //         .any(|(dataset, _)| dataset.needs_insert_input(height, date))
    // }

    pub fn compute(
        &mut self,
        compute_data: &ComputeData,
        closes: &mut BiMap<f32>,
        circulating_supply: &mut BiMap<f64>,
        market_cap: &mut BiMap<f32>,
    ) {
        self.metadata.compute(compute_data);

        self.cohorts
            .as_mut_vec()
            .into_iter()
            .for_each(|(cohort, _)| {
                cohort.compute(compute_data, closes, circulating_supply, market_cap)
            })
    }
}

impl AnyDatasets for AddressDatasets {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }

    fn to_any_dataset_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)> {
        self.cohorts
            .as_vec()
            .into_iter()
            .map(|(d, _)| d as &(dyn AnyDataset + Send + Sync))
            .chain(vec![&self.metadata as &(dyn AnyDataset + Send + Sync)])
            .collect_vec()
    }

    fn to_mut_any_dataset_vec(&mut self) -> Vec<&mut dyn AnyDataset> {
        self.cohorts
            .as_mut_vec()
            .into_iter()
            .map(|(d, _)| d as &mut dyn AnyDataset)
            .chain(vec![&mut self.metadata as &mut dyn AnyDataset])
            .collect_vec()
    }
}
