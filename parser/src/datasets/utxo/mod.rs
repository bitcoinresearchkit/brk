mod dataset;

use allocative::Allocative;
use dataset::*;
use rayon::prelude::*;

use itertools::Itertools;

use crate::{
    datasets::AnyDatasets,
    states::{SplitByUTXOCohort, UTXOCohortId},
    structs::{BiMap, Config, Date, Height},
};

use super::{AnyDataset, ComputeData, InsertData, MinInitialStates};

#[derive(Allocative)]
pub struct UTXODatasets {
    min_initial_states: MinInitialStates,

    cohorts: SplitByUTXOCohort<UTXODataset>,
}

impl UTXODatasets {
    pub fn import(parent_path: &str, config: &Config) -> color_eyre::Result<Self> {
        let mut cohorts = SplitByUTXOCohort::<Option<UTXODataset>>::default();

        cohorts
            .as_vec()
            .into_par_iter()
            .map(|(_, id)| (id, UTXODataset::import(parent_path, id, config)))
            .collect::<Vec<_>>()
            .into_iter()
            .try_for_each(|(id, dataset)| -> color_eyre::Result<()> {
                cohorts.get_mut(&id).replace(dataset?);
                Ok(())
            })?;

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            cohorts: cohorts.unwrap(),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_datasets(&s, config));

        Ok(s)
    }

    pub fn insert(&mut self, insert_data: &InsertData) {
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

    pub fn needs_sent_states(&self, height: Height, date: Date) -> bool {
        self.needs_insert_input(height, date) || self.needs_insert_realized(height, date)
    }

    pub fn needs_insert_utxo(&self, height: Height, date: Date) -> bool {
        self.as_vec()
            .iter()
            .any(|(dataset, _)| dataset.needs_insert_utxo(height, date))
    }

    pub fn needs_insert_capitalization(&self, height: Height, date: Date) -> bool {
        self.as_vec()
            .iter()
            .any(|(dataset, _)| dataset.needs_insert_capitalization(height, date))
    }

    pub fn needs_insert_supply(&self, height: Height, date: Date) -> bool {
        self.as_vec()
            .iter()
            .any(|(dataset, _)| dataset.needs_insert_supply(height, date))
    }

    pub fn needs_insert_price_paid(&self, height: Height, date: Date) -> bool {
        self.as_vec()
            .iter()
            .any(|(dataset, _)| dataset.needs_insert_price_paid(height, date))
    }

    pub fn needs_insert_realized(&self, height: Height, date: Date) -> bool {
        self.as_vec()
            .iter()
            .any(|(dataset, _)| dataset.needs_insert_realized(height, date))
    }

    pub fn needs_insert_unrealized(&self, height: Height, date: Date) -> bool {
        self.as_vec()
            .iter()
            .any(|(dataset, _)| dataset.needs_insert_unrealized(height, date))
    }

    pub fn needs_insert_input(&self, height: Height, date: Date) -> bool {
        self.as_vec()
            .iter()
            .any(|(dataset, _)| dataset.needs_insert_input(height, date))
    }

    pub fn compute(
        &mut self,
        compute_data: &ComputeData,
        closes: &mut BiMap<f32>,
        circulating_supply: &mut BiMap<f64>,
        market_cap: &mut BiMap<f32>,
    ) {
        self.cohorts
            .as_mut_vec()
            .into_iter()
            .for_each(|(cohort, _)| {
                cohort.compute(compute_data, closes, circulating_supply, market_cap)
            })
    }

    fn as_vec(&self) -> Vec<(&UTXODataset, UTXOCohortId)> {
        self.cohorts.as_vec()
    }

    fn as_mut_vec(&mut self) -> Vec<(&mut UTXODataset, UTXOCohortId)> {
        self.cohorts.as_mut_vec()
    }
}

impl AnyDatasets for UTXODatasets {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }

    fn to_any_dataset_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)> {
        self.as_vec()
            .into_iter()
            .map(|(dataset, _)| dataset as &(dyn AnyDataset + Send + Sync))
            .collect_vec()
    }

    fn to_mut_any_dataset_vec(&mut self) -> Vec<&mut dyn AnyDataset> {
        self.as_mut_vec()
            .into_iter()
            .map(|(dataset, _)| dataset as &mut dyn AnyDataset)
            .collect_vec()
    }
}
